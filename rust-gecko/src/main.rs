use async_std::task;
use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

macro_rules! hashmap {
    ($($key: expr => $val: expr), *) => {{
        let mut map = HashMap::new();
        $(map.insert($key,$val);)*
            map
    }}
}

// parsing optional params, looks like : k1=v1:k2=v2:k3=v3
fn parse_optional_params(params: &str) -> Result<HashMap<&str, &str>, Error> {
    let list: Vec<&str> = params.split(':').collect();
    let mut params = HashMap::new();
    for item in list {
        let pair: Vec<&str> = item.split('=').collect();
        if pair.len() != 2 {
            return Err(Error::new(ErrorKind::Other, "Invalid parameter"));
        }
        params.insert(pair[0], pair[1]);
    }
    Ok(params)
}

fn request(url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    task::block_on(async {
        let res = surf::get(url).recv_string().await?;
        println!("{}", res);
        Ok(())
    })
}

// something looks like: api_url?k1=v1&k2=v2
fn fill_url_params(api_url: &String, params: &HashMap<&str, &str>) -> String {
    let mut url = api_url.clone();
    url.push('?');
    for (k, v) in params {
        url.push_str(&format!("{}={}&", k, v));
    }
    let _ = url.pop();
    println!("api_url = {}", url);
    url
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
        .subcommand(
            SubCommand::with_name("coins")
                .help("Get token information. Format of params: key-value pairs separated by colon. \
                        The CoinGecko API required keys are 'id'.  Detail info at: https://www.coingecko.com/api/documentations/v3#/coins. \
                        For example:  -i bitcoin -o localization=true:tickers=false:market_data=false:community_data=false")
                .arg(Arg::with_name("id")
                       .short("i")
                       .takes_value(true)
                       .required(true)
                       .help("token symbol, e.g. -i btc")
                )
                .arg(Arg::with_name("option")
                       .short("o")
                       .takes_value(true)
                       .help("optional parameters, e.g. -o localization=true:tickers=false:market_data=false:community_data=false")
                )
        )
        .subcommand(
            SubCommand::with_name("simple")
                 .help("Get token price, marketcap etc. Format of params: key-value pairs separated by colon. \
                        The CoinGecko API required keys are 'ids' and 'vs_currencies'. Detail info at: https://www.coingecko.com/api/documentations/v3#/simple. \
                        For example:  -i bitcoin,ethereum -v usd -o include_market_cap=true:include_24hr_vol=true")
                 .arg(Arg::with_name("ids")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .help("token list, comma separated. e.g. -i bitcoin,ethereum")
                 )
                 .arg(Arg::with_name("vs_currencies")
                        .short("v")
                        .takes_value(true)
                        .required(true)
                        .help("vs currencies, comma separated. e.g. -v usd")
                 )
                 .arg(Arg::with_name("option")
                       .short("o")
                       .takes_value(true)
                       .help("optional parameters, e.g. -o include_market_cap=true")
                )

        )
        .get_matches();

    let api_base_url = "https://api.coingecko.com/api/v3/";
    if matches.is_present("ping") {
        let url = format!("{}{}", api_base_url, "ping");
        let _ = request(url);
    }

    if let Some(matches) = matches.subcommand_matches("simple") {
        let ids = matches.value_of("ids").unwrap();
        let vs_currencies = matches.value_of("vs_currencies").unwrap();
        let mut params = hashmap!["include_market_cap"=>"true"];
        params.insert("ids", &ids);
        params.insert("vs_currencies", &vs_currencies);

        if let Some(pp) = matches.value_of("option") {
            match parse_optional_params(pp) {
                Ok(pp) => {
                    for (k, v) in pp {
                        params.insert(k, v);
                    }
                }
                Err(e) => {
                    println!("parse_optional_params error = {}", e);
                    return;
                }
            }
        }

        let api_url = format!("{}simple/price", api_base_url);
        let url = fill_url_params(&api_url, &params);
        let _ = request(url);
    }

    if let Some(matches) = matches.subcommand_matches("coins") {
        let id = matches.value_of("id").unwrap();
        let mut params = hashmap!["localization"=>"false","tickers"=>"false","market_data"=>"false","community_data"=>"false"];

        if let Some(pp) = matches.value_of("option") {
            match parse_optional_params(pp) {
                Ok(pp) => {
                    for (k, v) in pp {
                        params.insert(k, v);
                    }
                }
                Err(e) => {
                    println!("parse_optional_params error = {}", e);
                    return;
                }
            }
        }

        let api_url = format!("{}coins/{}", api_base_url, id);
        let url = fill_url_params(&api_url, &params);
        let _ = request(url);
    }
}
