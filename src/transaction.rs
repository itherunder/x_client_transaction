use std::{time::{ SystemTime, UNIX_EPOCH}, vec};

use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use scraper::{html::Select, ElementRef, Html, Selector};
use sha2::{Sha256, Digest};

use crate::{cubic_curve::Cubic, interpolate::interpolate, rotation::convert_rotation_to_matrix, utils::{base64_decode, base64_encode, float_to_hex, is_odd}};

lazy_static! {
    pub static ref ON_DEMAND_FILE_REGEX: Regex =
        Regex::new(r#"['|"]{1}ondemand\.s['|"]{1}:\s*['|"]{1}([\w]*)['|"]{1}"#).unwrap();
    pub static ref INDICES_REGEX: Regex = Regex::new(r#"(\(\w{1}\[(\d{1,2})\],\s*16\))+"#).unwrap();
}

pub const ADDITIONAL_RANDOM_NUMBER: u8 = 3;
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
        _self.animation_key = _self.get_animation_key(&_self.key_bytes, &home_page_response);
        _self
    }

    pub fn get_animation_key(&self, key_bytes: &[u8], page: &Html) -> String {
        let total_time = 4096_f64;
        let row_index = key_bytes[self.row_index as usize] % 16;
        let frame_time = self.key_bytes_indices.iter().map(|index| {
            (key_bytes[*index as usize] % 16) as u64
        }).collect::<Vec<u64>>().into_iter().reduce(|num1, num2| {
            num1 * num2
        }).unwrap();
        let arr = get_2d_array(key_bytes, page, None);
        let frame_row = &arr[row_index as usize];
        let target_time = frame_time as f64 / total_time;

        animate(&frame_row, target_time)
    }

    pub fn generate_transaction_id(&self, method: &str, path: &str) -> String {
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() - 1682924400;
        let time_now_bytes = [0,1,2,3].into_iter().map(|i| {
            (time_now >> (i * 8)) as u8 & 0xFF
        }).collect::<Vec<u8>>();
        // let key = &self.key; // useless
        let key_bytes = &self.key_bytes;
        let animation_key= &self.animation_key;
        let input = format!(
            "{}!{}!{}{}{}",
            method,
            path,
            time_now,
            DEFAULT_KEYWORD,
            animation_key
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash_bytes = hasher.finalize().to_vec();
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(0..255) as u8;
        let mut bytes_arr = vec![];
        bytes_arr.extend_from_slice(&key_bytes);
        bytes_arr.extend_from_slice(&time_now_bytes);
        bytes_arr.extend_from_slice(&hash_bytes[..16]);
        bytes_arr.push(ADDITIONAL_RANDOM_NUMBER);
        let mut out = vec![random_num as u8];
        out.extend(bytes_arr.into_iter().map(|item| {
            item ^ random_num
        }));

        let id: String = base64_encode(&out).strip_prefix('=').unwrap().into();
        id.strip_suffix('=').unwrap().into()
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

pub fn animate<'a, 'b>(_frames: &[i64], target_time: f64) -> String {
    let mut from_color = _frames[..3].into_iter().map(|item| {
        *item as f64
    }).collect::<Vec<f64>>();
    from_color.push(1.0);
    let mut to_color = _frames[3..6].into_iter().map(|item| {
        *item as f64
    }).collect::<Vec<f64>>();
    to_color.push(1.0);
    let from_rotation = vec![0.0];
    let to_rotation = vec![solve(_frames[6] as f64, 60.0, 360.0, true)];
    let frames = &_frames[7..];
    let curves = frames.into_iter().enumerate().map(|(counter, item)| {
        solve(*item as f64, is_odd(counter as i64), 1.0, false)
    }).collect::<Vec<f64>>();
    let cubic = Cubic::new(curves);
    let val = cubic.get_value(target_time);
    let _color = interpolate(from_color, to_color, val);
    let color = _color.into_iter().map(|value|{
        if value > 0.0 {
            value
        } else {
            0.0
        }
    }).collect::<Vec<f64>>();
    let rotation = interpolate(from_rotation, to_rotation, val);
    let matrix = convert_rotation_to_matrix(rotation[0]);
    let mut str_arr = color[..color.len()-1].into_iter().map(|value| {
        format!("{:x}", value.round() as i64)
    }).collect::<Vec<String>>();
    matrix.into_iter().for_each(|value| {
        let rounded =((value * 100.0).round() / 100.0).abs();
        let hex_value = float_to_hex(rounded);
        str_arr.push(if hex_value.starts_with('.') {
            format!("0{}", hex_value).to_lowercase()
        } else {
            if hex_value.is_empty() {
                "0".into()
            } else {
                hex_value
            }
        });
    });
    str_arr.extend(vec!["0".into(), "0".into()].into_iter());
    let re = regex::Regex::new(r#"[.-]"#).unwrap();
    let str_arr_str = str_arr.join("");
    let animation_key = re.replace_all(&str_arr_str, "");
    animation_key.into_owned()
}
