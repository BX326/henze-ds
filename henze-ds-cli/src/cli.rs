//! Command-line argument parsing and validation.

use henze_ds::HenzeFilter;

/// Parsed command-line arguments.
pub struct CliArgs {
    pub target: f64,
    pub tolerance: f64,
}

/// Parse command-line arguments into a structured format.
///
/// # Arguments
/// - `args`: The raw command-line arguments (including program name).
///
/// # Returns
/// - `Ok(CliArgs)` with parsed values, or exits with usage message on error.
pub fn parse_args(args: Vec<String>) -> CliArgs {
    match args.len() {
        1 => CliArgs {
            target: henze_ds::DEFAULT_TARGET_ODDS,
            tolerance: henze_ds::DEFAULT_TOLERANCE,
        },
        2 => {
            let target: f64 = args[1].parse().expect("Invalid target odds");
            CliArgs {
                target,
                tolerance: henze_ds::DEFAULT_TOLERANCE,
            }
        }
        3 => {
            let target: f64 = args[1].parse().expect("Invalid target odds");
            let tolerance: f64 = args[2].parse().expect("Invalid tolerance");
            CliArgs { target, tolerance }
        }
        _ => {
            eprintln!("Usage: henze-ds-cli [target_odds] [tolerance]");
            eprintln!("  Default: target=1.1, tolerance=0.04");
            std::process::exit(1);
        }
    }
}

/// Create a HenzeFilter from parsed CLI arguments.
pub fn create_filter(args: &CliArgs) -> HenzeFilter {
    HenzeFilter::new(args.target, args.tolerance)
}
