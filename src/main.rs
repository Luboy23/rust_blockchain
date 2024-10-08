use blockchain_rust::cli::Cli;
use blockchain_rust::errors::Result;



fn main() -> Result<()>{
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
