use std::path::PathBuf;


/// A cross-platform dotfiles/configuration management tool
#[derive(clap::Parser, Debug)]
#[clap(name = "kuro", version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    /// Hello
    #[clap(short, long, env = "KURO_DIR", value_parser)]
    pub kuro_dir: Option<PathBuf>,
}
