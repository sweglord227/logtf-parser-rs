pub mod structs;

use crate::structs::Log;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const LOGURL: &str = "https://logs.tf/json/";


#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    // for arg in args {
    let Some(arg) = args.first() else { println!("give args"); return; };
    let logs = read_lines(arg);
    let logs = match logs {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };
    let logs = logs.flatten()
        .into_iter()
        .map(|log| {
            tokio::spawn(async move {
                return get_log(log);
            })
        });
    // }

    for log in logs {
        let log: Log = match log.await {
            Ok(r) => match r.await {
                Ok(r) => r,
                Err(e) => panic!("{}", e),
            },
            Err(e) => panic!("{}", e),
        };

        match log.players.get("[U:1:90026832]") {
                Some(p) => println!("{}", p.deaths),
                None => println!("ballin"),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

async fn get_log(id: String) -> Result<Log, Box<dyn std::error::Error>> {
    return Ok(serde_json::from_str(&reqwest::get(LOGURL.to_owned() + &id).await?.text().await?)?);
}
