pub mod structs;

use structs::{log::Log, search::SearchResponse, args::*, stats::Stats};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;
use futures::StreamExt;
use indicatif::*;
use log::*;
use reqwest::{Client, RequestBuilder};
use steamid_ng::*;
use tokio::time::{sleep, Duration};

/// logs.tf limits the amount of queries per second.
const LOG_URL: &str = "https://logs.tf/api/v1/log";
const LOGS_PER_QUERY: usize = 5;
const LAST_SUPPORTED_LOG: u32 = 151744;
const NEW_STRING: String = String::new();


#[tokio::main]
async fn main() {
    // if let Err(e) = simple_logger::SimpleLogger::new()
    //     .with_level(LevelFilter::Info)
    //     // .with_level(LevelFilter::Trace)
    //     .init() { println!("logger failed to init: {e}")}

    let Some(args) = Cli::parse().command else { return; };
    let mut tasks: Vec<[String; LOGS_PER_QUERY]> = vec![];
    let mut steamids: Vec<SteamID> = vec![];
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
                    if index == tasks.len() { tasks.push([NEW_STRING; LOGS_PER_QUERY]); }
                    match line {
                        Ok(log_id) => tasks[index][line_number] = log_id,
                        Err(e) => { warn!("{e}"); continue; }
                    }
                    line_number += 1;
                }
            }
        },
        Commands::Search { title, map, uploader, players, limit, offset } => {
            let mut client = Client::new().request(reqwest::Method::GET, LOG_URL.to_string());
            if let Some(title) = title { 
                if title.len() < 2 { error!("Title is less than 2 characters"); return; }
                client = client.query(&[("title", title.clone())]);
            };
            if let Some(map) = map { client = client.query(&("map", map.clone())) };
            if let Some(uploader) = uploader { client = client.query(&[("uploader", uploader.clone())]) };
            if let Some(players) = players { 
                let mut search = String::new();
                for player in players { 
                    steamids.push(SteamID::from(player));
                    search += &(player.to_string() + ","); 
                }
                search.pop();
                client = client.query(&[("player", search)]);
            };
            if let Some(limit) = limit { client = client.query(&[("limit", limit.clone())]) };
            if let Some(offset) = offset { client = client.query(&[("offset", offset.clone())]) };

            let response = match get_search(client).await {
                Err(e) => { warn!("{e}"); return; },
                Ok(r) => r,
            };
            info!("found {} results", response.results);
            let (mut line_number, mut index) = (0, 0);
            for log in response.logs {
                if line_number == LOGS_PER_QUERY {
                    line_number = 0;
                    index += 1;
                }
                if index == tasks.len() { tasks.push([NEW_STRING; LOGS_PER_QUERY]); }
                tasks[index][line_number] = log.id.to_string();
                line_number += 1;
            }
        }
    }

    let bar = ProgressBar::new(tasks.len() as u64);

    if tasks.is_empty() { return; }

    let mut logs = vec![vec![]];
    'mark: for task in tasks.iter() {
        let _ = sleep(Duration::from_secs_f64(1.0)).await;
        for log in task.iter() {
            if log.is_empty() { continue; }
            if log.trim().parse::<u32>().expect("id should only have numbers") < LAST_SUPPORTED_LOG { 
                error!("Logs before ID {} are unsupported.", LAST_SUPPORTED_LOG);
                break 'mark;
            }
        }
        logs.push(futures::stream::iter(task.iter())
            .map(|id| {
                info!("getting log from ID: {id}");
                let request = Client::new() .request(reqwest::Method::GET, LOG_URL.to_string() + "/" + id);
                return get_log(request);
            })
            .buffer_unordered(task.len())
            .collect()
            .await
        );
        bar.inc(1);
    }

    bar.finish_and_clear();

    let mut stats: HashMap<SteamID, Stats> = HashMap::new();
    for steamid in steamids.iter() {
        stats.insert(*steamid, Stats::default());
    }
    for log in logs.iter() {
        for log in log {
            let log: &Log = match log {
                Err(e) => { warn!("{e}"); continue; },
                Ok(r) => r,
            };
            // for (id, player) in log.players.iter() {
            //     for class in player.class_stats.iter() {
            //         for weapon in class.weapon.iter() {
            //             let weapon = weapon.0;
            //             print!("{weapon}, ");
            //         }
            //     }
            // }

            for steamid in steamids.iter() {
                match log.players.get(&steamid.steam3()) {
                    Some(player) => stats.entry(*steamid)
                        .and_modify(|value| value.kills += player.kills),
                    None => continue,
                };
                match log.names.get(&steamid.steam3()) {
                    Some(name) => stats.entry(*steamid)
                        .and_modify(|stats| stats.name.push(name.to_string()) ),
                    None => continue,
                };
            }

            // for message in &log.chat {
            //     println!("{} :  {}", message.name, message.message);
            // }
            // println!();
        }
    }
    for (_id, stat) in stats.iter_mut() {
        stat.name.sort();
        stat.name.dedup();
        let name = stat.name.last().expect("should have name").clone();
        stat.name.remove(0);
        // let mut aliases = String::new();
        // for name in stat.1.name.iter() { aliases += &(name.to_owned() + ", "); }
        // if !aliases.is_empty() { name += &(" AKA: ".to_owned() + &aliases) }
        println!("{} got {} kills over {} logs", name, stat.kills, (logs.len() - 1) * LOGS_PER_QUERY);
    }
}

fn read_lines(filename: &String) -> io::Result<io::Lines<io::BufReader<File>>> {
    debug!("Reading file: {filename}");
    let file = File::open(filename)?;
    trace!("{filename}");
    return Ok(io::BufReader::new(file).lines());
}

async fn get_log(request: RequestBuilder) -> Result<Log, Box<dyn std::error::Error>> {
    trace!("{request:?}");
    return Ok(request.send().await?.json::<Log>().await?);
}

async fn get_search(request: RequestBuilder) -> Result<SearchResponse, Box<dyn std::error::Error>> {
    trace!("{request:?}");
    return Ok(request.send().await?.json::<SearchResponse>().await?);
}
