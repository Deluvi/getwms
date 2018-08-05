extern crate reqwest;
extern crate serde_json;
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod webmention;
mod webmention_dot_io;

use clap::{Arg,App};
use reqwest::Url;

use webmention::{Webmention,WebmentionCollection};

use std::collections::HashMap;
use std::fs::{write,create_dir_all};
use std::path::Path;

fn write_json(folder : &str, collection_webmention : &WebmentionCollection) {
    for (path, webmentions_vec) in collection_webmention.iter() {
        let path_file_json = "./".to_owned() + folder + (path.trim_right_matches('/')) + ".json";
        let path_folder = path_file_json.trim_right_matches(Path::new(&path_file_json).file_name().unwrap().to_str().unwrap());
        create_dir_all(&path_folder).unwrap();
        write(&path_file_json,serde_json::to_string(webmentions_vec).unwrap().into_bytes()).unwrap();
    }
}

fn get_webmentions(url : &str) -> Result<WebmentionCollection,String> {
    let url = Url::parse(url).unwrap();
    if let Some(domain) = url.domain() {
        match domain {
            "webmention.io" => webmention_dot_io::query_backend(&url),
            _ => Err("Webmention backend domain unknown: ".to_string() + domain),
        }
    }
    else {
        Err("The URL is invalid".to_string())
    }
}

fn main() {
    let matches = App::new("Get Webmentions")
        .version("0.1")
        .author("Deluvi")
        .about("Pulls webmentions from a webmention backend and store them for further use.")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help("Sets the URL where the webmentions are stored")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("ofolder")
            .short("o")
            .long("outputfolder")
            .value_name("FOLDER")
            .help("Sets the folder where the webmentions are stored")
            .takes_value(true))
        .get_matches();

    match get_webmentions(matches.value_of("url").unwrap()) { //Safe unwrap since url is defined as required.
        Ok(webmentions) => {
            write_json(matches.value_of("ofolder").unwrap_or("data").trim_right_matches('/'),&webmentions)
        },
        Err(err_string) => println!("Error! {}", err_string),
    };
}
