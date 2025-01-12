use std::{collections::HashMap, str::FromStr};

use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use reqwest::Method;
use scraper::{Html, Selector};

use crate::cubic_curve::Curve;

pub async fn handle_x_migration(http_cli: &reqwest::Client) -> Result<Html, anyhow::Error> {
    let migration_redirection_regex = Regex::new(
        r#"(http(?:s)?://(?:www\.)?(twitter|x){1}\.com(/x)?/migrate([/?])?tok=[a-zA-Z0-9%\-_]+)+"#,
    )
    .unwrap();

    let mut response = http_cli.get("https://x.com").send().await?;
    let mut body = response.text().await?;
    let mut home_page = Html::parse_document(&body);
    let migration_url = home_page
        .select(&Selector::parse("meta[http-equiv='refresh']").unwrap())
        .next();
    let migration_url_str = match migration_url {
        Some(url) => url.value().attr("content").unwrap(),
        None => "",
    };
    let migration_redirection_url = migration_redirection_regex
        .find(migration_url_str)
        .or_else(|| migration_redirection_regex.find(&body));
    if let Some(redirection_url) = migration_redirection_url {
        response = http_cli.get(redirection_url.as_str()).send().await?;
        body = response.text().await?;
        home_page = Html::parse_document(&body);
    }

    let migration_form = home_page
        .select(&Selector::parse("form[name='f']").unwrap())
        .next()
        .or_else(|| {
            home_page
                .select(&Selector::parse("form[action='https://x.com/x/migrate']").unwrap())
                .next()
        });

    if let Some(form) = migration_form {
        let url = form.attr("action").unwrap_or("https://x.com/x/migrate");
        let method = form.attr("method").unwrap_or("POST");
        let selector = Selector::parse("input").unwrap();
        let input = form.select(&selector);
        let mut request_payload = HashMap::new();
        for input_field in input {
            let name = input_field.attr("name").unwrap();
            let value = input_field.attr("value").unwrap();
            request_payload.insert(name, value);
        }
        let data = serde_json::to_string(&request_payload)?;
        response = http_cli
            .request(Method::from_str(method)?, url)
            .body(data)
            .send()
            .await?;
        body = response.text().await?;
        home_page = Html::parse_document(&body);
    }

    Ok(home_page)
}

pub fn float_to_hex(mut x: f64) -> String {
    let mut result = Vec::new();
    let mut quotient = x.trunc() as i64;
    let mut fraction = x.fract();

    while quotient > 0 {
        quotient = (x / 16.0).trunc() as i64;
        let remainder = (x - (quotient as f64 * 16.0)).trunc() as i64;
        if remainder > 9 {
            result.insert(0, (remainder as u8 + 55) as char);
        } else {
            result.insert(0, (remainder as u8 + b'0') as char);
        }
        x = quotient as f64;
    }

    if fraction == 0.0 {
        return result.into_iter().collect();
    }

    result.push('.');

    while fraction > 0.0 {
        fraction *= 16.0;
        let integer = fraction.trunc() as i64;
        fraction -= integer as f64;

        if integer > 9 {
            result.push((integer as u8 + 55) as char);
        } else {
            result.push((integer as u8 + b'0') as char);
        }
    }

    result.into_iter().collect()
}

pub fn is_odd(num: Curve) -> f64 {
    if match num {
        Curve::Float(f) => f % 2.0 != 0.0,
        Curve::Int(i) => i % 2 != 0,
    } {
        -1.0
    } else {
        0.0
    }
}

pub fn base64_encode(input: &[u8]) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn base64_decode(input: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(input).unwrap()
}
