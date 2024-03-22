mod commands;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Stress {
        /// The hostname to benchmark
        #[arg(name = "hostname", long)]
        hostname: String,

        /// The number of threads to use (example: 1 2 4 8)
        /// Make sure to not use more threads than your CPU has cores
        #[arg(name = "threads", short, long, default_value = "1", num_args = 1.., value_delimiter = ',')]
        threads: Vec<u32>,

        /// The duration of the benchmark in seconds
        #[arg(name = "duration", short, long, default_value = "10")]
        duration: f64,

        /// The size of the body in MiB
        /// If the value is 0 or not provided, no request body will be sent and GET requests will be made
        #[arg(name = "body", short, long, default_value = "0")]
        body_size: usize,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        None => {}
        Some(Commands::Stress {
            hostname,
            threads,
            duration,
            body_size,
        }) => commands::bench::stress(hostname, threads, duration, body_size).await?,
    }

    Ok(())
}
