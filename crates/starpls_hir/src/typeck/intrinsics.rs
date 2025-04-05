use std::sync::Arc;

use rustc_hash::FxHashMap;
use smallvec::smallvec;

use crate::def::Argument;
use crate::def::InternedString;
use crate::typeck::Binders;
use crate::typeck::DictLiteral;
use crate::typeck::Substitution;
use crate::typeck::Tuple as TupleVariants;
use crate::typeck::Ty;
use crate::typeck::TyKind;
use crate::typeck::{self};
use crate::Db;
use crate::Name;

#[salsa::tracked]
pub(crate) struct Intrinsics {
    #[return_ref]
    pub(crate) types: IntrinsicTypes,

    // Base classes for types with fields/methods.
    pub(crate) string_base_class: IntrinsicClass,
    pub(crate) bytes_base_class: IntrinsicClass,
    pub(crate) list_base_class: IntrinsicClass,
    pub(crate) dict_base_class: IntrinsicClass,
}

/// This serves to pre-intern common types.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct IntrinsicTypes {
    pub(crate) any: Ty,
    pub(crate) unbound: Ty,
    pub(crate) unknown: Ty,
    pub(crate) none: Ty,
    pub(crate) bool: Ty,
    pub(crate) int: Ty,
    pub(crate) float: Ty,
    pub(crate) string: Ty,
    pub(crate) string_elems: Ty,
    pub(crate) bytes: Ty,
    pub(crate) bytes_elems: Ty,
    pub(crate) range: Ty,
}

impl Default for IntrinsicTypes {
    fn default() -> Self {
        Self {
            any: Ty::any(),
            unbound: TyKind::Unbound.intern(),
            unknown: Ty::unknown(),
            none: TyKind::None.intern(),
            bool: Ty::bool(),
            int: Ty::int(),
            float: TyKind::Float.intern(),
            string: Ty::string(),
            string_elems: TyKind::StringElems.intern(),
            bytes: TyKind::Bytes.intern(),
            bytes_elems: TyKind::BytesElems.intern(),
            range: TyKind::Range.intern(),
        }
    }
}

#[salsa::tracked]
pub(crate) struct IntrinsicClass {
    pub(crate) name: Name,
    pub(crate) num_vars: usize,
    #[return_ref]
    pub(crate) fields: Vec<IntrinsicField>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IntrinsicField {
    pub(crate) name: Name,
    pub(crate) doc: String,
    ty: Ty,
}

impl IntrinsicField {
    fn new_inline(name: &'static str, doc: &'static str, ty: Ty) -> Self {
        Self {
            name: Name::new_inline(name),
            doc: doc.to_string(),
            ty,
        }
    }
}

#[salsa::tracked]
pub(crate) struct IntrinsicFieldTypes {
    #[return_ref]
    pub(crate) field_tys: Vec<Binders>,
}

#[salsa::tracked]
pub(crate) fn intrinsic_field_types(db: &dyn Db, class: IntrinsicClass) -> IntrinsicFieldTypes {
    let field_tys = class
        .fields(db)
        .iter()
        .map(|field| Binders::new(class.num_vars(db), field.ty.clone()))
        .collect();
    IntrinsicFieldTypes::new(db, field_tys)
}

#[salsa::tracked]
pub(crate) struct IntrinsicFunctions {
    #[return_ref]
    pub functions: FxHashMap<Name, IntrinsicFunction>,
}

#[salsa::tracked]
pub(crate) struct IntrinsicFunction {
    pub name: Name,
    #[return_ref]
    pub doc: String,
    pub num_vars: usize,
    #[return_ref]
    pub params: Vec<IntrinsicFunctionParam>,
    pub ret_ty: Ty,
    is_dict_constructor: bool,
}

impl IntrinsicFunction {
    pub(crate) fn maybe_unique_ret_type<'a, I>(&'a self, db: &'a dyn Db, args: I) -> Option<Ty>
    where
        I: Iterator<Item = (&'a Argument, &'a Ty)>,
    {
        if !self.is_dict_constructor(db) {
            return None;
        }

        let known_keys = args
            .filter_map(|(arg, ty)| match arg {
                Argument::Keyword { name, .. } => Some((
                    InternedString::new(db, name.as_str().to_string().into_boxed_str()),
                    ty.clone(),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();

        let key_ty = if known_keys.is_empty() {
            Ty::unknown()
        } else {
            Ty::string()
        };

        // Determine the common type of the dict values.
        let mut iter = known_keys.iter().map(|(_, ty)| ty);
        let value_ty = iter
            .next()
            .and_then(|first_ty| {
                iter.all(|ty| match (ty.kind(), first_ty.kind()) {
                    (TyKind::Attribute(_), TyKind::Attribute(_)) => true,
                    _ => ty == first_ty,
                })
                .then_some(first_ty)
            })
            .cloned()
            .unwrap_or_else(Ty::unknown);

        Some(
            TyKind::Dict(
                key_ty,
                value_ty,
                Some(Arc::new(DictLiteral {
                    expr: None,
                    known_keys: known_keys.into_boxed_slice(),
                })),
            )
            .intern(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum IntrinsicFunctionParam {
    Positional { ty: Ty, optional: bool },
    Keyword { name: Name, ty: Ty },
    ArgsList { ty: Ty },
    KwargsDict,
}

impl IntrinsicFunctionParam {
    pub(crate) fn is_optional(&self) -> bool {
        match self {
            IntrinsicFunctionParam::Positional { optional, .. } => *optional,
            _ => true,
        }
    }

    pub(crate) fn name(&self) -> Option<&Name> {
        match self {
            IntrinsicFunctionParam::Keyword { name, .. } => Some(name),
            _ => None,
        }
    }

    pub(crate) fn ty(&self) -> Option<Ty> {
        Some(match self {
            IntrinsicFunctionParam::Positional { ty, .. }
            | IntrinsicFunctionParam::Keyword { ty, .. }
            | IntrinsicFunctionParam::ArgsList { ty } => ty.clone(),
            IntrinsicFunctionParam::KwargsDict => return None,
        })
    }
}

#[salsa::tracked]
pub(crate) fn intrinsic_functions(db: &dyn Db) -> IntrinsicFunctions {
    // TODO(withered-magic): Many of these signatures are wrong
    // since the implementation of Starlark's type system is still
    // heavily WIP. For example, for the `list` intrinsic, we need to
    // support the `(List[T]) -> T` signature.
    // We also still need to support features like optional arguments,
    // keyword-only parameters, union types, "traits" like `Sequence[T]`,
    // function overloads, and so on.

    use IntrinsicFunctionParam::*;
    use TyKind::*;
    let mut functions = FxHashMap::default();
    let mut add_function = |name, doc, params, ret_ty| {
        functions.insert(
            Name::new_inline(name),
            function(db, name, doc, params, 0, ret_ty),
        );
    };

    // TODO(withered-magic): SupportsAbs[T] -> T
    add_function("abs", "`abs(x)` takes either an integer or a float, and returns the absolute value of that number (a non-negative number with the same magnitude).", vec![positional(Any)], Any);
    add_function("any", "`any(x)` returns `True` if any element of the iterable sequence x is true. If the iterable is empty, it returns `False`.", vec![positional(Any)], non_literal_bool());
    add_function("all", "`all(x)` returns `False` if any element of the iterable sequence x is false. If the iterable is empty, it returns `True`.", vec![positional(Any)], non_literal_bool());
    add_function("bool", "`bool(x)` interprets `x` as a Boolean value---`True` or `False`. With no argument, `bool()` returns `False`.", vec![positional_opt(Any)], non_literal_bool());
    // TODO(withered-magic): SupportsBytes[T] -> T
    add_function(
        "bytes",
        r#"`bytes(x)` converts its argument to a `bytes`.

If x is a `bytes`, the result is `x`.
    
If x is a string, the result is a `bytes` whose elements are the UTF-8 encoding of the string. Each element of the string that is not part of a valid encoding of a code point is replaced by the UTF-8 encoding of the replacement character, U+FFFD.
    
If x is an iterable sequence of int values, the result is a `bytes` whose elements are those integers. It is an error if any element is not in the range 0-255.

```python
bytes("hello 😃")		# b"hello 😃"
bytes(b"hello 😃")		# b"hello 😃"
bytes("hello 😃"[:-1])          # b"hello ���"
bytes([65, 66, 67])		# b"ABC"
bytes(65)			# error: got int, want string, bytes, or iterable of int
```
"#,
        vec![positional(Any)],
        Bytes,
    );
    add_function(
        "dict",
        r#"`dict` creates a dictionary.  It accepts up to one positional
argument, which is interpreted as an iterable of two-element
sequences (pairs), each specifying a key/value pair in
the resulting dictionary.
        
`dict` also accepts any number of keyword arguments, each of which
specifies a key/value pair in the resulting dictionary;
each keyword is treated as a string.

```python
dict()                          # {}, empty dictionary
dict([(1, 2), (3, 4)])          # {1: 2, 3: 4}
dict([(1, 2), ["a", "b"]])      # {1: 2, "a": "b"}
dict(one=1, two=2)              # {"one": 1, "two", 1}
dict([(1, 2)], x=3)             # {1: 2, "x": 3}
```
        
With no arguments, `dict()` returns a new empty dictionary.
        
`dict(x)` where x is a dictionary returns a new copy of x."#,
        vec![
            positional_opt(TyKind::Union(smallvec![
                Ty::dict(Ty::unknown(), Ty::unknown(), Option::None),
                Protocol(typeck::Protocol::Iterable(
                    Protocol(typeck::Protocol::Iterable(Any.intern())).intern(),
                ))
                .intern(),
            ])),
            KwargsDict,
        ],
        Dict(Unknown.intern(), Unknown.intern(), Option::None),
    );

    add_function(
        "dir",
        r#"`dir(x)` returns a new sorted list of the names of the attributes (fields and methods) of its operand.
The attributes of a value `x` are the names `f` such that `x.f` is a valid expression.
        
For example,
        
```python
dir("hello")                    # ['capitalize', 'count', ...], the methods of a string
```
        
Several types known to the interpreter, such as list, string, and dict, have methods, but none have fields.
However, an application may define types with fields that may be read or set by statements such as these:
        
```text
y = x.f
x.f = y
```
"#,
        vec![positional(Any)],
        List(Ty::string()),
    );
    add_function(
        "enumerate",
        r#"`enumerate(x)` returns a list of (index, value) pairs, each containing
successive values of the iterable sequence xand the index of the value
within the sequence.
        
The optional second parameter, `start`, specifies an integer value to
add to each index.
        
```python
enumerate(["zero", "one", "two"])               # [(0, "zero"), (1, "one"), (2, "two")]
enumerate(["one", "two"], 1)                    # [(1, "one"), (2, "two")]
```
"#,
        vec![positional(Any)],
        List(Tuple(TupleVariants::Simple(smallvec![Ty::int(), Any.intern()])).intern()),
    );
    add_function(
        "float",
        r#"`float(x)` interprets its argument as a floating-point number.

If x is a `float`, the result is x.
        
If x is an `int`, the result is the floating-point value nearest x.
The call fails if x is too large to represent as a finite `float`.
        
If x is a `bool`, the result is `1.0` for `True` and `0.0` for `False`.
        
If x is a string, the string is interpreted as a floating-point literal.
The function also recognizes the names `Inf` (or `Infinity`) and `NaN`,
optionally preceded by a `+` or `-` sign.
These construct the IEEE 754 non-finite values.
Letter case is not significant.
The call fails if the literal denotes a value too large to represent as
a finite `float`.
        
With no argument, `float()` returns `0.0`.
"#,
        vec![positional(Any)],
        Float,
    );
    add_function(
        "fail",
        r#"The `fail(*args)` function causes execution to fail
with an error message that includes the string forms of the argument values.
The precise formatting depends on the implementation.
        
```python
fail("oops")			# "fail: oops"
fail("oops", 1, False)		# "fail: oops 1 False"
```
"#,
        vec![
            Keyword {
                name: Name::new_inline("msg"),
                ty: Ty::string(),
                deprecated: true,
            },
            Keyword {
                name: Name::new_inline("attr"),
                ty: Ty::string(),
                deprecated: true,
            },
            Keyword {
                name: Name::new_inline("sep"),
                ty: TyKind::String(Some(InternedString::new(db, " ".to_string().into_boxed_str()))).intern(),
                deprecated: false,
            },
            ArgsList { ty: Any.intern() },
        ],
        Never,
    );
    add_function(
        "getattr",
        r#"`getattr(x, name[, default])` returns the value of the attribute (field or method) of x named `name`
if it exists. If not, it either returns `default` (if specified) or raises an error.
        
`getattr(x, "f")` is equivalent to `x.f`.
        
```python
getattr("banana", "split")("a")	       		# ["b", "n", "n", ""], equivalent to "banana".split("a")
getattr("banana", "myattr", "mydefault")	# "mydefault"
```
        
The three-argument form `getattr(x, name, default)` returns the
provided `default` value instead of failing.
"#,
        vec![
            positional(Any),
            positional(non_literal_string()),
            positional_opt(Any),
        ],
        Any,
    );
    add_function(
        "hasattr",
        r#"`hasattr(x, name)` reports whether x has an attribute (field or method) named `name`."#,
        vec![positional(Any), positional(non_literal_string())],
        non_literal_bool(),
    );
    // // TODO(withered-magic): SupportsHash[T] -> T
    add_function(
        "hash",
        r#"`hash(x)` returns an integer hash of a string or bytes x
such that two equal values have the same hash.
In other words `x == y` implies `hash(x) == hash(y)`.
Any other type of argument in an error, even if it is suitable as the key of a dict.
        
In the interests of reproducibility of Starlark program behavior over time and
across implementations, the specific hash function for bytes is 32-bit FNV-1a,
and the hash function for strings is the same as that implemented by
[java.lang.String.hashCode](https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#hashCode),
a simple polynomial accumulator over the UTF-16 transcoding of the string:
        
```python
s[0]*31^(n-1) + s[1]*31^(n-2) + ... + s[n-1]
```
"#,
        vec![positional(Any)],
        non_literal_int(),
    );
    // // TODO(withered-magic): SupportInt[T] -> T
    add_function(
        "int",
        r#"`int(x[, base])` interprets its argument as an integer.

If `x` is an `int`, the result is `x`.
        
If x is a `float`, the result is the integer value nearest to x,
truncating towards zero. It is an error if x is not finite (`NaN`
or infinity).
        
If x is a `bool`, the result is 0 for `False` or 1 for `True`.
        
If x is a string, it is interpreted as a sequence of digits in the
specified base, decimal by default.
        
If `base` is zero, x is interpreted like an integer literal,
the base being inferred from an optional base prefix such as
`0b`, `0o`, or `0x` preceding the first digit.

When a nonzero `base` is provided explicitly,
its value must be between 2 and 36.
The letters `a-z` represent the digits 11 through 35.
A matching base prefix is also permitted, and has no effect.

Irrespective of base, the string may start with an optional `+` or `-`,
indicating the sign of the result.

```python
int("21")          # 21
int("1234", 16)    # 4660
int("0x1234", 16)  # 4660
int("0x1234", 0)   # 4660
int("0b0", 16)     # 176
int("0b111", 0)    # 7
int("0x1234")      # error (invalid base 10 number)
```
"#,
        vec![positional(Any), positional_opt(non_literal_int())],
        non_literal_int(),
    );
    add_function(
        "len",
        r#"`len(x)` returns the number of elements in its argument.

It is a dynamic error if its argument is not a sequence."#,
        vec![positional(Any)],
        non_literal_int(),
    );
    add_function(
        "list",
        r#"`list` constructs a list.

`list(x)` returns a new list containing the elements of the iterable sequence x.
    
With no argument, `list()` returns a new empty list."#,
        vec![positional_opt(Any)],
        List(Any.intern()),
    );
    add_function(
        "max",
        r#"`max(x)` returns the greatest element in the iterable sequence x.

It is an error if any element does not support ordered comparison,
or if the sequence is empty.
        
The optional named parameter `key` specifies a function to be applied
to each element prior to comparison.
        
```python
max([3, 1, 4, 1, 5, 9])                         # 9
max("two", "three", "four")                     # "two", the lexicographically greatest
max("two", "three", "four", key=len)            # "three", the longest
```
"#,
        vec![
            ArgsList { ty: Any.intern() },
            Keyword {
                name: Name::new_inline("key"),
                ty: Any.intern(),
            },
        ],
        Any,
    );
    add_function(
        "min",
        r#"`min(x)` returns the least element in the iterable sequence x.

It is an error if any element does not support ordered comparison,
or if the sequence is empty.
        
The optional named parameter `key` specifies a function to be applied
to each element prior to comparison.
        
```python
min([3, 1, 4, 1, 5, 9])                         # 1
min("two", "three", "four")                     # "four", the lexicographically least
min("two", "three", "four", key=len)            # "two", the shortest
```
"#,
        vec![
            ArgsList { ty: Any.intern() },
            Keyword {
                name: Name::new_inline("key"),
                ty: Any.intern(),
            },
        ],
        Any,
    );
    add_function(
        "print",
        r#"`print(*args, sep=" ")` prints its arguments, followed by a newline.
Arguments are formatted as if by `str(x)` and separated with a space,
unless an alternative separator is specified by a `sep` named argument.

Example:

```python
print(1, "hi", x=3)                             # "1 hi x=3\n"
print("hello", "world")                         # "hello world\n"
print("hello", "world", sep=", ")               # "hello, world\n"
```

Typically the formatted string is printed to the standard error file,
but the exact behavior is a property of the Starlark thread and is
determined by the host application.
"#,
        vec![
            ArgsList { ty: Any.intern() },
            Keyword {
                name: Name::new_inline("str"),
                ty: Ty::string(),
            },
        ],
        None,
    );
    add_function(
        "range",
        r#"`range` returns an immutable sequence of integers defined by the specified interval and stride.

```python
range(stop)                             # equivalent to range(0, stop)
range(start, stop)                      # equivalent to range(start, stop, 1)
range(start, stop, step)
```

`range` requires between one and three integer arguments.
With one argument, `range(stop)` returns the ascending sequence of non-negative integers less than `stop`.
With two arguments, `range(start, stop)` returns only integers not less than `start`.

With three arguments, `range(start, stop, step)` returns integers
formed by successively adding `step` to `start` until the value meets or passes `stop`.
A call to `range` fails if the value of `step` is zero.

A call to `range` does not materialize the entire sequence, but
returns a fixed-size value of type `"range"` that represents the
parameters that define the sequence.
The `range` value is iterable and may be indexed efficiently.

```python
list(range(10))                         # [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
list(range(3, 10))                      # [3, 4, 5, 6, 7, 8, 9]
list(range(3, 10, 2))                   # [3, 5, 7, 9]
list(range(10, 3, -2))                  # [10, 8, 6, 4]
```

The `len` function applied to a `range` value returns its length.
The truth value of a `range` value is `True` if its length is non-zero.

Range values are comparable: two `range` values compare equal if they
denote the same sequence of integers, even if they were created using
different parameters.

Range values are not hashable.  <!-- should they be? -->

The `str` function applied to a `range` value yields a string of the
form `range(10)`, `range(1, 10)`, or `range(1, 10, 2)`.

The `x in y` operator, where `y` is a range, reports whether `x` is equal to
some member of the sequence `y`; the operation fails unless `x` is a
number.
"#,
        vec![
            positional(non_literal_int()),
            positional_opt(non_literal_int()),
            positional_opt(non_literal_int()),
        ],
        Range,
    );
    add_function(
        "repr",
        r#"`repr(x)` formats its argument as a string.

All strings in the result are double-quoted.

```python
repr(1)                 # '1'
repr("x")               # '"x"'
repr([1, "x"])          # '[1, "x"]'
```

When applied to a string containing valid text,
`repr` returns a string literal that denotes that string.
When applied to a string containing an invalid UTF-K sequence,
`repr` uses `\x` and `\u` escapes with out-of-range values to indicate
the invalid elements; the result is not a valid literal.

```python
repr("🙂"[:1])		# "\xf0" (UTF-8) or "\ud83d" (UTF-16)
"\xf0"                  # error: non-ASCII hex escape
"\ud83d"                # error: invalid Unicode code point U+D83D
```"#,
        vec![positional(Any)],
        non_literal_string(),
    );
    // TODO(withered-magic): Iterable[T] -> List[T]
    add_function(
        "reversed",
        r#"`reversed(x)` returns a new list containing the elements of the iterable sequence x in reverse order.

```python
reversed(range(5))                              # [4, 3, 2, 1, 0]
reversed({"one": 1, "two": 2}.keys())           # ["two", "one"]
```
"#,
        vec![positional(Any)],
        List(Any.intern()),
    );
    // TODO(withered-magic): Iterable[T] -> List[T]
    add_function(
        "sorted",
        r#"`sorted(x)` returns a new list containing the elements of the iterable sequence x,
in sorted order.  The sort algorithm is stable.

The optional named boolean parameter `reverse`, if true, causes `sorted` to
return results in reverse sorted order.

The optional named parameter `key` specifies a function of one
argument to apply to obtain the value's sort key.
The default behavior is the identity function.
The `key` function is called exactly once per element of the sequence, in order,
even for a single-element list.

```python
sorted([3, 1, 4, 1, 5, 9])                                 # [1, 1, 3, 4, 5, 9]
sorted([3, 1, 4, 1, 5, 9], reverse=True)                   # [9, 5, 4, 3, 1, 1]

sorted(["two", "three", "four"], key=len)                  # ["two", "four", "three"], shortest to longest
sorted(["two", "three", "four"], key=len, reverse=True)    # ["three", "four", "two"], longest to shortest
```
"#,
        vec![
            positional(Any),
            Keyword {
                name: Name::new_inline("reverse"),
                ty: non_literal_bool().intern(),
            },
            Keyword {
                name: Name::new_inline("key"),
                ty: Any.intern(),
            },
        ],
        List(Any.intern()),
    );
    add_function(
        "str",
        r#"`str(x)` formats its argument as a string.

If x is a string, the result is x (without quotation).
All other strings, such as elements of a list of strings, are double-quoted.

```python
str(1)                          # '1'
str("x")                        # 'x'
str([1, "x"])                   # '[1, "x"]'
str(0.0)                        # '0.0'        (formatted as if by "%g")
str(b"abc")                     # 'abc'
```

The string form of a bytes value is the UTF-K decoding of the bytes.
Each byte that is not part of a valid encoding is replaced by the
UTF-K encoding of the replacement character, U+FFFD.
"#,
        vec![positional(Any)],
        non_literal_string(),
    );
    // TODO(withered-magic): The tuple returned here can be of any size,
    // might have to introduce a separate type.
    add_function(
        "tuple",
        r#"`tuple(x)` returns a tuple containing the elements of the iterable x.

With no arguments, `tuple()` returns the empty tuple."#,
        vec![positional_opt(Any)],
        Any,
    );
    add_function(
        "type",
        r#"`type(x)` returns a string describing the type of its operand.

```python
type(None)              # "NoneType"
type(0)                 # "int"
type(0.0)               # "float"
    ```"#,
        vec![positional(Any)],
        non_literal_string(),
    );
    add_function(
        "zip",
        r#"`zip()` returns a new list of n-tuples formed from corresponding
elements of each of the n iterable sequences provided as arguments to
`zip`.  That is, the first tuple contains the first element of each of
the sequences, the second element contains the second element of each
of the sequences, and so on.  The result list is only as long as the
shortest of the input sequences.

```python
zip()                                   # []
zip(range(5))                           # [(0,), (1,), (2,), (3,), (4,)]
zip(range(10), ["a", "b", "c"])         # [(0, "a"), (1, "b"), (2, "c")]
```
"#,
        vec![ArgsList { ty: Any.intern() }],
        List(Any.intern()),
    );

    IntrinsicFunctions::new(db, functions)
}

#[salsa::tracked]
pub(crate) fn intrinsic_types(db: &dyn Db) -> Intrinsics {
    Intrinsics::new(
        db,
        Default::default(),
        make_string_base_class(db),
        make_bytes_base_class(db),
        make_list_base_class(db),
        make_dict_base_class(db),
    )
}

fn make_string_base_class(db: &dyn Db) -> IntrinsicClass {
    use IntrinsicFunctionParam::*;
    use TyKind::*;
    IntrinsicClass::new(
        db,
        crate::Name::new_inline("string"),
        0,
        vec![
            function_field(
                db,
                "capitalize",
                r#"`S.capitalize()` returns a copy of string S, where the first character (if any)
is converted to uppercase; all other characters are converted to lowercase.

```python
"hello, world!".capitalize()		# "Hello, world!"
```
"#,
                vec![],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "count",
                r#"`S.count(sub[, start[, end]])` returns the number of occurrences of
`sub` within the string S, or, if the optional substring indices
`start` and `end` are provided, within the designated substring of S.
They are interpreted according to Starlark's [indexing conventions](#indexing).

```python
"hello, world!".count("o")              # 2
"hello, world!".count("o", 7, 12)       # 1  (in "world")
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_int(),
                0,
            ),
            function_field(
                db,
                "elems",
                r#"`S.elems()` returns an opaque iterable value containing successive
1-element substrings of S.
Its type is `"string.elems"`, and its string representation is of the form `"...".elems()`.

```python
"Hello, 123".elems()	        # "Hello, 123".elems()
type("Hello, 123".elems())	# "string.elems"
list("Hello, 123".elems())	# ["H", "e", "l", "l", "o", ",", " ", "1", "2", "3"]
```
"#,
                vec![],
                StringElems,
                0,
            ),
            function_field(
                db,
                "endswith",
                r#"`S.endswith(suffix[, start[, end]])` reports whether the string
`S[start:end]` has the specified suffix.

```python
"filename.sky".endswith(".sky")         # True
"filename.sky".endswith(".sky", 9, 12)  # False
"filename.sky".endswith("name", 0, 8)   # True
```

The `suffix` argument may be a tuple of strings, in which case the
function reports whether any one of them is a suffix.

```python
'foo.cc'.endswith(('.cc', '.h'))         # True
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "find",
                r#"`S.find(sub[, start[, end]])` returns the index of the first
occurrence of the substring `sub` within S.

If either or both of `start` or `end` are specified,
they specify a subrange of S to which the search should be restricted.
They are interpreted according to Starlark's [indexing conventions](#indexing).

If no occurrence is found, `found` returns -1.

```python
"bonbon".find("on")             # 1
"bonbon".find("on", 2)          # 4
"bonbon".find("on", 2, 5)       # -1
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_int(),
                0,
            ),
            // TODO(withered-magic): Handle *args and **kwargs for format().
            function_field(
                db,
                "format",
                r#"`S.format(*args, **kwargs)` returns a version of the format string S
in which bracketed portions `{...}` are replaced
by arguments from `args` and `kwargs`.

Within the format string, a pair of braces `{{` or `}}` is treated as
a literal open or close brace.
Each unpaired open brace must be matched by a close brace `}`.
The optional text between corresponding open and close braces
specifies which argument to use.

```text
{}
{field}
```

The *field name* may be either a decimal number or a keyword.
A number is interpreted as the index of a positional argument;
a keyword specifies the value of a keyword argument.
If all the numeric field names form the sequence 0, 1, 2, and so on,
they may be omitted and those values will be implied; however,
the explicit and implicit forms may not be mixed.

```python
"a{x}b{y}c{}".format(1, x=2, y=3)               # "a2b3c1"
"a{}b{}c".format(1, 2)                          # "a1b2c"
"({1}, {0})".format("zero", "one")              # "(one, zero)"
```
"#,
                vec![ArgsList { ty: Any.intern() }, KwargsDict],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "index",
                r#"`S.index(sub[, start[, end]])` returns the index of the first
occurrence of the substring `sub` within S, like `S.find`, except
that if the substring is not found, the operation fails.

```python
"bonbon".index("on")             # 1
"bonbon".index("on", 2)          # 4
"bonbon".index("on", 2, 5)       # error: substring not found  (in "nbo")
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_int(),
                0,
            ),
            function_field(
                db,
                "isalnum",
                r#"`S.isalnum()` reports whether the string S is non-empty and consists only
Unicode letters and digits.

```python
"base64".isalnum()              # True
"Catch-22".isalnum()            # False
```
"#,
                vec![],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "isalpha",
                r#"`S.isalpha()` reports whether the string S is non-empty and consists only of Unicode letters.

```python
"ABC".isalpha()                 # True
"Catch-22".isalpha()            # False
"".isalpha()                    # False
```
"#,
                vec![],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "isdigit",
                r#"`S.isdigit()` reports whether the string S is non-empty and consists only of Unicode digits.

```python
"123".isdigit()                 # True
"Catch-22".isdigit()            # False
"".isdigit()                    # False
```
"#,
                vec![],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "islower",
                r#"`S.islower()` reports whether the string S contains at least one cased Unicode
letter, and all such letters are lowercase.

```python
"hello, world".islower()        # True
"Catch-22".islower()            # False
"123".islower()                 # False
```
"#,
                vec![],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "isspace",
                r#"`S.isspace()` reports whether the string S is non-empty and consists only of Unicode spaces.

```python
"    ".isspace()                # True
"\r\t\n".isspace()              # True
"".isspace()                    # False
```
"#,
                vec![],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "istitle",
                r#"`S.istitle()` reports whether the string S contains at least one cased Unicode
letter, and all such letters that begin a word are in title case.

```python
"Hello, World!".istitle()       # True
"Catch-22".istitle()            # True
"HAL-9000".istitle()            # False
"123".istitle()                 # False
```
            "#,
                vec![],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "isupper",
                r#"`S.isupper()` reports whether the string S contains at least one cased Unicode
letter, and all such letters are uppercase.

```python
"HAL-9000".isupper()            # True
"Catch-22".isupper()            # False
"123".isupper()                 # False
```
"#,
                vec![],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "join",
                r#"`S.join(iterable)` returns the string formed by concatenating each
element of its argument, with a copy of the string S between
successive elements. The argument must be an iterable whose elements
are strings.

```python
", ".join(["one", "two", "three"])      # "one, two, three"
"a".join("ctmrn".elems())               # "catamaran"
```
"#,
                vec![positional(Any)],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "lower",
                r#"`S.lower()` returns a copy of the string S with letters converted to lowercase.

```python
"Hello, World!".lower()                 # "hello, world!"
```
"#,
                vec![],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "lstrip",
                r#"`S.lstrip([cutset])` returns a copy of the string S with leading whitespace removed.

Like `strip`, it accepts an optional string parameter that specifies an
alternative set of Unicode code points to remove.

```python
"\n hello  ".lstrip()                   # "hello  "
"   hello  ".lstrip("h o")              # "ello  "
```
"#,
                vec![positional_opt(non_literal_string())],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "partition",
                r#"`S.partition(x)` splits string S into three parts and returns them as
a tuple: the portion before the first occurrence of string `x`, `x` itself,
and the portion following it.
If S does not contain `x`, `partition` returns `(S, "", "")`.

`partition` fails if `x` is not a string, or is the empty string.

```python
"one/two/three".partition("/")		# ("one", "/", "two/three")
```
"#,
                vec![positional(non_literal_string())],
                Tuple(TupleVariants::Simple(smallvec![
                    Ty::string(),
                    Ty::string(),
                    Ty::string()
                ])),
                0,
            ),
            function_field(
                db,
                "removeprefix",
                r#"`S.removeprefix(x)` removes the prefix `x` from the string S at most once,
and returns the rest of the string.
If the prefix string is not found then it returns the original string.

`removeprefix` fails if `x` is not a string.

```python
"banana".removeprefix("ban")		# "ana"
"banana".removeprefix("ana")		# "banana"
"bbaa".removeprefix("b")		# "baa"
```
"#,
                vec![positional(non_literal_string())],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "removesuffix",
                r#"`S.removesuffix(x)` removes the suffix `x` from the string S at most once,
and returns the rest of the string.
If the suffix string is not found then it returns the original string.

`removesuffix` fails if `x` is not a string.

```python
"banana".removesuffix("ana")		# "ban"
"banana".removesuffix("ban")		# "banana"
"bbaa".removesuffix("a")		# "bba"
```
"#,
                vec![positional(non_literal_string())],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "replace",
                r#"`S.replace(old, new[, count])` returns a copy of string S with all
occurrences of substring `old` replaced by `new`. If the optional
argument `count`, which must be an `int`, is non-negative, it
specifies a maximum number of occurrences to replace.

```python
"banana".replace("a", "o")		# "bonono"
"banana".replace("a", "o", 2)		# "bonona"
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "rfind",
                r#"`S.rfind(sub[, start[, end]])` returns the index of the substring `sub` within
S, like `S.find`, except that `rfind` returns the index of the substring's
_last_ occurrence.

```python
"bonbon".rfind("on")             # 4
"bonbon".rfind("on", None, 5)    # 1
"bonbon".rfind("on", 2, 5)       # -1
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_int(),
                0,
            ),
            function_field(
                db,
                "rindex",
                r#"`S.rindex(sub[, start[, end]])` returns the index of the substring `sub` within
S, like `S.index`, except that `rindex` returns the index of the substring's
_last_ occurrence.

```python
"bonbon".rindex("on")             # 4
"bonbon".rindex("on", None, 5)    # 1                           (in "bonbo")
"bonbon".rindex("on", 2, 5)       # error: substring not found  (in "nbo")
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_int(),
                0,
            ),
            function_field(
                db,
                "rpartition",
                r#"`S.rpartition(x)` is like `partition`, but splits `S` at the last occurrence of `x`.

```python
"one/two/three".rpartition("/")         # ("one/two", "/", "three")
```
"#,
                vec![positional(non_literal_string())],
                Tuple(TupleVariants::Simple(smallvec![
                    Ty::string(),
                    Ty::string(),
                    Ty::string(),
                ])),
                0,
            ),
            function_field(
                db,
                "rsplit",
                r#"`S.rsplit([sep[, maxsplit]])` splits a string into substrings like `S.split`,
except that when a maximum number of splits is specified, `rsplit` chooses the
rightmost splits.

```python
"banana".rsplit("n")                         # ["ba", "a", "a"]
"banana".rsplit("n", 1)                      # ["bana", "a"]
"one two  three".rsplit(None, 1)             # ["one two", "three"]
```
"#,
                vec![
                    positional(non_literal_string()),
                    positional_opt(non_literal_int()),
                ],
                List(Ty::string()),
                0,
            ),
            function_field(
                db,
                "rstrip",
                r#"`S.rstrip([cutset])` returns a copy of the string S with trailing whitespace removed.

Like `strip`, it accepts an optional string parameter that specifies an
alternative set of Unicode code points to remove.

```python
"  hello\r ".rstrip()                   # "  hello"
"  hello   ".rstrip("h o")              # "  hell"
```
"#,
                vec![positional_opt(non_literal_string())],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "split",
                r#"`S.split([sep [, maxsplit]])` returns the list of substrings of S,
splitting at occurrences of the delimiter string `sep`.

Consecutive occurrences of `sep` are considered to delimit empty
strings, so `'food'.split('o')` returns `['f', '', 'd']`.
Splitting an empty string with a specified separator returns `['']`.
If `sep` is the empty string, `split` fails.

If `sep` is not specified or is `None`, `split` uses a different
algorithm: it removes all leading spaces from S
(or trailing spaces in the case of `rsplit`),
then splits the string around each consecutive non-empty sequence of
Unicode white space characters.

If S consists only of white space, `split` returns the empty list.

If `maxsplit` is given and non-negative, it specifies a maximum number of splits.

```python
"one two  three".split()                    # ["one", "two", "three"]
"one two  three".split(" ")                 # ["one", "two", "", "three"]
"one two  three".split(None, 1)             # ["one", "two  three"]
"banana".split("n")                         # ["ba", "a", "a"]
"banana".split("n", 1)                      # ["ba", "ana"]
```
"#,
                vec![
                    positional_opt(non_literal_string()),
                    positional_opt(non_literal_int()),
                ],
                List(Ty::string()),
                0,
            ),
            function_field(
                db,
                "splitlines",
                r#"`S.splitlines([keepends])` returns a list whose elements are the
successive lines of S, that is, the strings formed by splitting S at
line terminators (currently assumed to be `\n`, `\r` and `\r\n`,
regardless of platform).

The optional argument, `keepends`, is interpreted as a Boolean.
If true, line terminators are preserved in the result, though
the final element does not necessarily end with a line terminator.

```python
"A\nB\rC\r\nD".splitlines()     # ["A", "B", "C", "D"]
"one\n\ntwo".splitlines()       # ["one", "", "two"]
"one\n\ntwo".splitlines(True)   # ["one\n", "\n", "two"]
```
"#,
                vec![positional_opt(non_literal_bool())],
                List(Ty::string()),
                0,
            ),
            function_field(
                db,
                "startswith",
                r#"`S.startswith(prefix[, start[, end]])` reports whether the string
`S[start:end]` has the specified prefix.

```python
"filename.sky".startswith("filename")         # True
"filename.star".startswith("name", 4)         # True
"filename.star".startswith("name", 4, 7)      # False
```

The `prefix` argument may be a tuple of strings, in which case the
function reports whether any one of them is a prefix.

```python
'abc'.startswith(('a', 'A'))                  # True
'ABC'.startswith(('a', 'A'))                  # True
'def'.startswith(('a', 'A'))                  # False
```
"#,
                vec![
                    positional(TyKind::Union(smallvec![
                        Ty::string(),
                        TyKind::Tuple(typeck::Tuple::Variable(Ty::string())).intern()
                    ])),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_bool(),
                0,
            ),
            function_field(
                db,
                "strip",
                r#"`S.strip([cutset])` returns a copy of the string S with leading and trailing whitespace removed.

It accepts an optional string argument,
`cutset`, which instead removes all leading
and trailing Unicode code points contained in `cutset`.

```python
"\rhello\t ".strip()                    # "hello"
"  hello   ".strip("h o")               # "ell"
```
"#,
                vec![positional_opt(non_literal_bool())],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "title",
                r#"`S.title()` returns a copy of the string S with letters converted to titlecase.

Letters are converted to uppercase at the start of words, lowercase elsewhere.

```python
"hElLo, WoRlD!".title()                 # "Hello, World!"
```
"#,
                vec![],
                non_literal_string(),
                0,
            ),
            function_field(
                db,
                "upper",
                r#"
`S.upper()` returns a copy of the string S with letters converted to uppercase.

```python
"Hello, World!".upper()                 # "HELLO, WORLD!"
```
"#,
                vec![],
                non_literal_string(),
                0,
            ),
        ],
    )
}

fn make_bytes_base_class(db: &dyn Db) -> IntrinsicClass {
    use TyKind::*;
    IntrinsicClass::new(
        db,
        crate::Name::new_inline("bytes"),
        0,
        vec![function_field(
            db,
            "elems",
            r#"`b.elems()` returns an opaque iterable value containing successive int elements of b.
Its type is `"bytes.elems"`, and its string representation is of the form `b"...".elems()`.

```python
type(b"ABC".elems())	# "bytes.elems"
b"ABC".elems()	        # b"ABC".elems()
list(b"ABC".elems())  	# [65, 66, 67]
```
<!-- TODO: signpost how to convert an single int or list of int to a bytes. -->

<a id='dict·clear'></a>
### dict·clear

`D.clear()` removes all the entries of dictionary D and returns `None`.
It fails if the dictionary is frozen or if there are active iterators.

```python
x = {"one": 1, "two": 2}
x.clear()                               # None
print(x)                                # {}
```
"#,
            vec![],
            BytesElems,
            0,
        )],
    )
}

fn make_list_base_class(db: &dyn Db) -> IntrinsicClass {
    use TyKind::*;
    IntrinsicClass::new(
        db,
        crate::Name::new_inline("list"),
        1,
        vec![
            function_field(
                db,
                "append",
                r#"`append` fails if the list is frozen or has active iterators.

```python
x = []
x.append(1)                             # None
x.append(2)                             # None
x.append(3)                             # None
x                                       # [1, 2, 3]
```
"#,
                vec![positional(BoundVar(0))],
                None,
                1,
            ),
            function_field(
                db,
                "clear",
                r#"`L.clear()` removes all the elements of the list L and returns `None`.
It fails if the list is frozen or if there are active iterators.

```python
x = [1, 2, 3]
x.clear()                               # None
x                                       # []
```
"#,
                vec![],
                None,
                1,
            ),
            function_field(
                db,
                "extend",
                r#"`L.extend(x)` appends the elements of `x`, which must be iterable, to
the list L, and returns `None`.

It is permissible to extend the list with itself. The operation
doubles the list.

`extend` fails if `x` is not iterable, or if the list L is frozen or has active iterators.

```python
x = []
x.extend([1, 2, 3])                     # None
x.extend(["foo"])                       # None
x                                       # [1, 2, 3, "foo"]

y = [1, 2]
y.extend(y)
y                                       # [1, 2, 1, 2]
```
"#,
                vec![positional(Any)],
                None,
                1,
            ),
            function_field(
                db,
                "index",
                r#"`L.index(x[, start[, end]])` finds `x` within the list L and returns its index.

The optional `start` and `end` parameters restrict the portion of
list L that is inspected.  If provided and not `None`, they must be list
indices of type `int`. If an index is negative, `len(L)` is effectively
added to it, then if the index is outside the range `[0:len(L)]`, the
nearest value within that range is used; see [Indexing](#indexing).

`index` fails if `x` is not found in L, or if `start` or `end`
is not a valid index (`int` or `None`).
To avoid this error, test `x in list` before calling `list.index(x)`.

```python
x = ["b", "a", "n", "a", "n", "a"]
x.index("a")                            # 1 (bAnana)
x.index("a", 2)                         # 3 (banAna)
x.index("a", -2)                        # 5 (bananA)
```
"#,
                vec![
                    positional(BoundVar(0)),
                    positional_opt(non_literal_int()),
                    positional_opt(non_literal_int()),
                ],
                non_literal_int(),
                1,
            ),
            function_field(
                db,
                "insert",
                r#"`L.insert(i, x)` inserts the value `x` in the list L at index `i`, moving
higher-numbered elements along by one.  It returns `None`.

As usual, the index `i` must be an `int`. If its value is negative,
the length of the list is added, then its value is clamped to the
nearest value in the range `[0:len(L)]` to yield the effective index.

`insert` fails if the list is frozen or has active iterators.

```python
x = ["b", "c", "e"]
x.insert(0, "a")                        # None
x.insert(-1, "d")                       # None
x                                       # ["a", "b", "c", "d", "e"]
```
"#,
                vec![positional(non_literal_int()), positional(BoundVar(0))],
                None,
                1,
            ),
            function_field(
                db,
                "pop",
                r#"`L.pop([index])` removes and returns the last element of the list L, or,
if the optional index is provided, at that index.

`pop` fails if the index is negative or not less than the length of
the list, of if the list is frozen or has active iterators.

```python
x = [1, 2, 3]
x.pop()                                 # 3
x.pop()                                 # 2
x                                       # [1]
```
"#,
                vec![positional_opt(non_literal_int())],
                BoundVar(0),
                1,
            ),
            function_field(
                db,
                "remove",
                r#"`L.remove(x)` removes the first occurrence of the value `x` from the list L, and returns `None`.

`remove` fails if the list does not contain `x`, is frozen, or has active iterators.

```python
x = [1, 2, 3, 2]
x.remove(2)                             # None (x == [1, 3, 2])
x.remove(2)                             # None (x == [1, 3])
x.remove(2)                             # error: element not found
```
"#,
                vec![positional(BoundVar(0))],
                None,
                1,
            ),
        ],
    )
}

fn make_dict_base_class(db: &dyn Db) -> IntrinsicClass {
    use IntrinsicFunctionParam::*;
    use TyKind::*;
    IntrinsicClass::new(
        db,
        crate::Name::new_inline("dict"),
        2,
        vec![
            function_field(
                db,
                "clear",
                r#"`D.clear()` removes all the entries of dictionary D and returns `None`.
It fails if the dictionary is frozen or if there are active iterators.

```python
x = {"one": 1, "two": 2}
x.clear()                               # None
print(x)                                # {}
```
"#,
                vec![],
                None,
                2,
            ),
            function_field(
                db,
                "get",
                r#"`D.get(key[, default])` returns the dictionary value corresponding to the given key.
If the dictionary contains no such value, `get` returns `None`, or the
value of the optional `default` parameter if present.

`get` fails if `key` is unhashable, or the dictionary is frozen or has active iterators.

```python
x = {"one": 1, "two": 2}
x.get("one")                            # 1
x.get("three")                          # None
x.get("three", 0)                       # 0
```
"#,
                vec![positional(BoundVar(0)), positional_opt(BoundVar(1))],
                BoundVar(1),
                2,
            ),
            function_field(
                db,
                "items",
                r#"`D.items()` returns a new list of key/value pairs, one per element in
dictionary D, in the same order as they would be returned by a `for` loop.

```python
x = {"one": 1, "two": 2}
x.items()                               # [("one", 1), ("two", 2)]
```
"#,
                vec![],
                List(
                    Tuple(TupleVariants::Simple(smallvec![
                        BoundVar(0).intern(),
                        BoundVar(1).intern()
                    ]))
                    .intern(),
                ),
                2,
            ),
            function_field(
                db,
                "keys",
                r#"`D.keys()` returns a new list containing the keys of dictionary D, in the
same order as they would be returned by a `for` loop.

```python
x = {"one": 1, "two": 2}
x.keys()                               # ["one", "two"]
```
"#,
                vec![],
                List(BoundVar(0).intern()),
                2,
            ),
            function_field(
                db,
                "pop",
                r#"`D.pop(key[, default])` returns the value corresponding to the specified
key, and removes it from the dictionary.  If the dictionary contains no
such value, and the optional `default` parameter is present, `pop`
returns that value; otherwise, it fails.

`pop` fails if `key` is unhashable, or the dictionary is frozen or has active iterators.

```python
x = {"one": 1, "two": 2}
x.pop("one")                            # 1
x                                       # {"two": 2}
x.pop("three", 0)                       # 0
x.pop("four")                           # error: missing key
```
"#,
                vec![positional(BoundVar(0)), positional_opt(BoundVar(1))],
                BoundVar(1),
                2,
            ),
            function_field(
                db,
                "popitem",
                r#"`D.popitem()` returns the first key/value pair, removing it from the dictionary.

`popitem` fails if the dictionary is empty, frozen, or has active iterators.

```python
x = {"one": 1, "two": 2}
x.popitem()                             # ("one", 1)
x.popitem()                             # ("two", 2)
x.popitem()                             # error: empty dict
```
"#,
                vec![],
                Tuple(TupleVariants::Simple(smallvec![
                    BoundVar(0).intern(),
                    BoundVar(1).intern()
                ])),
                2,
            ),
            function_field(
                db,
                "setdefault",
                r#"`D.setdefault(key[, default])` returns the dictionary value corresponding to the given key.
If the dictionary contains no such value, `setdefault`, like `get`,
returns `None` or the value of the optional `default` parameter if
present; `setdefault` additionally inserts the new key/value entry into the dictionary.

`setdefault` fails if the key is unhashable, or if the dictionary is frozen or has active iterators.

```python
x = {"one": 1, "two": 2}
x.setdefault("one")                     # 1
x.setdefault("three", 3)                # 3
x                                       # {"one": 1, "two": 2, "three": 3}
x.setdefault("three", 33)               # 3
x                                       # {"one": 1, "two": 2, "three": 3}
x.setdefault("four")                    # None
x                                       # {"one": 1, "two": 2, "three": 3, "four": None}
```
"#,
                vec![positional(BoundVar(0)), positional_opt(BoundVar(1))],
                BoundVar(1),
                2,
            ),
            function_field(
                db,
                "update",
                r#"`D.update([pairs][, name=value[, ...])` makes a sequence of key/value
insertions into dictionary D, then returns `None.`

If the positional argument `pairs` is present, it must be `None`,
another `dict`, or some other iterable.
If it is another `dict`, then its key/value pairs are inserted into D.
If it is an iterable, it must provide a sequence of pairs (or other iterables of length 2),
each of which is treated as a key/value pair to be inserted into D.

Then, for each `name=value` argument present, an entry with key `name`
and value `value` is inserted into D.

All insertions overwrite any previous entries having the same key.

It is permissible to update the dict with itself given as pairs.
The operation is no-op.

`update` fails if the dictionary is frozen or has active iterators.

```python
x = {}
x.update([("a", 1), ("b", 2)], c=3)
x.update({"d": 4})
x.update(e=5)
x                                       # {"a": 1, "b": "2", "c": 3, "d": 4, "e": 5}
```
"#,
                vec![
                    positional_opt(TyKind::Union(smallvec![
                        Ty::dict(Ty::unknown(), Ty::unknown(), std::option::Option::None,),
                        TyKind::Protocol(typeck::Protocol::Iterable(
                            TyKind::Protocol(typeck::Protocol::Iterable(Ty::any())).intern()
                        ))
                        .intern(),
                        Ty::none(),
                    ])),
                    KwargsDict,
                ],
                None,
                2,
            ),
            function_field(
                db,
                "values",
                r#"`D.values()` returns a new list containing the dictionary's values, in the
same order as they would be returned by a `for` loop over the
dictionary.

```python
x = {"one": 1, "two": 2}
x.values()                              # [1, 2]
```
"#,
                vec![],
                List(BoundVar(1).intern()),
                2,
            ),
        ],
    )
}

fn function(
    db: &dyn Db,
    name: &'static str,
    doc: &'static str,
    params: Vec<IntrinsicFunctionParam>,
    num_vars: usize,
    ret_ty: TyKind,
) -> IntrinsicFunction {
    IntrinsicFunction::new(
        db,
        Name::new_inline(name),
        doc.to_string(),
        num_vars,
        params,
        ret_ty.intern(),
        name == "dict",
    )
}

fn function_field(
    db: &dyn Db,
    name: &'static str,
    doc: &'static str,
    params: Vec<IntrinsicFunctionParam>,
    ret_ty: TyKind,
    num_vars: usize,
) -> IntrinsicField {
    IntrinsicField::new_inline(
        name,
        doc,
        TyKind::IntrinsicFunction(
            function(db, name, doc, params, num_vars, ret_ty),
            Substitution::new_identity(num_vars),
        )
        .intern(),
    )
}

fn positional(kind: TyKind) -> IntrinsicFunctionParam {
    IntrinsicFunctionParam::Positional {
        ty: kind.intern(),
        optional: false,
    }
}

fn positional_opt(kind: TyKind) -> IntrinsicFunctionParam {
    IntrinsicFunctionParam::Positional {
        ty: kind.intern(),
        optional: true,
    }
}

fn non_literal_string() -> TyKind {
    TyKind::String(None)
}

fn non_literal_bool() -> TyKind {
    TyKind::Bool(None)
}

fn non_literal_int() -> TyKind {
    TyKind::Int(None)
}
