// use tokio::sync::mpsc;
use std::{ sync::mpsc, path, fs };

pub mod scanner;

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: [u8; 7]
}

#[tokio::main]
async fn main() {
    let save_image: bool = std::env::args().nth(1).unwrap_or("true".to_owned()).parse().unwrap();

    // If the directory './images' doesnt exist, create it.
    if save_image && !path::Path::new("./images").exists() {
        fs::create_dir("./images").unwrap();
    }

    let threads = 4;
    let (tx, rx) = mpsc::channel::<Task>();

    for _ in 0..threads {
        // Tokio oneshot
        let c = tx.clone();
        let save = save_image.clone();
        tokio::spawn(async move {
            scanner::scrape(c, save).await;
        });
    }

    loop {
        match rx.try_recv() {
            Ok(task) => {
                println!("{:?}", task.id);
            }
            Err(_) => {}
        }
    }
}
