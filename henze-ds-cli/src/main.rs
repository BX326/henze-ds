use henze_ds;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let collected_info = henze_ds::retrieve_henze_data().await?;

    for info in collected_info {
        println!("{:?}", info);
    }

    Ok(())
}