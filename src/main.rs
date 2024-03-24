mod commands;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    /// Makes a lot of requests
    Requests {
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

        /// The size of the body in KiB
        /// If the value is 0 or not provided, no request body will be sent and GET requests will be made
        #[arg(name = "body", short, long, default_value = "0")]
        body_size: usize,
    },

    /// Opens a lot of long-lasting connections
    Connections {
        /// The hostname to benchmark
        #[arg(name = "hostname", long)]
        hostname: String,

        #[arg(name = "connection_count", short, long)]
        connection_count: u32,
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
        Some(Commands::Requests {
            hostname,
            threads,
            duration,
            body_size,
        }) => commands::bench::stress(hostname, threads, duration, body_size).await?,
        Some(Commands::Connections {
            hostname,
            connection_count,
        }) => commands::bench::connections(hostname, connection_count).await,
    }

    Ok(())
}
