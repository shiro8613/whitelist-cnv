use std::{collections::VecDeque, time::Duration};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use uuid::Uuid;

const MOJANG_API_URL :&str = "https://api.mojang.com/profiles/minecraft";

#[derive(Debug, Deserialize, Serialize)]
pub struct WhitelistEntry {
    name: String,
    #[serde(skip_deserializing)]
    uuid: String,
    #[serde(skip_serializing)]
    id: String
}

pub type WhitelistEntries = Vec<WhitelistEntry>;

pub struct Task<'a> {
    entries: &'a [String],
    retry_count: usize,
}

pub struct Mojang<'a> {
    client :Client,
    queue: VecDeque<Task<'a>>,
    max_retry: usize, 
}

impl<'a> Mojang<'a> {
    pub fn new(max_retry :usize) -> Self {
        Self {
            client: Client::new(),
            queue: VecDeque::new(),
            max_retry
        }
    }

    pub fn add(&mut self, entries :&'a Vec<String>) {
        let chunks = entries.chunks(10);
        for chunk in chunks {
            let task = Task::new(chunk);
            self.queue.push_back(task);
        }
    }

    pub async fn start_query(&mut self) -> WhitelistEntries {
        let mut completed :WhitelistEntries= Vec::with_capacity(self.queue.len());

        while let Some(mut task) = self.queue.pop_front() {
            println!("fetching: {:?}", task.entries);
            let res = self.client.post(MOJANG_API_URL)
                .json(&task.entries)
                .send()
                .await;
            if let Ok(res) = res {
                match res.json::<WhitelistEntries>().await {
                    Ok(mut entries) => {
                        println!("complete: {:?}", entries);
                        completed.append(&mut entries)
                    },
                    Err(_) => continue
                }
            } else {
                if task.retry_count <= self.max_retry {
                    println!("response error. retry {}/{}", task.retry_count, self.max_retry);
                    task.retry_count += 1;
                    self.queue.push_back(task);
                }
            }

            sleep(Duration::from_secs(1)).await;
        }

        let completed = completed.iter()
            .filter(|e| e.copy_uuid())
            .collect();

        completed

    }


}

impl<'a> Task<'a> {
    pub fn new(entries :&'a [String]) -> Self {
        Self { 
            entries, 
            retry_count: 0
        }
    }
}

impl WhitelistEntry {
    pub fn copy_uuid(mut self) -> bool {
        match Uuid::parse_str(&self.id) {
            Ok(uuid) => {           
                self.uuid = uuid.to_string();         
                true
            },
            Err(_) => false
        }
    }
}