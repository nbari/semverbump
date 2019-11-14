use clap::{App, Arg};
use git2::Repository;
use regex::Regex;
use std::{collections::BTreeSet, env, process};

const SEMVER_RX: &str = r"(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)";

fn main() {
    // cli options default to patch
    let matches = App::new("bump")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("version")
                .required(true)
                .takes_value(false)
                .default_value("patch")
                .possible_value("major")
                .possible_value("minor")
                .possible_value("patch"),
        )
        .get_matches();

    let _version = matches.value_of("version").unwrap();

    let repo = match env::current_dir() {
        Ok(path) => match Repository::discover(path) {
            Ok(repo) => repo,
            _ => {
                eprintln!("not in a git repository");
                process::exit(1);
            }
        },
        _ => {
            eprintln!("could not get current_dir");
            process::exit(1);
        }
    };

    let tags = get_tags(repo);
    println!("{:#?}", tags);
}

fn get_tags(repo: Repository) -> Result<BTreeSet<String>, git2::Error> {
    let mut tags = BTreeSet::new();
    let re = Regex::new(SEMVER_RX).unwrap();
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;

    for name in repo.tag_names(None)?.iter() {
        if let Some(tag) = name {
            if let Some(cap) = re.captures(tag) {
                if &cap["major"].parse::<usize>().unwrap() > &major {
                    major = cap["major"].parse::<usize>().unwrap();
                }
                //let major, minor, patch  = (&cap["major"], &cap["minor"], &cap["patch"]);
                tags.insert(tag.to_string());
            }
        }
    }
    Ok(tags)
}
