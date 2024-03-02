use std::{
    fs::File,
    io::{copy, Write},
    path::Path,
};

const API_URL: &str = "https://meme-api.com/gimme/";

const SUBREDDITS: [&str; 2] = ["ProgrammerHumor", "WholesomeMemes"];

#[derive(Clone, Copy)]
pub enum MemeSource {
    ProgrammerHumor,
    WholesomeMemes,
}

impl ToString for MemeSource {
    fn to_string(&self) -> String {
        SUBREDDITS[*self as usize].to_string()
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Meme {
    title: String,
    url: String,
}

pub fn get_meme(source: MemeSource) -> Result<Meme, reqwest::Error> {
    let url = format!("{}{}", API_URL, source.to_string());

    let response = reqwest::blocking::get(url)?;

    let meme: Meme = response.json()?;

    Ok(meme)
}

pub fn download_meme(meme: &Meme, dest_dir: &str) -> Result<(), reqwest::Error> {
    let path = format!("{}/{}.png", dest_dir, meme.title);

    let mut dest = File::create(&path).expect("Failed to create file");

    let response = reqwest::blocking::get(meme.url.as_str())?
        .copy_to(&mut dest)
        .unwrap();

    Ok(())
}
