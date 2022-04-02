use scraper::{Html, Selector};
use std::collections::HashSet;
use std::thread::sleep;
use std::time;

const SITE_URL: &str = "https://www.otodom.pl";
const SEARCH_URL: &str = "https://www.otodom.pl/pl/oferty/wynajem/mieszkanie/lodz";
const TELEGRAM_SEND_MSG_URL: &str = "https://api.telegram.org/bot{BOT_TOKEN}/sendMessage";
const CHAT_ID_PARAM: (&str, &str) = ("chat_id", "{chat_id}");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut posts = HashSet::new();
    let client = reqwest::blocking::Client::new();

    loop {
        sleep(time::Duration::from_secs(5));
        let resp = reqwest::blocking::get(SEARCH_URL)?;
        let body = resp.text()?;

        let document = Html::parse_document(&body);
        let selector = Selector::parse(r#"a[data-cy="listing-item-link"]"#).unwrap();
        for element in document.select(&selector) {
            let link = SITE_URL.to_owned() + element.value().attr("href").unwrap();
            println!("Link:{:#?}", &link);
            if !posts.contains(&link) {
                let params = [CHAT_ID_PARAM, ("text", &link)];
                let res = client.post(TELEGRAM_SEND_MSG_URL).form(&params).send()?;
                println!("Telegram response:{:#?}", res.status());

                posts.insert(link);
            }
        }
    }
}
