use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::thread::sleep;
use std::time;

const OTODOM_BASE_URL: &str = "https://www.otodom.pl";
const OTODOM_SEARCH_URL: &str = "https://www.otodom.pl/pl/oferty/wynajem/mieszkanie/lodz";
const OLX_SEARCH_URL: &str = "https://www.olx.pl/nieruchomosci/mieszkania/wynajem/lodzkie/";
const TELEGRAM_SEND_MSG_URL: &str = "https://api.telegram.org/{TELEGRAM_BOT_ID}:{TELEGRAM_BOT_TOKEN}/sendMessage";
const CHAT_ID_PARAM: (&str, &str) = ("chat_id", "{TELEGRAM_CHAT_ID}");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handled_links = HashSet::new();
    let client = reqwest::blocking::Client::new();

    loop {
        sleep(time::Duration::from_secs(5));
        handle_otodom_posts(&client, &mut handled_links)?;
        handle_olx_posts(&client, &mut handled_links)?;
    }
}

fn handle_otodom_posts(
    client: &Client,
    handled_links: &mut HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(OTODOM_SEARCH_URL)?;
    let body = resp.text()?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"a[data-cy="listing-item-link"]"#).unwrap();
    for element in document.select(&selector) {
        let link = OTODOM_BASE_URL.to_owned() + element.value().attr("href").unwrap();
        println!("Link:{:#?}", &link);
        handle_parsed_link(client, handled_links, link);
    }
    Ok(())
}

fn handle_olx_posts(client: &Client, handled_links: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(OLX_SEARCH_URL)?;
    let body = resp.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"a[data-cy="listing-ad-title"]"#).unwrap();
    for element in document.select(&selector) {
        let link = element.value().attr("href").unwrap();
        println!("Link:{:#?}", &link);
        handle_parsed_link(client, handled_links, link.to_string());
    }
    Ok(())
}

fn handle_parsed_link(client: &Client, handled_links: &mut HashSet<String>, link: String) {
    if !handled_links.contains(&link) {
        send_link_to_telegram(client, &link);
        handled_links.insert(link);
    }
}

fn send_link_to_telegram(client: &Client, link: &str) {
    let params = [CHAT_ID_PARAM, ("text", link)];
    match client.post(TELEGRAM_SEND_MSG_URL).form(&params).send() {
        Ok(res) => println!("Telegram response:{:#?}", res.status()),
        Err(e) => println!("Error sending message to Telegram: {}", e),
    }
}
