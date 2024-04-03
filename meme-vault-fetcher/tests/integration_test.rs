use std::{path::Path, fs};

use meme_vault_fetcher::*;

#[test]
pub fn downloading_random_meme_works() {
    let dest_dir = ".";
    let source = MemeSubreddit::WholesomeMemes;

    let dest_file = "test.png";

    let meme_name = download_random_meme(source, dest_dir, dest_file).expect("Failed to download meme");

    let meme_path = format!("./{}", dest_file);

    assert!(Path::new(&meme_path).exists());

    fs::remove_file(meme_path).expect("Failed to delete file");
}
