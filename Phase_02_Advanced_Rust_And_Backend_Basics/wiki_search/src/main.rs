use std::env::args;
use reqwest::header
::{HeaderMap,HeaderValue,USER_AGENT};
use serde_json::Value;
#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>>
{
    let args:Vec<String>=args().collect();
    if args.len()<2
    {
        println!("Sorry but you didnt enter any search value!");
        return Ok(());
    }
    let search_value=&args[1];
    let client=reqwest::Client::new();
    let mut headers=HeaderMap::new();
    headers.insert(USER_AGENT,HeaderValue::from_static("Mozilla/5.0 (Rust-Bot/1.0)"));
    let search_url=format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",search_value);
    let search_resp:Value=client.get(&search_url).headers(headers.clone()).send().await?.json().await?;
    let search_list=&search_resp["query"]["search"];
    if let Some(first_value)=search_list.get(0)
    {
        let raw_value=first_value["title"].as_str().unwrap_or("");
        let correct_value=raw_value.replace(" ","_");
        let info_url=format!("https://en.wikipedia.org/api/rest_v1/page/summary/{}",correct_value);
        let info_resp:Value=client.get(&info_url).headers(headers.clone()).send().await?.json().await?;
        if let Some(data)=info_resp["extract"].as_str()
        {
            println!("\n\n{}\n\n",data);
        }
    }
    else
    {
        println!("Result not found!");
    }
    Ok(())
}