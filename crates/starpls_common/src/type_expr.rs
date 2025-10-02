/*!
Type expression parser for JSON extension type definitions.

This module parses type expressions like:
- Basic types: `string`, `int`, `bool`, `any`
- Generic types: `list<T>`, `dict<K,V>`, `tuple<T1,T2>`
- Union types: `string | int`
- Function types: `function(int, string) -> bool`
- Optional types: `?string` (shorthand for `string | None`)
- Custom types: `Atom`, `ExecContext`
*/

use std::fmt;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeExpr {
    /// Primitive type by name (string, int, bool, any, None, etc.)
    Primitive(String),

    /// List type: list<T>
    List(Box<TypeExpr>),

    /// Dictionary type: dict<K, V>
    Dict(Box<TypeExpr>, Box<TypeExpr>),

    /// Tuple type: tuple<T1, T2, ...>
    Tuple(Vec<TypeExpr>),

    /// Union type: T1 | T2 | ...
    Union(Vec<TypeExpr>),

    /// Function type: function(param_types...) -> return_type
    Function {
        params: Vec<TypeExpr>,
        return_type: Box<TypeExpr>,
    },

    /// Reference to a custom type by name
    Reference(String),

    /// Optional type (shorthand for T | None)
    Optional(Box<TypeExpr>),
}

impl fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeExpr::Primitive(name) => write!(f, "{}", name),
            TypeExpr::List(inner) => write!(f, "list<{}>", inner),
            TypeExpr::Dict(key, value) => write!(f, "dict<{}, {}>", key, value),
            TypeExpr::Tuple(types) => {
                write!(f, "tuple<")?;
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", ty)?;
                }
                write!(f, ">")
            }
            TypeExpr::Union(types) => {
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 { write!(f, " | ")?; }
                    write!(f, "{}", ty)?;
                }
                Ok(())
            }
            TypeExpr::Function { params, return_type } => {
                write!(f, "function(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            TypeExpr::Reference(name) => write!(f, "{}", name),
            TypeExpr::Optional(inner) => write!(f, "?{}", inner),
        }
    }
}

impl TypeExpr {
    /// Parse a type expression from a string.
    pub fn parse(input: &str) -> Result<Self> {
        let input = input.trim();

        // Handle empty or unknown types
        if input.is_empty() || input == "unknown" {
            return Ok(TypeExpr::Primitive("any".to_string()));
        }

        // Handle optional types: ?T
        if let Some(inner) = input.strip_prefix('?') {
            return Ok(TypeExpr::Optional(Box::new(Self::parse(inner)?)));
        }

        // Handle union types: T1 | T2 | ...
        if input.contains(" | ") {
            let parts: Vec<&str> = input.split(" | ").collect();
            if parts.len() > 1 {
                let types: Result<Vec<_>> = parts.iter().map(|part| Self::parse(part)).collect();
                return Ok(TypeExpr::Union(types?));
            }
        }

        // Handle function types: function(...) -> ...
        if input.starts_with("function(") {
            return Self::parse_function_type(input);
        }

        // Handle generic types: type<args...>
        if let Some((base, args_str)) = Self::split_generic(input) {
            return Self::parse_generic_type(base, args_str);
        }

        // Handle primitive types and references
        if Self::is_primitive_type(input) {
            Ok(TypeExpr::Primitive(input.to_string()))
        } else {
            Ok(TypeExpr::Reference(input.to_string()))
        }
    }

    /// Check if a type name is a primitive Starlark type.
    fn is_primitive_type(name: &str) -> bool {
        matches!(name,
            "any" | "string" | "int" | "bool" | "float" | "bytes" |
            "None" | "NoneType" | "list" | "dict" | "tuple" | "set" | "range"
        )
    }

    /// Split a generic type like "list<int>" into ("list", "int").
    fn split_generic(input: &str) -> Option<(&str, &str)> {
        if let Some(open_pos) = input.find('<') {
            if input.ends_with('>') {
                let base = &input[..open_pos];
                let args = &input[open_pos + 1..input.len() - 1];
                return Some((base, args));
            }
        }
        None
    }

    /// Parse a generic type like list<T>, dict<K,V>, etc.
    fn parse_generic_type(base: &str, args_str: &str) -> Result<Self> {
        match base {
            "list" => {
                if args_str.is_empty() {
                    Ok(TypeExpr::List(Box::new(TypeExpr::Primitive("any".to_string()))))
                } else {
                    Ok(TypeExpr::List(Box::new(Self::parse(args_str)?)))
                }
            }
            "dict" => {
                let args = Self::split_comma_args(args_str)?;
                if args.is_empty() {
                    Ok(TypeExpr::Dict(
                        Box::new(TypeExpr::Primitive("any".to_string())),
                        Box::new(TypeExpr::Primitive("any".to_string())),
                    ))
                } else if args.len() == 1 {
                    Ok(TypeExpr::Dict(
                        Box::new(TypeExpr::Primitive("string".to_string())),
                        Box::new(Self::parse(&args[0])?),
                    ))
                } else if args.len() == 2 {
                    Ok(TypeExpr::Dict(
                        Box::new(Self::parse(&args[0])?),
                        Box::new(Self::parse(&args[1])?),
                    ))
                } else {
                    Err(anyhow!("dict type expects 0-2 type arguments, got {}", args.len()))
                }
            }
            "tuple" => {
                let args = Self::split_comma_args(args_str)?;
                let types: Result<Vec<_>> = args.iter().map(|arg| Self::parse(arg)).collect();
                Ok(TypeExpr::Tuple(types?))
            }
            _ => Err(anyhow!("Unknown generic type: {}", base))
        }
    }

    /// Parse a function type like "function(int, string) -> bool".
    fn parse_function_type(input: &str) -> Result<Self> {
        if !input.starts_with("function(") {
            return Err(anyhow!("Invalid function type: {}", input));
        }

        // Find the closing ) for parameters
        let mut paren_count = 0;
        let mut end_pos = None;
        for (i, ch) in input.char_indices() {
            match ch {
                '(' => paren_count += 1,
                ')' => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        end_pos = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }

        let end_pos = end_pos.ok_or_else(|| anyhow!("Unclosed parentheses in function type"))?;
        let params_str = &input[9..end_pos]; // Skip "function("
        let rest = &input[end_pos + 1..].trim();

        // Parse parameters
        let param_types = if params_str.is_empty() {
            Vec::new()
        } else {
            let param_strs = Self::split_comma_args(params_str)?;
            let types: Result<Vec<_>> = param_strs.iter().map(|s| Self::parse(s)).collect();
            types?
        };

        // Parse return type
        let return_type = if rest.starts_with("-> ") {
            Self::parse(&rest[3..])?
        } else if rest.is_empty() {
            TypeExpr::Primitive("None".to_string())
        } else {
            return Err(anyhow!("Invalid function type syntax: {}", input));
        };

        Ok(TypeExpr::Function {
            params: param_types,
            return_type: Box::new(return_type),
        })
    }

    /// Split comma-separated arguments, respecting nested brackets.
    fn split_comma_args(input: &str) -> Result<Vec<String>> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut args = Vec::new();
        let mut current = String::new();
        let mut bracket_count = 0;
        let mut paren_count = 0;

        for ch in input.chars() {
            match ch {
                ',' if bracket_count == 0 && paren_count == 0 => {
                    args.push(current.trim().to_string());
                    current.clear();
                }
                '<' => {
                    bracket_count += 1;
                    current.push(ch);
                }
                '>' => {
                    bracket_count -= 1;
                    current.push(ch);
                }
                '(' => {
                    paren_count += 1;
                    current.push(ch);
                }
                ')' => {
                    paren_count -= 1;
                    current.push(ch);
                }
                _ => current.push(ch),
            }
        }

        if !current.trim().is_empty() {
            args.push(current.trim().to_string());
        }

        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_types() {
        assert_eq!(TypeExpr::parse("string").unwrap(), TypeExpr::Primitive("string".to_string()));
        assert_eq!(TypeExpr::parse("int").unwrap(), TypeExpr::Primitive("int".to_string()));
        assert_eq!(TypeExpr::parse("any").unwrap(), TypeExpr::Primitive("any".to_string()));
    }

    #[test]
    fn test_generic_types() {
        assert_eq!(
            TypeExpr::parse("list<string>").unwrap(),
            TypeExpr::List(Box::new(TypeExpr::Primitive("string".to_string())))
        );

        assert_eq!(
            TypeExpr::parse("dict<string, int>").unwrap(),
            TypeExpr::Dict(
                Box::new(TypeExpr::Primitive("string".to_string())),
                Box::new(TypeExpr::Primitive("int".to_string()))
            )
        );
    }

    #[test]
    fn test_union_types() {
        assert_eq!(
            TypeExpr::parse("string | int").unwrap(),
            TypeExpr::Union(vec![
                TypeExpr::Primitive("string".to_string()),
                TypeExpr::Primitive("int".to_string())
            ])
        );
    }

    #[test]
    fn test_optional_types() {
        assert_eq!(
            TypeExpr::parse("?string").unwrap(),
            TypeExpr::Optional(Box::new(TypeExpr::Primitive("string".to_string())))
        );
    }

    #[test]
    fn test_function_types() {
        assert_eq!(
            TypeExpr::parse("function(int, string) -> bool").unwrap(),
            TypeExpr::Function {
                params: vec![
                    TypeExpr::Primitive("int".to_string()),
                    TypeExpr::Primitive("string".to_string())
                ],
                return_type: Box::new(TypeExpr::Primitive("bool".to_string()))
            }
        );
    }

    #[test]
    fn test_custom_types() {
        assert_eq!(
            TypeExpr::parse("Atom").unwrap(),
            TypeExpr::Reference("Atom".to_string())
        );
    }
}