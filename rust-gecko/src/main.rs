pub mod cmd_error;

use anyhow::{Error, Result};
use async_std::task;
use clap::{App, Arg};
use cmd_error::CmdError;
use std::collections::HashMap;

fn request(url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    task::block_on(async {
        let res = surf::get(url).recv_string().await?;
        println!("{}", res);
        Ok(())
    })
}

// something looks like: api_url?k1=v1&k2=v2
fn fill_url_params(api_url: &String, params: &HashMap<String, String>) -> String {
    let mut url = api_url.clone();
    url.push('?');
    for (k, v) in params {
        url.push_str(&format!("{}={}&", k, v));
    }
    let _ = url.pop();
    println!("api_url = {}", url);
    url
}

// Coins command params looks like: k1=v1:k2=v2:k3=v3
// required keys: id
fn parse_coins_cmd(params: &String) -> Result<HashMap<String, String>, Error> {
    let list: Vec<&str> = params.split(':').collect();

    let mut params = HashMap::new();
    for item in list {
        let pair: Vec<&str> = item.split('=').collect();
        if pair.len() != 2 {
            return Err(CmdError::CoinsCmdError.into());
        }
        params.insert(pair[0].to_string(), pair[1].to_string());
    }
    if !params.contains_key("id") {
        return Err(CmdError::CoinsCmdError.into());
    }
    Ok(params)
}

// simple command params looks like: k1=v1:k2=v2:k3=v3
// required keys: ids and vs_currencies
// here we will use the same spec (comma separated symbols) of the value of ids and vs_currencies
// so that we don't need to treat value specifically
fn parse_simple_cmd(params: &String) -> Result<HashMap<String, String>, Error> {
    let list: Vec<&str> = params.split(':').collect();
    if list.len() <= 1 {
        return Err(CmdError::SimpleCmdError.into());
    }

    let mut params = HashMap::new();
    for item in list {
        let pair: Vec<&str> = item.split('=').collect();
        if pair.len() != 2 {
            return Err(CmdError::SimpleCmdError.into());
        }
        params.insert(pair[0].to_string(), pair[1].to_string());
    }
    if !params.contains_key("ids") || !params.contains_key("vs_currencies") {
        return Err(CmdError::SimpleCmdError.into());
    }
    Ok(params)
}

fn main() {
    let matches = App::new("Token Information Quote")
        .version("v3")
        .about("Based on the CoinGecko API: https://www.coingecko.com/api/documentations/v3")
        .arg(
            Arg::with_name("ping")
                .short("p")
                .long("ping")
                .help("check api server status"),
        )
        .arg(
            Arg::with_name("coins")
                .short("c")
                .long("coins")
                .help("Get token information. Format of params: key-value pairs separated by colon. \
                        The CoinGecko API required keys are 'id'.  Detail info at: https://www.coingecko.com/api/documentations/v3#/coins. \
                        For example:  id=bitcoin:localization=false:tickers=false:market_data=false:community_data=false")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("simple")
                 .short("s")
                 .long("simple")
                 .help("Get token price, marketcap etc. Format of params: key-value pairs separated by colon. \
                        The CoinGecko API required keys are 'ids' and 'vs_currencies'. Detail info at: https://www.coingecko.com/api/documentations/v3#/simple. \
                        For example:  ids=bitcoin,ethereum:vs_currencies=usd:include_market_cap=true")
                 .takes_value(true),
        )
        .get_matches();

    let api_base_url = "https://api.coingecko.com/api/v3/";
    if matches.is_present("ping") {
        let url = format!("{}{}", api_base_url, "ping");
        let _ = request(url);
    }

    if let Some(params) = matches.value_of("simple") {
        match parse_simple_cmd(&params.to_string()) {
            Ok(params) => {
                let api_url = format!("{}simple/price", api_base_url);
                let url = fill_url_params(&api_url, &params);
                let _ = request(url);
            }
            Err(e) => {
                println!("parse_simple_cmd error = {}", e);
                return;
            }
        }
    }

    if let Some(params) = matches.value_of("coins") {
        match parse_coins_cmd(&params.to_string()) {
            Ok(mut params) => {
                let coin = params.remove("id").unwrap(); // id exists because we have checked
                let api_url = format!("{}coins/{}", api_base_url, coin);
                let url = fill_url_params(&api_url, &params);
                let _ = request(url);
            }
            Err(e) => {
                println!("parse_coins_cmd error = {}", e);
                return;
            }
        }
    }
}
