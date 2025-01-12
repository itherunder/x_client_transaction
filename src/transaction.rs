use lazy_static::lazy_static;
use regex::Regex;
use scraper::{html::Select, ElementRef, Html, Selector};

use crate::utils::base64_decode;

lazy_static! {
    pub static ref ON_DEMAND_FILE_REGEX: Regex =
        Regex::new(r#"['|"]{1}ondemand\.s['|"]{1}:\s*['|"]{1}([\w]*)['|"]{1}"#).unwrap();
    pub static ref INDICES_REGEX: Regex = Regex::new(r#"(\(\w{1}\[(\d{1,2})\],\s*16\))+"#).unwrap();
}

pub const ADDITIONAL_RANDOM_NUMBER: i64 = 3;
pub const DEFAULT_KEYWORD: &str = "obfiowerehiring";

#[derive(Debug, Default)]
pub struct ClientTransaction {
    pub row_index: i64,
    pub key_bytes_indices: Vec<i64>,
    pub home_page_response: Option<Html>,
    pub key: String,
    pub key_bytes: Vec<u8>,
    pub animation_key: String,
}

impl ClientTransaction {
    pub async fn new(home_page_response: Html) -> ClientTransaction {
        let mut _self = ClientTransaction::default();
        _self.home_page_response = Some(home_page_response.clone());
        (_self.row_index, _self.key_bytes_indices) = get_indices(&home_page_response).await;

        _self.key = get_key(&home_page_response);
        _self.key_bytes = get_key_bytes(&_self.key);
        _self.animation_key = get_animation_key(&_self.key_bytes, &home_page_response);
        _self
    }
}

pub async fn get_indices(page: &Html) -> (i64, Vec<i64>) {
    let mut key_byte_indices = Vec::new();
    let err_msg = "Couldn't get KEY_BYTE indices";
    let page_str = page.html();
    let on_demand_file = ON_DEMAND_FILE_REGEX.captures(&page_str).expect(err_msg);

    let s = on_demand_file.get(1).expect(err_msg).as_str();
    let on_demand_file_url =
        format!("https://abs.twimg.com/responsive-web/client-web/ondemand.s.{s}a.js");
    let response = reqwest::get(on_demand_file_url).await.expect(err_msg);
    INDICES_REGEX
        .captures_iter(&response.text().await.expect(err_msg))
        .into_iter()
        .for_each(|m| {
            key_byte_indices.push(
                m.get(2)
                    .expect(err_msg)
                    .as_str()
                    .parse::<i64>()
                    .unwrap_or_default(),
            );
        });

    (
        key_byte_indices[0],
        key_byte_indices[1..].into_iter().map(|i| *i).collect(),
    )
}

pub fn get_key(page: &Html) -> String {
    page.select(&Selector::parse("[name='twitter-site-verification']").unwrap())
        .next()
        .expect("Couldn't get key from the page source")
        .attr("content")
        .unwrap_or("")
        .into()
}

pub fn get_key_bytes(key: &str) -> Vec<u8> {
    base64_decode(key)
}

pub fn get_frames<'a, 'b>(page: &'a Html, selector: &'b Selector) -> Select<'a, 'b> {
    page.select(selector)
}

pub fn get_2d_array<'a, 'b>(
    key_bytes: &[u8],
    page: &'a Html,
    frames: Option<Select<'a, 'b>>,
) -> Vec<Vec<i64>> {
    let selector = Selector::parse("[id^='loading-x-anim']").unwrap();
    let _frames = frames.unwrap_or(get_frames(page, &selector));
    let child = ElementRef::wrap(
        _frames.collect::<Vec<_>>()[key_bytes[5] as usize % 4]
            .children()
            .collect::<Vec<_>>()[0]
            .children()
            .collect::<Vec<_>>()[1],
    )
    .unwrap();
    let re = Regex::new(r#"[^\d]+"#).unwrap();
    child.attr("d").unwrap()[9..]
        .split('C')
        .into_iter()
        .map(|item| {
            re.replace_all(item, " ")
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .collect()
}

pub fn solve(value: f64, min_val: f64, max_val: f64, rounding: bool) -> f64 {
    let res = value * (max_val - min_val) / 255.0 + min_val;
    if rounding {
        res.floor()
    } else {
        (res * 100.0).round() / 100.0
    }
}

pub fn animate<'a, 'b>(frames: &[i64], target_time: f64) {
    todo!()
}

pub fn get_animation_key(key_bytes: &[u8], page: &Html) -> String {
    todo!()
}
