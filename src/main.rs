use std::error::Error;
use std::env;
use select::document::Document;
use select::predicate::Name;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    const NYAA: &str = "https://nyaa.si";

    let response = reqwest::blocking::get(&args[1])?
        .text()?;

    Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|l| l.starts_with("/download"))
        .for_each(|x| download_torrent(NYAA.to_string().push_str(x)?, x));
    Ok(())
}

fn download_torrent(url: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(url)?;
    let mut out = File::create(file_name)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}
