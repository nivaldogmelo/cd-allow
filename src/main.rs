use anyhow::Error;

mod cli;
mod domain;
mod licenses;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = cli::parse_args();

    licenses::get_licenses(&args).await?;

    println!("The file {:?} has been updated", args.filepath);

    Ok(())
}
