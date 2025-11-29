//! Test harness generator for function-based challenges.
//!
//! Wraps user-written functions with a main() that calls them with test inputs
//! and prints the result for validation.

use super::loader::{FunctionSignature, TestCase};

/// Generate a complete C program that wraps the user's function with a test harness
pub fn generate_harness(
    user_code: &str,
    signature: &FunctionSignature,
    test_case: &TestCase,
) -> Result<String, String> {
    let call_args = format_call_args(&signature.parameters, &test_case.input)?;
    let print_format = get_print_format(&signature.return_type)?;

    let harness = format!(
        r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// User's function
{user_code}

int main() {{
    {result_decl}
    {print_stmt}
    return 0;
}}
"#,
        user_code = user_code,
        result_decl = generate_result_decl(&signature, &call_args),
        print_stmt = generate_print_stmt(&signature.return_type, &print_format),
    );

    Ok(harness)
}

/// Format the arguments for the function call based on parameter types and test input
fn format_call_args(
    params: &[super::loader::FunctionParameter],
    input: &[serde_json::Value],
) -> Result<String, String> {
    if params.len() != input.len() {
        return Err(format!(
            "Parameter count mismatch: expected {}, got {}",
            params.len(),
            input.len()
        ));
    }

    let args: Result<Vec<String>, String> = params
        .iter()
        .zip(input.iter())
        .map(|(param, value)| format_single_arg(&param.param_type, value))
        .collect();

    Ok(args?.join(", "))
}

/// Format a single argument value based on its type
fn format_single_arg(param_type: &str, value: &serde_json::Value) -> Result<String, String> {
    match param_type {
        "int" | "long" | "short" => {
            let n = value
                .as_i64()
                .ok_or_else(|| format!("Expected integer, got {:?}", value))?;
            Ok(n.to_string())
        }
        "unsigned int" | "unsigned long" | "size_t" => {
            let n = value
                .as_u64()
                .ok_or_else(|| format!("Expected unsigned integer, got {:?}", value))?;
            Ok(n.to_string())
        }
        "float" | "double" => {
            let n = value
                .as_f64()
                .ok_or_else(|| format!("Expected float, got {:?}", value))?;
            Ok(format!("{:.6}", n))
        }
        "char" => {
            let s = value
                .as_str()
                .ok_or_else(|| format!("Expected char string, got {:?}", value))?;
            if s.len() != 1 {
                return Err(format!("Expected single char, got '{}'", s));
            }
            Ok(format!("'{}'", s.chars().next().unwrap()))
        }
        t if t.contains("char*") || t.contains("char *") || t == "string" => {
            let s = value
                .as_str()
                .ok_or_else(|| format!("Expected string, got {:?}", value))?;
            Ok(format!("\"{}\"", escape_c_string(s)))
        }
        _ => Err(format!("Unsupported parameter type: {}", param_type)),
    }
}

/// Get the printf format specifier for a return type
fn get_print_format(return_type: &str) -> Result<&'static str, String> {
    match return_type {
        "int" | "short" => Ok("%d"),
        "long" => Ok("%ld"),
        "unsigned int" => Ok("%u"),
        "unsigned long" | "size_t" => Ok("%lu"),
        "float" => Ok("%f"),
        "double" => Ok("%lf"),
        "char" => Ok("%c"),
        "char*" | "char *" | "string" => Ok("%s"),
        "void" => Ok(""), // No print for void
        _ => Err(format!("Unsupported return type: {}", return_type)),
    }
}

/// Generate the result declaration and function call
fn generate_result_decl(signature: &FunctionSignature, call_args: &str) -> String {
    if signature.return_type == "void" {
        format!("{}({});", signature.name, call_args)
    } else {
        format!(
            "{} result = {}({});",
            signature.return_type, signature.name, call_args
        )
    }
}

/// Generate the print statement for the result
fn generate_print_stmt(return_type: &str, format: &str) -> String {
    if return_type == "void" {
        String::from("printf(\"done\\n\");")
    } else {
        format!("printf(\"{}\\n\", result);", format)
    }
}

/// Escape special characters in a C string
fn escape_c_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::levels::loader::FunctionParameter;

    #[test]
    fn test_generate_simple_harness() {
        let user_code = "int add(int a, int b) { return a + b; }";
        let signature = FunctionSignature {
            name: "add".to_string(),
            return_type: "int".to_string(),
            parameters: vec![
                FunctionParameter {
                    name: "a".to_string(),
                    param_type: "int".to_string(),
                },
                FunctionParameter {
                    name: "b".to_string(),
                    param_type: "int".to_string(),
                },
            ],
        };
        let test_case = TestCase {
            input: vec![serde_json::json!(2), serde_json::json!(3)],
            expected: "5".to_string(),
            sample: true,
        };

        let harness = generate_harness(user_code, &signature, &test_case).unwrap();
        assert!(harness.contains("int result = add(2, 3);"));
        assert!(harness.contains("printf(\"%d\\n\", result);"));
    }

    #[test]
    fn test_void_function() {
        let signature = FunctionSignature {
            name: "hello".to_string(),
            return_type: "void".to_string(),
            parameters: vec![],
        };
        let test_case = TestCase {
            input: vec![],
            expected: "Hello, World!".to_string(),
            sample: true,
        };

        let harness = generate_harness("void hello() { printf(\"Hello, World!\\n\"); }", &signature, &test_case).unwrap();
        assert!(harness.contains("hello();"));
        assert!(harness.contains("printf(\"done\\n\");"));
    }
}
