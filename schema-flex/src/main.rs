//! Post-processor for Quicktype-generated JSON Schema.
//!
//! This tool makes the schema more resilient to API changes by:
//! 1. Removing `"additionalProperties": false` - allows unknown fields to be ignored
//! 2. Converting strict string enums to allow unknown values via `anyOf`
//!
//! # Usage
//!
//! ```bash
//! schema-flex schema.json
//! schema-flex schema.json --output flexible-schema.json
//! schema-flex schema.json --dry-run
//! schema-flex schema.json --no-flexible-enums
//! ```

use serde_json::{Map, Value};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
    no_flexible_enums: bool,
    dry_run: bool,
}

fn print_usage() {
    eprintln!("schema-flex - Make a Quicktype-generated JSON Schema more flexible for Typify");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    schema-flex <INPUT> [OPTIONS]");
    eprintln!();
    eprintln!("ARGS:");
    eprintln!("    <INPUT>    Input JSON Schema file");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    -o, --output <FILE>     Output file (default: overwrite input)");
    eprintln!("    --no-flexible-enums     Don't make string enums flexible");
    eprintln!("    --dry-run               Print the result instead of writing to file");
    eprintln!("    -h, --help              Print help");
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err("Missing input file".to_string());
    }

    let mut input: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut no_flexible_enums = false;
    let mut dry_run = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    return Err("--output requires a value".to_string());
                }
                output = Some(PathBuf::from(&args[i]));
            }
            "--no-flexible-enums" => {
                no_flexible_enums = true;
            }
            "--dry-run" => {
                dry_run = true;
            }
            arg if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg));
            }
            arg => {
                if input.is_some() {
                    return Err("Multiple input files not supported".to_string());
                }
                input = Some(PathBuf::from(arg));
            }
        }
        i += 1;
    }

    let input = input.ok_or("Missing input file")?;

    Ok(Args {
        input,
        output,
        no_flexible_enums,
        dry_run,
    })
}

/// Recursively remove `"additionalProperties": false` from all objects.
/// This prevents serde from adding `#[serde(deny_unknown_fields)]`.
fn remove_additional_properties_false(value: &mut Value) {
    match value {
        Value::Object(map) => {
            // Remove additionalProperties: false, but keep other values
            if map.get("additionalProperties") == Some(&Value::Bool(false)) {
                map.remove("additionalProperties");
            }

            // Recurse into all values
            for (_, v) in map.iter_mut() {
                remove_additional_properties_false(v);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                remove_additional_properties_false(item);
            }
        }
        _ => {}
    }
}

/// Check if a definition is a string enum (type: string with enum array of strings)
fn is_string_enum(definition: &Map<String, Value>) -> bool {
    let is_string_type = definition.get("type") == Some(&Value::String("string".to_string()));

    let has_string_enum = definition.get("enum").map_or(false, |e| {
        if let Value::Array(arr) = e {
            !arr.is_empty() && arr.iter().all(|v| v.is_string())
        } else {
            false
        }
    });

    is_string_type && has_string_enum
}

/// Convert a strict string enum to just a plain string type.
///
/// Before:
/// ```json
/// { "type": "string", "enum": ["A", "B", "C"], "title": "Status" }
/// ```
///
/// After:
/// ```json
/// { "type": "string", "title": "Status" }
/// ```
///
/// This causes Typify to generate a String type that accepts any value,
/// avoiding deserialization failures when new enum variants appear.
fn make_enum_flexible(definition: &mut Map<String, Value>) -> Option<Value> {
    if !is_string_enum(definition) {
        return None;
    }

    // Remove the enum constraint, keeping it as just a string
    definition.remove("enum");

    // Return None to indicate we modified in place, not replaced
    None
}

/// Process all definitions in the schema
fn process_definitions(schema: &mut Value, make_enums_flexible: bool) {
    let definitions = match schema.get_mut("definitions") {
        Some(Value::Object(defs)) => defs,
        _ => return,
    };

    if make_enums_flexible {
        for (_name, def) in definitions.iter_mut() {
            if let Value::Object(map) = def {
                make_enum_flexible(map);
            }
        }
    }
}

/// Process the entire schema to make it more flexible
fn process_schema(schema: &mut Value, make_enums_flexible: bool) {
    // Remove additionalProperties: false throughout
    remove_additional_properties_false(schema);

    // Process definitions
    process_definitions(schema, make_enums_flexible);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!();
            print_usage();
            process::exit(1);
        }
    };

    // Read input
    let content = fs::read_to_string(&args.input)?;
    let mut schema: Value = serde_json::from_str(&content)?;

    // Process
    process_schema(&mut schema, !args.no_flexible_enums);

    // Output
    let output_json = serde_json::to_string_pretty(&schema)?;

    if args.dry_run {
        println!("{}", output_json);
    } else {
        let output_path = args.output.as_ref().unwrap_or(&args.input);
        fs::write(output_path, format!("{}\n", output_json))?;
        println!("✓ Processed schema written to {}", output_path.display());
    }

    Ok(())
}
