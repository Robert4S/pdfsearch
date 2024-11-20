use std::{
    collections::LinkedList,
    convert::identity,
    fs::{self, DirEntry},
    io,
    path::{self, PathBuf},
    result,
};

use clap::Parser;

#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = ".")]
    directory_path: String,
    #[arg(short, long)]
    search_term: String,
}

fn main() {
    let args = Config::parse();

    let matches = search_dir(args.directory_path.into(), &args.search_term).unwrap();

    for m in matches {
        println!("{m}")
    }
}

fn search_dir(dir: PathBuf, term: &str) -> Result<Vec<String>, io::Error> {
    let subdirs = fs::read_dir(dir)?;

    let mut dirs = vec![];
    let mut files = vec![];

    subdirs
        .into_iter()
        .filter_map(|x| x.ok())
        .for_each(|entry| {
            let meta = entry.metadata().unwrap();
            if meta.is_dir() {
                dirs.push(entry.path());
            } else {
                files.push(entry.path());
            }
        });

    let mut dir_matches: Vec<_> = dirs
        .into_iter()
        .filter_map(|dir| search_dir(dir, term).ok())
        .flatten()
        .collect();

    let mut file_matches: Vec<_> = files
        .into_iter()
        .filter_map(|file: PathBuf| fs::read_to_string(file).ok())
        .filter(|x| x.contains(&term))
        .collect();

    file_matches.append(&mut dir_matches);

    Ok(file_matches)
}
