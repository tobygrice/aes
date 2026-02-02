mod args;

use args::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Encrypt(enc) => {
            // common args:
            let input = enc.common.input;
            let output = enc.common.output;
            let key = enc.common.key;

            // encrypt-only args:
            let key_gen = enc.generate_key;
            let key_size = enc.key_size;

            println!("encrypt: {input:?} -> {output:?}, key={key:?}, key_gen={key_gen}, size={key_size:?}");
        }
        Commands::Decrypt(common) => {
            let input = common.input;
            let output = common.output;
            let key = common.key;

            println!("decrypt: {input:?} -> {output:?}, key={key:?}");
        }
    }
}

// #[derive(Parser)]
// #[command(version, about, author)]
// struct Cli {
//     #[arg(short = 'i', long = "input")]
//     input: String,
//     #[arg(short = 'o', long = "output")]
//     output: String,
// }
