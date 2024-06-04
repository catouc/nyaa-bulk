use std::fs::File;
use std::io;
use url::Url;


static BASE_URL_STR: &str = "https://nyaa.si";

pub fn is_valid_torrent_url(url: &str) -> bool {
    const TORRENT_URL_PREFIX: &str = "/download";
    url.starts_with(TORRENT_URL_PREFIX)
}

pub fn download_torrent(relative_link: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = Url::parse(BASE_URL_STR)?;
    let url = base_url.join(relative_link)?;

    let body = ureq::get(url.as_str())
        .call()?;

    let file_name = construct_file_name(relative_link)?;
    let mut out = File::create(file_name)?;
    io::copy(&mut body.into_reader(), &mut out)?;
    Ok(())
}

fn construct_file_name(relative_link: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let file_name = match relative_link.split("download/").nth(1) {
        Some(file_name) => file_name,
        None => return Err("expected 2 elements in link".into()),
    };

    if !file_name.ends_with(".torrent") {
        return Err("expected torrent file".into())
    }

    Ok(file_name)
}

