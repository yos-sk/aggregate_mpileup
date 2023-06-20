use clap::{Parser, Subcommand};
use std::process;
mod aggregate;
mod parse;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(short, long)]
        input_file: String,
        
        #[arg(short, long)]
        output_file: String,

        #[arg(short, long)]
        mode: String,
    },
    
    Aggregate {
        #[arg(short, long)]
        input_file: String,

        #[arg(short, long)]
        whitelist_output_file: String,

        #[arg(short, long)]
        blacklist_output_file: String,

        #[arg(long, default_value = "10")]
        min_depth: String,

        #[arg(long, default_value = "0.05")]
        min_mismatch_ratio: String,

        #[arg(long, default_value = "5")]
        min_samples_for_blacklist: String,

        #[arg(long, default_value = "20")]
        min_samples_for_whitelist: String,
    },
}

fn main() {
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::Parse{input_file,
                        output_file,
                        mode} => {
            if let Err(error) = parse::run(input_file,
                                           output_file,
                                           mode) {
                eprintln!("{}", error);
                process::exit(1);
            }
        },
        Commands::Aggregate{input_file,
                            whitelist_output_file,
                            blacklist_output_file,
                            min_depth,
                            min_mismatch_ratio,
                            min_samples_for_blacklist,
                            min_samples_for_whitelist} => {
            if let Err(error) = aggregate::run(input_file, 
                                               whitelist_output_file,
                                               blacklist_output_file,
                                               min_depth,
                                               min_mismatch_ratio,
                                               min_samples_for_blacklist,
                                               min_samples_for_whitelist) {
                eprintln!("{}", error);
                process::exit(1);
            }
        },
    }
}