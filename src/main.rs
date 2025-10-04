mod cli;

use kuro::config::Config;

fn main() -> anyhow::Result<()> {
    // let value =  cli::Cli::parse();
    let _config = Config::load()?;
    Ok(())
}
