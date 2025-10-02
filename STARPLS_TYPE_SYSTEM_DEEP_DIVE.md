# Starpls Type System and Extension Architecture Deep Dive

## Table of Contents
1. [Overview](#overview)
2. [How Bazel Built-ins Actually Work](#how-bazel-built-ins-actually-work)
3. [The Protobuf Architecture](#the-protobuf-architecture)
4. [Type System Implementation Details](#type-system-implementation-details)
5. [How apple_common.apple_toolchain Works](#how-apple_commonapple_toolchain-works)
6. [Current Extension System Problems](#current-extension-system-problems)
7. [How Tilt and tilt-starlark-lsp Handle Extensions](#how-tilt-and-tilt-starlark-lsp-handle-extensions)
8. [Proposed Solutions](#proposed-solutions)
9. [Implementation Roadmap](#implementation-roadmap)

## Overview

This document captures our deep investigation into how starpls handles types, built-ins, and extensions. The key insight is that Bazel's approach using protobuf definitions is fundamentally different from our current JSON-based extension system, and this difference causes significant issues with type inference and completions.

## How Bazel Built-ins Actually Work

### The Data Flow

1. **Source of Truth**: Bazel generates a protobuf file (`builtin.pb`) containing all built-in symbols
2. **Loading**: Located at `/crates/starpls/src/builtin/builtin.pb` and loaded via:
   ```rust
   // In server.rs
   pub(crate) fn load_bazel_builtins() -> Builtins {
       let data = include_bytes!("builtin/builtin.pb");
       decode_builtins(&data[..]).expect("bug: invalid builtin.pb")
   }
   ```

3. **Protobuf Schema** (from `builtin.proto`):
   ```protobuf
   message Builtins {
     repeated Type type = 1;     // Types like "apple_common", "ctx", etc.
     repeated Value global = 2;   // Global symbols available in files
   }

   message Type {
     string name = 1;
     repeated Value field = 2;    // Fields and methods of this type
     string doc = 3;
   }

   message Value {
     string name = 1;
     string type = 2;            // Type name as string
     Callable callable = 3;       // If it's a function
     string doc = 4;
     ApiContext api_context = 5;
   }

   message Callable {
     repeated Param param = 1;
     string return_type = 2;
   }
   ```

### Type System Processing

The processing happens in `/crates/starpls_hir/src/typeck/builtins.rs`:

1. **Type Registration** (`builtin_types_query` function, lines 714-845):
   ```rust
   for type_ in builtins.r#type.iter() {
       // Skip deny-listed types handled by intrinsics
       if BUILTINS_TYPES_DENY_LIST.contains(&type_.name.as_str()) {
           continue;
       }

       // Collect fields and methods
       let mut fields = Vec::new();
       let mut methods = Vec::new();

       for field in type_.field.iter() {
           if let Some(callable) = &field.callable {
               // It's a method
               methods.push(builtin_function(...));
           } else {
               // It's a field
               fields.push(BuiltinField { ... });
           }
       }

       // Create BuiltinType
       types.insert(
           type_.name.clone(),
           TyKind::BuiltinType(
               BuiltinType::new(db, name, fields, methods, doc, indexable_by),
               None
           ).intern()
       );
   }
   ```

2. **Global Registration** (`builtin_globals_query`, lines 668-706):
   ```rust
   let bzl_globals = APIGlobals::from_values(
       db,
       providers,
       builtins.global.iter()
           .chain(rules.global.iter()),
   );
   ```

3. **Global Processing** (`APIGlobals::from_values`, lines 105-141):
   ```rust
   for value in values {
       match (providers.get(value.name.as_str()), &value.callable) {
           (Some(provider), _) => {
               // It's a provider
               variables.insert(value.name.clone(), TypeRef::Provider(*provider));
           }
           (None, Some(callable)) => {
               // It's a function
               functions.insert(value.name.clone(), builtin_function(...));
           }
           (None, None) => {
               // It's a variable - THIS IS KEY!
               variables.insert(value.name.clone(), parse_type_ref(&value.r#type));
           }
       }
   }
   ```

## The Protobuf Architecture

### Where Protobuf Comes From

The `builtin.pb` file is a binary protobuf file included in the starpls binary. It's generated from Bazel's built-in definitions and contains:

1. **Types** (`apple_common`, `ctx`, `native`, etc.)
2. **Global values** (functions and variables available globally)
3. **Documentation** for everything

### How It's Integrated

The integration happens via conditional compilation:

```rust
// In /crates/starpls_bazel/src/lib.rs
#[cfg(bazel)]
pub mod builtin {
    pub use builtin_proto::builtin::*;
}

#[cfg(not(bazel))]
pub mod builtin {
    include!(concat!(env!("OUT_DIR"), "/builtin.rs"));
}
```

This means the protobuf definitions are compiled into Rust types that can be used directly.

## Type System Implementation Details

### The Type Hierarchy

1. **TypeRef** (`/crates/starpls_hir/src/typeck.rs`, lines 175-202):
   ```rust
   pub enum TypeRef {
       Name(Name, Option<Box<[TypeRef]>>),  // Named type with optional args
       Union(Vec<TypeRef>),                 // Union type
       Tuple(Vec<TypeRef>),                 // Tuple type
       Function(Box<FunctionTypeRef>),      // Function type
       Provider(BuiltinProvider),           // Provider type
       Ellipsis,                            // Variable-length indicator
       Unknown,                             // Unknown type
   }
   ```

2. **TyKind** (internal representation after resolution):
   ```rust
   enum TyKind {
       BuiltinType(BuiltinType, Option<TyData>),
       Struct(Option<Struct>),
       Function(Function),
       BuiltinFunction(BuiltinFunction),
       IntrinsicFunction(IntrinsicClass, Vec<Ty>),
       // ... many more
   }
   ```

3. **BuiltinType** (`/crates/starpls_hir/src/typeck/builtins.rs`, lines 64-73):
   ```rust
   #[salsa::tracked]
   pub(crate) struct BuiltinType {
       pub(crate) name: Name,
       pub(crate) fields: Vec<BuiltinField>,    // Properties
       pub(crate) methods: Vec<BuiltinFunction>, // Methods
       pub(crate) doc: String,
       pub(crate) indexable_by: Option<(TypeRef, TypeRef)>,
   }
   ```

### Field Resolution

When you write `apple_common.apple_toolchain`, the resolution happens in `Ty::fields()` (`/crates/starpls_hir/src/typeck.rs`, lines 260-370):

```rust
pub(crate) fn fields<'a>(&'a self, db: &'a dyn Db)
    -> Option<impl Iterator<Item = (Field, Ty)> + 'a> {
    let kind = self.kind();

    let fields = match kind {
        TyKind::BuiltinType(ty, data) => Fields::Builtin(
            ty.fields(db).iter()
                .map(|(index, field)| {
                    let resolved = resolve_builtin_type_ref(db, &field.type_ref);
                    (Field(FieldInner::BuiltinField { parent: *ty, index }), resolved)
                })
                .chain(ty.methods(db).iter().map(|func| {
                    (Field(FieldInner::BuiltinMethod { func: *func }),
                     TyKind::BuiltinFunction(*func).intern())
                })),
        ),
        TyKind::Struct(strukt) => Fields::Struct(...),
        // ... other cases
    };
}
```

## How apple_common.apple_toolchain Works

This is the key insight that explains everything:

### 1. The Protobuf Definition

In `builtin.pb`, there are TWO entries for `apple_common`:

```protobuf
// Entry 1: A Type definition
Type {
  name: "apple_common"
  field: [
    Value {
      name: "apple_toolchain"
      type: "unknown"  // or some other type
      doc: "Utilities for resolving items from the apple toolchain"
    },
    // ... other fields like XcodeProperties, platform, etc.
  ]
}

// Entry 2: A Global Value
Value {
  name: "apple_common"
  type: "apple_common"  // SELF-REFERENTIAL!
  // No callable - it's a variable, not a function
}
```

### 2. The Self-Referential Pattern

The magic is that `apple_common` is:
1. A **Type** (defines what fields/methods it has)
2. A **Global Variable** with type pointing to itself

This means:
- When you write `apple_common`, it resolves to a global variable
- That variable has type `TypeRef::Name("apple_common")`
- That TypeRef points to the `BuiltinType` for "apple_common"
- When you write `apple_common.apple_toolchain`, it looks up "apple_toolchain" in the BuiltinType's fields

### 3. Why This Works in .bzl Files

The global is registered in `bzl_globals` (line 673-682 in builtin_globals_query):
```rust
let bzl_globals = APIGlobals::from_values(
    db,
    providers,
    env::make_bzl_builtins().global.iter()
        .chain(builtins.global.iter())  // <-- apple_common global is here
        .chain(rules.global.iter()),
);
```

This makes `apple_common` available as a global in .bzl files.

## Current Extension System Problems

### What We're Doing Wrong

1. **Virtual File Generation** (`/crates/starpls/src/document.rs`, lines 395-440):
   ```rust
   fn create_virtual_file_content(symbols: &[Symbol]) -> String {
       for symbol in symbols {
           if !symbol.properties.is_empty() {
               // Generate: exec = struct(sh = lambda command: None, ...)
               content.push_str(&format!("{} = struct(\n", symbol.name));
               for (prop_name, prop_symbol) in &symbol.properties {
                   // We FIXED this to use proper signatures
                   if prop_symbol.r#type == "function" && prop_symbol.callable.is_some() {
                       let params = /* extract params */;
                       content.push_str(&format!("    {} = lambda {}: None,\n",
                           prop_name, params.join(", ")));
                   }
               }
           }
       }
   }
   ```

2. **Why struct() Doesn't Work**:
   - `struct()` creates a `TyKind::Struct` at runtime
   - It doesn't create a `BuiltinType` with proper field definitions
   - The type inference sees it as a generic struct, not a typed object
   - Result: "(variable) exec: Unknown" in hover

3. **Global Symbol Injection Issues** (`/crates/starpls/src/server.rs`, lines 408-453):
   ```rust
   fn inject_extension_globals(builtins: &mut Builtins, extensions: &Extensions) {
       for symbol in extensions.global_symbols() {
           // This only works for extensions without 'when' clauses!
           let value = /* convert to protobuf Value */;
           builtins.global.push(value);
       }
   }
   ```

   Problems:
   - Only works for global extensions (no `when` clause)
   - Doesn't create proper BuiltinTypes
   - No type system integration

### Comparison with Built-ins

| Aspect | Bazel Built-ins | Our Extensions |
|--------|----------------|----------------|
| Definition Format | Protobuf | JSON |
| Type Creation | BuiltinType instances | struct() in virtual files |
| Global Registration | Self-referential types | Basic Value objects |
| Field Access | Proper type resolution | Generic struct fields |
| Documentation | Full docs on types/fields | Limited lambda docs |
| Parameter Info | Complete Callable info | Basic lambda signatures |

## How Tilt and tilt-starlark-lsp Handle Extensions

### Tilt's Approach

1. **Uses Python files** for API definitions (`/internal/tiltfile/api/__init__.py`):
   ```python
   class PortForward:
       """Specifications for setting up port-forward."""
       pass

   def port_forward(local_port: int, container_port: Optional[int] = None) -> PortForward:
       """Creates a PortForward object..."""
       pass
   ```

2. **Code Generation** via `tilt-starlark-codegen`:
   - Reads Kubernetes-style API models
   - Generates Starlark functions
   - Makefile target: `update-codegen-starlark`

3. **Go Implementation**: Actually implements the functions in Go

### tilt-starlark-lsp's Approach

1. **Python Parsing** (`/pkg/analysis/builtins.go`):
   ```go
   func LoadBuiltinsFromSource(ctx context.Context, contents []byte, path string) (*Builtins, error) {
       tree, err := query.Parse(ctx, contents)  // Parse Python with tree-sitter
       doc := document.NewDocument(uri.File(path), contents, tree)

       functions := doc.Functions()  // Extract functions
       symbols := doc.Symbols()      // Extract symbols
       types := query.Types(doc, tree.RootNode())  // Extract types

       // Build type map, method map, etc.
   }
   ```

2. **Direct Integration**: No virtual files, just parses Python and uses the AST directly

3. **Benefits**:
   - Natural Python syntax for definitions
   - Full type annotations support
   - Rich documentation from docstrings
   - No virtual file generation needed

## Proposed Solutions

### Solution 1: Enhanced JSON with Type System Integration (Quick Fix)

Modify the extension loading to create proper BuiltinTypes:

```rust
// Instead of generating virtual files, do this:
fn load_extension_types(extension: &Extension, types: &mut HashMap<String, Ty>) {
    for module in &extension.modules {
        for symbol in module.symbols {
            if symbol.r#type == "object" && !symbol.properties.is_empty() {
                // Create a BuiltinType
                let fields = /* convert properties to BuiltinFields */;
                let methods = /* convert callable properties to BuiltinFunctions */;

                let builtin_type = BuiltinType::new(
                    db,
                    Name::from_str(&symbol.name),
                    fields,
                    methods,
                    symbol.doc,
                    None
                );

                // Register as type
                types.insert(
                    symbol.name.clone(),
                    TyKind::BuiltinType(builtin_type, None).intern()
                );

                // Also register as global variable (self-referential)
                globals.variables.insert(
                    symbol.name.clone(),
                    TypeRef::Name(Name::from_str(&symbol.name), None)
                );
            }
        }
    }
}
```

### Solution 2: Support Protobuf Extensions (Better)

Allow extensions to be defined in protobuf format:

```protobuf
// exec_extension.pb
message Extension {
  repeated Type type = 1;
  repeated Value global = 2;
}

Type {
  name: "exec"
  field: [
    Value {
      name: "sh"
      type: "function"
      callable: {
        param: [{
          name: "command"
          type: "string"
          is_mandatory: true
        }]
        return_type: "string"
      }
      doc: "Execute a shell command"
    },
    // ... other methods
  ]
}

Value {
  name: "exec"
  type: "exec"  // Self-referential
}
```

### Solution 3: Python-Based Extensions (Most Flexible)

Follow tilt-starlark-lsp's approach:

```python
# exec_extension.py
class exec:
    """Command execution utilities."""

    @staticmethod
    def sh(command: str) -> str:
        """Execute a shell command and return output."""
        pass

    @staticmethod
    def echo(message: str) -> None:
        """Print a message to stdout."""
        pass
```

Then parse with tree-sitter-python and create BuiltinTypes directly.

## Implementation Roadmap

### Phase 1: Fix Current System (1-2 days)
1. ✅ Fix lambda signatures in virtual content (DONE)
2. ❌ Create BuiltinTypes for object-type symbols
3. ❌ Register objects as self-referential globals
4. ❌ Fix global symbol injection for file-specific extensions

### Phase 2: Protobuf Support (3-4 days)
1. ❌ Add protobuf extension loading alongside JSON
2. ❌ Create protobuf schema for extensions
3. ❌ Update documentation and examples
4. ❌ Add tests for protobuf extensions

### Phase 3: Python Support (1 week)
1. ❌ Integrate tree-sitter-python
2. ❌ Create Python parser for extensions
3. ❌ Map Python types to starpls types
4. ❌ Support type annotations and docstrings

### Phase 4: Cleanup (2-3 days)
1. ❌ Remove virtual file generation for object types
2. ❌ Deprecate JSON format (with migration path)
3. ❌ Update all documentation
4. ❌ Add comprehensive tests

## Key Code Locations

### Core Type System
- `/crates/starpls_hir/src/typeck.rs` - Main type definitions
- `/crates/starpls_hir/src/typeck/builtins.rs` - Builtin type processing
- `/crates/starpls_hir/src/typeck/intrinsics.rs` - Intrinsic types (dict, list, etc.)

### Protobuf Handling
- `/crates/starpls_bazel/data/builtin.proto` - Protobuf schema
- `/crates/starpls_bazel/src/lib.rs` - Protobuf loading
- `/crates/starpls/src/builtin/builtin.pb` - Binary protobuf data
- `/crates/starpls/src/server.rs` - Loading and injection

### Extension System
- `/crates/starpls_common/src/extensions.rs` - Extension definitions
- `/crates/starpls/src/document.rs` - Virtual file generation
- `/crates/starpls/src/server.rs` - Extension loading and injection

### Completion and Hover
- `/crates/starpls_ide/src/completions.rs` - Completion logic
- `/crates/starpls_hir/src/typeck/infer.rs` - Type inference

## Lessons Learned

1. **Virtual files are a hack**: They work for simple cases but fail for complex types
2. **Self-referential types are key**: The pattern of having a type and a global with the same name
3. **Protobuf is powerful**: It provides a complete type system description
4. **Python is natural**: For Starlark extensions, Python syntax is ideal
5. **Type system integration is crucial**: Extensions must create proper types, not just code

## Next Steps

1. **Decide on approach**: Protobuf vs Python vs enhanced JSON
2. **Create proof of concept**: Implement for `exec` extension
3. **Test thoroughly**: Ensure completion, hover, and type inference work
4. **Document for users**: Create extension authoring guide
5. **Migrate existing extensions**: Convert from JSON to new format

## Conclusion

The current extension system's fundamental flaw is generating Starlark code instead of integrating with the type system. By following Bazel's protobuf model or tilt-starlark-lsp's Python parsing approach, we can provide proper type inference, completion, and documentation for extensions.

The key insight is the self-referential pattern: objects like `apple_common` work because they're both a type AND a global variable pointing to that type. Our extensions need to follow this same pattern to work properly.