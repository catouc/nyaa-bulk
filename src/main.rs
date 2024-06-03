use std::error::Error;
use std::env;
use std::fs::File;
use std::io;
use select::document::Document;
use select::predicate::Name;
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let nyaa_base_url: Url = Url::parse("https://nyaa.si")?;

    let response = reqwest::blocking::get(&args[1])?
        .text()?;

    Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|l| l.starts_with("/download"))
        .for_each(|x| { 
            let url = nyaa_base_url.join(x).expect("invalid url given");
            let file_name = x.split("download/").nth(1).expect("file_name split broke");
            download_torrent(url, file_name).expect("downloads work");
            }
        );
    Ok(())
}

fn download_torrent(url: Url, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(url)?;
    let mut out = File::create(file_name)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}
