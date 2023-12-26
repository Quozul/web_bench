use clap::Parser;
use plotters::prelude::*;
use reqwest::header::HeaderMap;
use std::time::Duration;

async fn bench(hostname: String, duration: Duration) -> u32 {
    let start = std::time::Instant::now();
    let mut requests = 0;
    let mut default_headers = HeaderMap::new();
    default_headers.insert("Connection", "keep-alive".parse().unwrap());

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .http1_only()
        .default_headers(default_headers)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();

    let hostname = hostname.clone();
    let hostname_str = hostname.as_str();

    while start.elapsed() < duration {
        let request = client.get(hostname_str).send().await;
        if request.is_ok() {
            requests += 1;
        }
    }
    requests
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// The hostname to benchmark
    #[arg(name = "hostname", short, long)]
    hostname: String,

    /// The number of threads to use (example: 1 2 4 8)
    /// Make sure to not use more threads than your CPU has cores
    #[arg(name = "threads", short, long, default_value = "1", num_args = 1.., value_delimiter = ',')]
    threads: Vec<u32>,

    /// The duration of the benchmark in seconds
    #[arg(name = "duration", short, long, default_value = "10")]
    duration: f64,

    /// If provided, a plot graph will be generated in the given file
    #[arg(name = "plot", short, long)]
    plot: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let threads_counts = cli.threads;
    let duration = Duration::from_secs_f64(cli.duration);

    let min_threads = threads_counts.iter().min().unwrap().clone();
    let max_threads = threads_counts.iter().max().unwrap().clone();

    let mut response_times = Vec::new();

    for threads in threads_counts.clone() {
        let tasks = (0..threads).map(|_| tokio::spawn(bench(cli.hostname.clone(), duration)));

        let start = std::time::Instant::now();
        let awaited = futures::future::join_all(tasks).await;
        let elapsed = start.elapsed();
        let total_requests_made: f64 = awaited.iter().flatten().sum::<u32>() as f64;

        let requests_per_second = total_requests_made / elapsed.as_secs_f64();

        println!(
            "{} requests made in {:?} on {} thread(s). Requests per second: {:.2}",
            total_requests_made, elapsed, threads, requests_per_second
        );
        response_times.push((threads, requests_per_second));
    }

    if let Some(file_name) = cli.plot {
        let max_requests_per_second = response_times
            .iter()
            .map(|(_, requests_per_second)| *requests_per_second)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let min_requests_per_second = response_times
            .iter()
            .map(|(_, requests_per_second)| *requests_per_second)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let root = BitMapBackend::new(file_name.as_str(), (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Requests per second", ("sans-serif", 24).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                min_threads..max_threads,
                (min_requests_per_second - 10.0)..(max_requests_per_second + 10.0),
            )?;

        chart.configure_mesh().draw()?;

        chart.draw_series(LineSeries::new(response_times, &RED))?;

        chart
            .configure_series_labels()
            .background_style(WHITE.mix(0.8))
            .border_style(BLACK)
            .draw()?;

        root.present()?;
    }

    Ok(())
}
