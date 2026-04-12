use henze_ds::{self, HenzeFilter};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let filter = match args.len() {
        1 => HenzeFilter::default(),
        2 => {
            let target: f64 = args[1].parse().expect("Invalid target odds");
            HenzeFilter::new(target, henze_ds::DEFAULT_TOLERANCE)
        }
        3 => {
            let target: f64 = args[1].parse().expect("Invalid target odds");
            let tolerance: f64 = args[2].parse().expect("Invalid tolerance");
            HenzeFilter::new(target, tolerance)
        }
        _ => {
            eprintln!("Usage: henze-ds-cli [target_odds] [tolerance]");
            eprintln!("  Default: target=1.1, tolerance=0.04");
            std::process::exit(1);
        }
    };

    println!(
        "Filtering for odds {:.2} ± {:.2} (range: {:.2} - {:.2})",
        filter.target,
        filter.tolerance,
        filter.min_odds(),
        filter.max_odds()
    );

    let collected_info = henze_ds::retrieve_henze_data_with_filter(filter).await?;

    println!("Found {} matching bets:\n", collected_info.len());
    for info in collected_info {
        println!("{:?}", info);
    }

    Ok(())
}