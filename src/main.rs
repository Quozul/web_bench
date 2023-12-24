use plotters::prelude::*;
use std::ops::Add;
use std::time::Duration;

async fn bench(requests: u32) -> Duration {
    let mut sum = Duration::new(0, 0);

    for _ in 0..requests {
        let start = std::time::Instant::now();
        let _ = reqwest::get("http://localhost:8080/").await;
        sum = sum.add(start.elapsed());
    }

    sum
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let threads_counts = vec![1, 2, 4, 8, 16];
    let request_counts = vec![500, 1000, 2000, 4000];

    let (min_threads, max_threads) = (1u32, 16);

    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Average Response Time", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_threads..max_threads, 0..100u128)?;

    chart.configure_mesh().draw()?;

    for total_requests in request_counts {
        let mut response_times = Vec::new();

        for threads in threads_counts.clone() {
            let requests_per_thread = total_requests / threads;

            let tasks = (0..threads).map(|_| tokio::spawn(bench(requests_per_thread)));
            let awaited = futures::future::join_all(tasks).await;

            let mut sum = Duration::new(0, 0);

            for duration in awaited.into_iter().flatten() {
                sum = sum.add(duration);
            }

            let average_response_time = sum / total_requests;
            println!(
                "Threads: {}, Total Requests: {}, Average: {:?}",
                threads, total_requests, average_response_time
            );
            response_times.push((threads, average_response_time.as_millis()));
        }

        chart
            .draw_series(LineSeries::new(response_times, &RED))?
            .label(format!("{total_requests} requests"))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
