use regex::Regex;
use crate::Streamable;
use crate::utils::error::StreamError;
use crate::utils::error::RsgetError;
use crate::plugins::{
    douyu::Douyu,
    panda::PandaTv,
    xingyan::Xingyan,
    inke::Inke,
    afreeca::Afreeca,
    douyin::Douyin,
    tiktok::TikTok
};

use reqwest;

pub fn get_site(input: &str) -> Result<Box<Streamable>, StreamError> {
    match _get_site(input) {
        Ok(s) => Ok(s),
        Err(StreamError::Rsget(_)) => {
            let res = reqwest::get(input)?;
            let final_url = res.url().as_str();
            _get_site(final_url)
        },
        Err(why) => Err(why),
    }
}

fn _get_site(input: &str) -> Result<Box<Streamable>, StreamError> {
    let re_xingyan_panda: Regex = Regex::new(r"^(?:https?://)?xingyan\.panda\.tv/[0-9]+/?")?;
    let re_panda: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?panda\.tv/[0-9]+/?")?;
    let re_douyu: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?douyu\.com/[a-zA-Z0-9]+/?")?;
    let re_afreeca: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?(?:play\.)?afreecatv.com/[a-zA-Z0-9]+/?(?:/[0-9]+)?")?;
    let re_inke: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?inke\.cn/live\.html\?uid=[0-9]+")?;
    let re_douyin: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?iesdouyin\.com/.*")?;
    let re_tiktok: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?(?:m\.)?tiktok\.com/v/(?:[a-zA-Z0-9]+)(?:\.html)?")?;
    match input {
        url if re_panda.is_match(url) => {
            Ok(PandaTv::new(String::from(url))?)
        },
        url if re_xingyan_panda.is_match(url) => {
            Ok(Xingyan::new(String::from(url))?)
        },
        url if re_douyu.is_match(url) => {
            Ok(Douyu::new(String::from(url))?)
        },
        url if re_afreeca.is_match(url) => {
            Ok(Afreeca::new(String::from(url))?)
        },
        url if re_inke.is_match(url) => {
            Ok(Inke::new(String::from(url))?)
        },
        url if re_douyin.is_match(url) => {
            Ok(Douyin::new(String::from(url))?)
        },
        url if re_tiktok.is_match(url) => {
            Ok(TikTok::new(String::from(url))?)
        },
        _ => Err(StreamError::Rsget(RsgetError::new("Site not supported."))),
    }
}
