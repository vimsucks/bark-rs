use std::{collections::HashMap};

use reqwest::header::HeaderMap;
use serde_json::{Value, json};

#[derive(Debug)]
pub struct App<'a> {
    server_url: &'a str,
    device_key: &'a str,
}

#[derive(Clone, Debug)]
pub enum PushLevel {
    Active,
    TimeSensitive,
    Passive,
}

impl PushLevel {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::TimeSensitive => "timeSensitive",
            Self::Passive => "passive",
        }
    }
}

#[derive(Debug)]
pub struct PushBuilder {
    request_body: HashMap<&'static str, Value>,
}

pub fn new_push() -> PushBuilder {
    PushBuilder { request_body: HashMap::new() }
}

impl PushBuilder {
    pub fn with_title(&mut self, title: &str) -> &mut PushBuilder {
        self.request_body.insert("title", json!(title));
        self
    }

    pub fn with_body(&mut self, body: &str) -> &mut PushBuilder {
        self.request_body.insert("body", json!(body));
        self
    }

    pub fn with_level(&mut self, level: PushLevel) -> &mut PushBuilder {
        self.request_body.insert("level", json!(level.to_str()));
        self
    }

    pub fn with_badge(&mut self, badge: u32) -> &mut PushBuilder {
        self.request_body.insert("badge", json!(badge));
        self
    }

    pub fn with_auto_copy(&mut self, auto_copy: bool) -> &mut PushBuilder {
        self.request_body.insert("autoCopy", json!(auto_copy));
        self
    }

    pub fn with_copy(&mut self, copy: &str) -> &mut PushBuilder {
        self.request_body.insert("copy", json!(copy));
        self
    }

    pub fn with_sound(&mut self, sound: &str) -> &mut PushBuilder {
        self.request_body.insert("sound", json!(sound));
        self
    }

    pub fn with_icon(&mut self, icon: &str) -> &mut PushBuilder {
        self.request_body.insert("icon", json!(icon));
        self
    }

    pub fn with_group(&mut self, group: &str) -> &mut PushBuilder {
        self.request_body.insert("group", json!(group));
        self
    }

    pub fn with_archive(&mut self, archive: bool) -> &mut PushBuilder {
        if archive {
        self.request_body.insert("isArchive", json!(1));
        }
        self
    }

    pub fn with_url(&mut self, url: &str) -> &mut PushBuilder {
        self.request_body.insert("url", json!(url));
        self
    }
}

impl<'a> App<'a> {

    pub fn new(server_url: &'a str, device_key: &'a str) -> App {
        App{ server_url, device_key }
    }

    pub async fn send(&self, args: &PushBuilder) -> Result<HashMap<String, Value>, reqwest::Error>{
        let url = format!("{}/{}", self.server_url, self.device_key);
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        Ok(client.post(url)
        .headers(headers)
        .json(&args.request_body)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
