
/// A cross-platform dotfiles/configuration management tool
#[derive(clap::Parser, Debug)]
#[clap(name = "kuro", version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Hello
    #[clap(short, long)]
    name: String,
}
