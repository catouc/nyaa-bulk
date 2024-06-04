use std::error::Error;
use std::env;
use select::document::Document;
use select::predicate::Name;

pub mod nyaa_torrent;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let response: String = ureq::get(&args[1])
        .call()?
        .into_string()?;

    Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|l| nyaa_torrent::is_valid_torrent_url(l))
        .for_each(|x| {
            nyaa_torrent::download_torrent(x).expect("downloads work");
            }
        );
    Ok(())
}


