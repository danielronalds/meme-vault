use std::{path::Path, fs};

use meme_vault_fetcher::*;

#[test]
pub fn downloading_random_meme_works() {
    let dest_dir = ".";
    let source = MemeSubreddit::WholesomeMemes;

    let meme_name = download_random_meme(source, dest_dir).expect("Failed to download meme");

    let meme_path = format!("./{}.png", meme_name.replace(" ", "_"));

    assert!(Path::new(&meme_path).exists());

    fs::remove_file(meme_path).expect("Failed to delete file");
}
