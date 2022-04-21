use rand::{distributions::Alphanumeric, Rng};
use tokio::io::AsyncWriteExt;
// use tokio::sync::mpsc::Sender;
use std::{sync::mpsc::Sender, io::Write};
use reqwest::{ StatusCode, Client, redirect::Policy };

use crate::Task;

pub async fn scrape(tx: Sender<Task>, save: bool) {
    // let rng = rand::thread_rng();
    let client = Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();

    loop {
        let sample = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .collect::<Vec<u8>>();

        let id = String::from_utf8(sample.clone()).unwrap();
        
        let r = client.get("https://i.imgur.com/".to_owned() + &id + ".png")
            .send()
            .await
            .unwrap();

        match r.status() {
            StatusCode::OK => {
                if save {
                    let bytes = r.bytes().await.unwrap();
                    let mut f = tokio::fs::File::create("./images/".to_owned() + &id + ".png")
                        .await
                        .unwrap();
                    f.write_all(&bytes).await.unwrap();
                }

                tx.send(Task {
                    id: sample.try_into().unwrap()
                }).unwrap();
            }
            _ => {}
        }
    }
}

pub async fn save() {

}