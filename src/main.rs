use plotters::prelude::*;
use std::time::Duration;

async fn bench(duration: Duration) -> u32 {
    let start = std::time::Instant::now();
    let mut requests = 0;
    while start.elapsed() < duration {
        let request = reqwest::get("http://127.0.0.1:8080/").await;
        if request.is_ok() {
            requests += 1;
        }
    }
    requests
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let threads_counts = vec![2, 4, 8, 16, 32];
    let duration = Duration::from_secs(10);

    let (min_threads, max_threads) = (2u32, 32);

    let mut response_times = Vec::new();

    for threads in threads_counts.clone() {
        let tasks = (0..threads).map(|_| tokio::spawn(bench(duration)));
        let awaited = futures::future::join_all(tasks).await;
        let total_requests_made: f64 = awaited.iter().flatten().sum::<u32>() as f64;

        let requests_per_second = total_requests_made / duration.as_secs_f64();

        println!(
            "Threads: {}, Requests: {}, Requests per second: {:?}",
            threads, total_requests_made, requests_per_second
        );
        response_times.push((threads, requests_per_second));
    }

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

    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
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

    Ok(())
}
