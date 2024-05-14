pub mod structs;

use crate::structs::{log::Log, args::*};
use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;
use futures::StreamExt;
use log::*;
use tokio::time::{sleep, Duration};

const LOGURL: &str = "https://logs.tf/api/v1/log";
/// logs.tf limits the amount of queries per second.
const LOGS_PER_QUERY: usize = 5;
const NEW_STRING: String = String::new();


#[tokio::main]
async fn main() {
    if let Err(e) = simple_logger::SimpleLogger::new().init() { println!("logger failed to init: {e}")}

    let mut tasks: Vec<[String; LOGS_PER_QUERY]> = vec![];
    let Some(args) = Cli::parse().command else { return; };
    match args {
        Commands::Parse { files } => {
            for file in files {
                let lines = match read_lines(&file) {
                    Ok(r) => r,
                    Err(e) => { warn!("Unable to open file [{file}]. {e}"); return; }
                };
                let (mut line_number, mut index) = (0, 0);
                for line in lines {
                    if line_number == LOGS_PER_QUERY {
                        line_number = 0;
                        index += 1;
                    }
                    if index == tasks.len() { 
                        tasks.push([NEW_STRING; LOGS_PER_QUERY]); 
                    }
                    match line {
                        Ok(log_id) => tasks[index][line_number] = log_id,
                        Err(e) => { warn!("{e}"); continue; }
                    }
                    line_number += 1;
                }
            }
        },
        Commands::Search { title, map, uploader, players, limit, offset } => {
            let mut search: String = "?".to_string();
            if let Some(title) = title { 
                if title.len() < 2 { error!("Title is less than 2 characters"); return; }
                search += &("title=".to_string() + &title.replace(" ", "+") + "&") 
            };
            if let Some(map) = map { search += &("map=".to_string() + &map + "&") };
            if let Some(uploader) = uploader { search += &("uploader=".to_string() + &uploader + "&") };
            if let Some(players) = players { 
                search += &"player=".to_string();
                for player in players { search += &(player + &",".to_string()); }
                search.pop();
                search += "&";
            };
            if let Some(limit) = limit { search += &("limit=".to_string() + &limit.to_string() + "&") };
            if let Some(offset) = offset { search += &("offset=".to_string() + &offset.to_string() + "&") };
            info!("{LOGURL}{search}");
        }
    }

    if tasks.is_empty() { return; }

    let mut logs = vec![vec![]];
    for task in tasks {
        let _ = sleep(Duration::from_secs_f64(1.0)).await;
        logs.push(futures::stream::iter(task.iter())
                  .map(|id| return get_log(id.to_string()))
                  .buffer_unordered(task.len())
                  .collect()
                  .await
                 );
    }

    for log in logs {
        for l in &log {
            let log: &Log = match l {
                Err(e) => { warn!("{e}"); continue; },
                Ok(r) => r,
            };
            // println!("");

            // for (id, player) in log.players.iter() {
            //     for class in player.class_stats.iter() {
            //         for weapon in class.weapon.iter() {
            //             let weapon = weapon.0;
            //             print!("{weapon}, ");
            //         }
            //     }
            // }

            for message in &log.chat {
                println!("{} :  {}", message.name, message.message);
            }
        }
    }
}

fn read_lines(filename: &String) -> io::Result<io::Lines<io::BufReader<File>>> {
    debug!("Reading file: {filename}");
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

async fn get_log(id: String) -> Result<Log, Box<dyn std::error::Error>> {
    debug!("Getting log: {id}");
    return Ok(serde_json::from_str(&reqwest::get(("/".to_string() + LOGURL).to_owned() + &id).await?.text().await?)?);
}
