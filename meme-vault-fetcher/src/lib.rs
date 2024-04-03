use std::{error::Error, fs::File};

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
fn get_random_meme(source: MemeSubreddit) -> Result<Meme, reqwest::Error> {
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
fn download_meme(meme: &Meme, dest_dir: &str, dist_file: &str) -> Result<(), Box<dyn Error>> {
    // Removing spaces from the file name
    let formated_title = meme.title.replace(" ", "_");

    let path = format!("{}/{}", dest_dir, dist_file);

    let mut dest = File::create(&path)?;

    let response = reqwest::blocking::get(meme.url.as_str())?.copy_to(&mut dest);

    match response {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}

/// Downloads a random meme to the directory supplied
///
/// # Parameters
///
/// - `source` The subreddit to get the Meme from
/// - `dest_dir` The directory to download the meme to. **NOTE** do not append a / at the end of
/// to the end of the path, this is appended in the function
/// - `dest_file` The name of the file to write the meme to
///
/// # Returns
///
/// An error if either the file fails to create, or if the GET request fails. Otherwise the path to
/// the downloaded file
pub fn download_random_meme(
    source: MemeSubreddit,
    dest_dir: &str,
    dest_file: &str
) -> Result<String, Box<dyn Error>> {
    let meme = get_random_meme(source)?;

    download_meme(&meme, dest_dir, dest_file)?;

    Ok(format!("{}/{}", dest_dir, dest_file))
}
