use rand::{distributions::Alphanumeric, Rng};
// use tokio::sync::mpsc::Sender;
use std::sync::mpsc::Sender;
use reqwest::{ StatusCode, Client };

use crate::Task;

pub async fn scrape(tx: Sender<Task>) {
    // let rng = rand::thread_rng();
    let client = Client::new();

    loop {
        let sample = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .collect::<Vec<u8>>();

        let id = String::from_utf8(sample.clone()).unwrap();
        
        let r = client.get("https://i.imgur.com/".to_owned() + &id)
            .send()
            .await
            .unwrap();

        match r.status() {
            StatusCode::OK => {
                tx.send(Task {
                    id: sample.try_into().unwrap()
                }).unwrap();
            }
            _ => {}
        }
    }
}