use crate::error;
use crate::parser::ssr::SsrUrl;
use lazing::lazy;
use reqwest::blocking;
use std::convert::TryInto;

#[lazy]
static HTTP_CLIENT: blocking::Client = blocking::Client::new();

pub fn get_text(url: impl reqwest::IntoUrl) -> error::Result<String> {
    let base64_text = HTTP_CLIENT.get(url).send()?.text()?;
    let text = String::from_utf8(base64::decode(&base64_text).map_err(|_| {
        error::Error::from_kind(error::ErrorKind::InvalidSsrSubscription(
            base64_text.clone(),
        ))
    })?)
    .map_err(|_| {
        error::Error::from_kind(error::ErrorKind::InvalidSsrSubscription(
            base64_text.clone(),
        ))
    })?;
    Ok(text)
}

pub fn get_ssr_urls(url: impl reqwest::IntoUrl) -> error::Result<Vec<SsrUrl>> {
    let text = get_text(url)?;
    text.lines().map(|line| line.try_into()).collect()
}
