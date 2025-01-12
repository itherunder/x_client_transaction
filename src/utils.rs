use regex::Regex;
use scraper::{Html, Selector};

pub async fn handle_x_migration(http_cli: &reqwest::Client) -> Result<(), anyhow::Error> {
    let mut home_page;
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

    todo!()
}
