use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct ProgramArguments {
    /// The filename for ri2p configurations
    #[clap(short, long, default_value = ".ri2p.config.toml")]
    pub config_filename: String,
}
