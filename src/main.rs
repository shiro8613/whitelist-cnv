
use std::path::PathBuf;
use clap::Parser;
use tokio::fs;

use crate::{mojang::Mojang, options::Options, script::ScriptEngine};

mod mojang;
mod options;
mod script;

#[tokio::main]
async fn main() {
    let args = Options::parse();

    let input_file = args.input.parse::<PathBuf>().expect("input file error");

    let filter_file = args.filter.parse::<PathBuf>().expect("filter file error");

    let output_file = args.output.parse::<PathBuf>().expect("output file error");

    let mut script_engine = ScriptEngine::new(filter_file).expect("script error");

    let mut data = csv::Reader::from_path(input_file).expect("csv reader error");

    let mut entries :Vec<String> = Vec::new();

    for line in data.records() {
        let Ok(line) = line else {
            continue;
        };
        
        if let Some(entry) = script_engine.run_filter(line) {
            entries.push(entry);
        }

    }

    println!("loaded users: {}", entries.len());

    let mut api_service = Mojang::new(3);
    api_service.add(&entries);

    let whitelist = api_service.start_query().await;
    let json_content = serde_json::to_vec_pretty(&whitelist).expect("json error");

    fs::write(output_file, json_content).await.expect("output error");
    println!("create completed. users: {}", whitelist.len());
}
