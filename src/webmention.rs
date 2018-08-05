use std::collections::HashMap;
use std::collections::hash_map::Iter;

#[derive(Serialize, Debug)]
pub struct Author {
    pub name : String,
    pub url : String,
    pub photo_url : String,
}

#[derive(Serialize, Debug)]
pub struct Webmention {
    pub author : Author,
    pub title : String,
    pub content : String,
    pub url : String,
    pub date : String,
    pub mention_type : WebmentionType,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug)]
pub enum WebmentionType {
    reply,
    like,
    repost,
    bookmark,
    mention,
    rsvp,
}

#[derive(Debug)]
pub struct WebmentionCollection {
    webmention_map : HashMap<String,Vec<Webmention>>,
}

impl WebmentionCollection {
    pub fn new() -> WebmentionCollection {
        WebmentionCollection {
            webmention_map : HashMap::new(),
        }
    }

    pub fn add(&mut self, path : String, webmention : Webmention) {
        self.webmention_map.entry(path).or_insert(Vec::new()).push(webmention)
    }

    pub fn iter(&self) -> Iter<String,Vec<Webmention>> {
        self.webmention_map.iter()
    }
}