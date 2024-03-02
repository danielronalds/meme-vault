use std::{
    fs::File,
    io::{copy, Write},
    path::Path, error::Error,
};

/// The URL of the API for meme fetching
const API_URL: &str = "https://meme-api.com/gimme/";

/// The possible subreddits to fetch from
const SUBREDDITS: [&str; 2] = ["ProgrammerHumor", "WholesomeMemes"];

/// The subreddit to get the meme from
#[derive(Clone, Copy)]
pub enum MemeSubreddit {
    ProgrammerHumor,
    WholesomeMemes,
}

impl ToString for MemeSubreddit {
    fn to_string(&self) -> String {
        SUBREDDITS[*self as usize].to_string()
    }
}

/// A meme that has been fetched from the API
#[derive(serde::Deserialize, Debug)]
pub struct Meme {
    title: String,
    url: String,
}

/// Gets a random meme
///
/// # Parameters
///
/// - `source` The subreddit to get the Meme from
///
/// # Returns
///
/// Either a [`reqwest::Error`] or a [`Meme`]
pub fn get_random_meme(source: MemeSubreddit) -> Result<Meme, reqwest::Error> {
    let url = format!("{}{}", API_URL, source.to_string());

    let response = reqwest::blocking::get(url)?;

    let meme: Meme = response.json()?;

    Ok(meme)
}

/// Downloads the given meme to the directory supplied
///
/// # Parameters
///
/// - `meme`     The [`Meme`] to download
/// - `dest_dir` The directory to download the meme to. **NOTE** do not append a / at the end of
/// to the end of the path, this is appended in the function
///
/// # Returns
///
/// An error if either the file fails to create, or if the GET request fails
pub fn download_meme(meme: &Meme, dest_dir: &str) -> Result<(), Box<dyn Error>> {
    // Removing spaces from the file name
    let formated_title = meme.title.replace(" ", "_");

    let path = format!("{}/{}.png", dest_dir, formated_title);

    let mut dest = File::create(&path)?;

    let response = reqwest::blocking::get(meme.url.as_str())?
        .copy_to(&mut dest);

    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err))
    }
}
