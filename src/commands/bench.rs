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

pub(crate) async fn stress(
    hostname: String,
    threads: Vec<u32>,
    duration: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let threads_counts = threads;
    let duration = Duration::from_secs_f64(duration);

    let mut response_times = Vec::new();

    for threads in threads_counts.clone() {
        let tasks = (0..threads).map(|_| tokio::spawn(bench(hostname.clone(), duration)));

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

    Ok(())
}
