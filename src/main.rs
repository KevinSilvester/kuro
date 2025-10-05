mod cli;

use clap::Parser;

use kuro::config::Config;

use crate::cli::Args;

fn main() -> anyhow::Result<()> {
    let value =  Args::parse();
    let _config = Config::load(value.kuro_dir)?;
    Ok(())
}
