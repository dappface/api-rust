mod block;
mod payload;
mod text_object;

pub use block::SectionBuilder;
pub use payload::{Payload, PayloadBuilder};
pub use reqwest::{self, Error, Response};
use std::future::Future;

#[derive(Clone)]
pub struct Slack<'a> {
    api_token: String,
    payload: Option<Payload<'a>>,
}

impl<'a> Slack<'a> {
    pub fn new(api_token: String) -> Slack<'a> {
        Slack {
            api_token,
            payload: None,
        }
    }

    pub fn payload(self, payload: Payload<'a>) -> Slack<'a> {
        Slack {
            api_token: self.api_token,
            payload: Some(payload),
        }
    }

    pub fn send(&self, url: &'a str) -> impl Future<Output = Result<Response, Error>> {
        reqwest::Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", &self.api_token))
            .json(&self.payload)
            .send()
    }
}
