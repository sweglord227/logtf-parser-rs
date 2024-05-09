pub mod structs;

use crate::structs::Log;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use tokio::time::{sleep, Duration};
use log::*;
use futures::StreamExt;

const LOGURL: &str = "https://logs.tf/api/v1/log/";


#[tokio::main]
async fn main() {
    let _ = simple_logger::SimpleLogger::new().init();
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);
    if args.first().is_none() { error!("give args"); return; };

    let mut tasks = vec![vec![]];
    for arg in args {
        let lines = match read_lines(&arg) {
            Ok(r) => r,
            Err(e) => { warn!("Unable to open file [{arg}]. {e}"); continue; }
        };
        let (mut line_number, mut index) = (0, 0);
        for line in lines {
            line_number += 1;
            if line_number == 7 {
                line_number = 0;
                index += 1;
            }
            if index == tasks.len() { tasks.push(vec![]); }
            match line {
                Ok(log_id) => tasks[index].push(log_id),
                Err(e) => { warn!("{e}"); continue;
                }
            }
        }
    }

    if tasks.is_empty() { return; }

    let mut logs = vec![vec![]];
    for task in tasks {
        let _ = sleep(Duration::from_secs_f64(1.0)).await;
        logs.push(futures::stream::iter(task.iter())
            .map(|id| get_log(id.to_string()))
            .buffer_unordered(task.len())
            .collect()
            .await);
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

            println!("");
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
    return Ok(serde_json::from_str(&reqwest::get(LOGURL.to_owned() + &id).await?.text().await?)?);
}
