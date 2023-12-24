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

async fn request(threads: u32, total_requests: u32) {
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
}

#[tokio::main]
async fn main() {
    for requests in 7..10 {
        for threads in 0..5 {
            request(2_u32.pow(threads), 2_u32.pow(requests)).await;
        }
    }
}
