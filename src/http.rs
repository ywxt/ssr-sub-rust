use crate::error;
use crate::parser::ssr::SsrUrl;
use lazing::lazy;
use reqwest::blocking;
use std::convert::TryInto;

#[lazy]
static HTTP_CLIENT: blocking::Client = blocking::Client::new();

pub fn get_text(url: impl reqwest::IntoUrl) -> error::Result<String> {
    static HTTP_CLIENT: blocking::Client = blocking::Client::new();
    HTTP_CLIENT.get(url).send()?.text().map_err(|e| e.into())
}

pub fn get_ssr_urls(url: impl reqwest::IntoUrl) -> error::Result<Vec<SsrUrl>> {
    let text = get_text(url)?;
    text.lines().map(|line| line.try_into()).collect()
}
