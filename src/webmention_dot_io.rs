use reqwest::{get,Url};

use serde_json::Value;
use webmention::{Webmention,Author,WebmentionType,WebmentionCollection};
use std::collections::HashMap;

fn value_to_string(value : &Value) -> String {
        match value {
                Value::String(string) => string.to_string(),
                _ => "".to_string(),
        }
}

fn convert_json_to_webmention(json : &Value) -> Webmention {
        Webmention {
                author : Author {
                        name : value_to_string(&json["data"]["author"]["name"]),
                        url : value_to_string(&json["data"]["author"]["url"]),
                        photo_url : value_to_string(&json["data"]["author"]["photo"]),
                },
                title : value_to_string(&json["data"]["name"]),
                content : value_to_string(&json["data"]["content"]),
                url : value_to_string(&json["data"]["url"]),
                date : if &json["data"]["published"] == &Value::Null {
                                value_to_string(&json["verified_date"])
                        }
                        else {
                                value_to_string(&json["data"]["published"])
                        },

                mention_type : match value_to_string(&json["activity"]["type"]).as_str() {
                                "mention" => WebmentionType::mention,
                                "like" => WebmentionType::like,
                                "reply" => WebmentionType::reply,
                                "repost" => WebmentionType::repost,
                                "bookmark" => WebmentionType::bookmark,
                                "rsvp" => WebmentionType::rsvp,
                                _ => WebmentionType::mention,

                }
        }
}

fn get_request(url : &Url) -> Result<Value,String> {
        match get(url.as_str()) {
                Ok(mut response) => match response.json() {
                        Ok(value) => Ok(value),
                        Err(_) => Err("The result could not be parsed into json".to_string())
                },
                Err(_) => Err("The URL could not be contacted".to_string())
        }
}

pub fn query_backend(url : &Url) -> Result<WebmentionCollection,String> {
        let json : Value = get_request(url)?;
        if let Some(Value::Array(array)) = json.get("links") {
                let mut webmention_collection : WebmentionCollection = WebmentionCollection::new();
                for json_webmention in array {
                        let webmention : Webmention = convert_json_to_webmention(json_webmention);
                        if let Ok(target_url) = Url::parse(&value_to_string(&json_webmention["target"])) {
                                webmention_collection.add(target_url.path().to_string(),webmention);
                        }
                        else {
                                return Err("A link is invalid in the json: ".to_string() + &json_webmention["target"].to_string());
                        }
                }
                Ok(webmention_collection)
        }
        else {
                Err("Json format is invalid".to_string())
        }
}