use std::{collections::VecDeque, time::Duration};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use uuid::Uuid;

const CHUNK_SIZE :usize = 10;
const MOJANG_API_URL :&str = "https://api.mojang.com/profiles/minecraft";

#[derive(Debug, Deserialize, Serialize)]
pub struct WhitelistEntry {
    name: String,
    #[serde(skip_serializing)]
    id: String,
    #[serde(skip_deserializing)]
    uuid: String,
}

pub type WhitelistEntries = Vec<WhitelistEntry>;

pub struct Task<'a> {
    entries: &'a [String],
    retry_count: usize,
}

pub struct Mojang<'a> {
    client: Client,
    queue: VecDeque<Task<'a>>,
    max_retry: usize,
}

impl<'a> Mojang<'a> {
    pub fn new(max_retry: usize) -> Self {
        Self {
            client: Client::new(),
            queue: VecDeque::new(),
            max_retry,
        }
    }

    pub fn add(&mut self, entries: &'a [String]) {
        let chunks = entries.chunks(CHUNK_SIZE);
        for chunk in chunks {
            let task = Task::new(chunk);
            self.queue.push_back(task);
        }
    }

    pub async fn start_query(&mut self) -> WhitelistEntries {
        let mut completed: WhitelistEntries = Vec::with_capacity(self.queue.len().saturating_mul(CHUNK_SIZE));
        while let Some(mut task) = self.queue.pop_front() {
            println!("fetching: {:?}", task.entries);
            let res = self
                .client
                .post(MOJANG_API_URL)
                .json(&task.entries)
                .send()
                .await;
            if let Ok(res) = res {
                if res.status() != 200 && task.retry_count < self.max_retry {
                    task.retry_count += 1;
                    println!(
                        "rate_limited. retry {}/{}",
                        task.retry_count, self.max_retry
                    );

                    self.queue.push_back(task);
                    continue;
                }

                match res.json::<WhitelistEntries>().await {
                    Ok(mut entries) => {
                        println!("fetched: {} users", entries.len());
                        completed.append(&mut entries)
                    }
                    Err(_) => continue,
                }
            }

            sleep(Duration::from_secs(1)).await;
        }

        completed.retain_mut(WhitelistEntry::check_and_set);
        completed
    }
}

impl<'a> Task<'a> {
    pub fn new(entries: &'a [String]) -> Self {
        Self {
            entries,
            retry_count: 0,
        }
    }
}

impl WhitelistEntry {
    pub fn check_and_set(&mut self) -> bool {
        match Uuid::parse_str(&self.id) {
            Ok(uuid) => {
                self.uuid = uuid.to_string();
                true
            }
            Err(_) => false,
        }
    }
}
