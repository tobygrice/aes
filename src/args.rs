use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, author, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encrypt input to output
    Encrypt(EncryptArgs),

    /// Decrypt input to output
    Decrypt(CommonArgs),
}

#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct CommonArgs {
    /// Input file path. Use '-' to read from stdin
    #[arg(short = 'i', long = "input")]
    pub input: PathBuf,

    /// Output file path. Use '-' to write to stdout
    #[arg(short = 'o', long = "output")]
    pub output: PathBuf,

    /// Key file path. Use '-' for stdin/stdout (inferred from COMMAND)
    #[arg(short = 'k', long = "key")]
    pub key: PathBuf,
}


#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct EncryptArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Generate a random key (written to path specified by key)
    #[arg(long = "generate-key")]
    pub generate_key: bool,

    /// Only valid with --generate-key.
    #[arg(
        long = "key-size",
        value_enum,
        default_value_t = KeySize::Bits256,
        requires = "generate_key"
    )]
    pub key_size: KeySize,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum KeySize {
    #[value(name = "128")]
    Bits128,
    #[value(name = "192")]
    Bits192,
    #[value(name = "256")]
    Bits256,
}
