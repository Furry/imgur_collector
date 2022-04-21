// use tokio::sync::mpsc;
use std::sync::mpsc;

pub mod scanner;

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: [u8; 7]
}

#[tokio::main]
async fn main() {
    let threads = 4;
    let (tx, rx) = mpsc::channel::<Task>();

    for _ in 0..threads {
        // Tokio oneshot
        let c = tx.clone();
        tokio::spawn(async move {
            scanner::scrape(c).await;
        });
    }

    loop {
        match rx.try_recv() {
            Ok(task) => {
                println!("{:?}", task);
            }
            Err(_) => {}
        }
    }
}
