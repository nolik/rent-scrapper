use reqwest::blocking::{get, Client};
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::thread::sleep;
use std::time;

const OTODOM_BASE_URL: &str = "https://www.otodom.pl";
const OTODOM_SEARCH_URL: &str = "https://www.otodom.pl/pl/oferty/wynajem/mieszkanie/";
const OLX_SEARCH_URL: &str = "https://www.olx.pl/nieruchomosci/mieszkania/wynajem/";
const TELEGRAM_SEND_MSG_URL: &str =
    "https://api.telegram.org/{TELEGRAM_BOT_ID}:{TELEGRAM_BOT_TOKEN}/sendMessage";
const CHAT_SEARCH_LINKS: [(&str, &str); 2] = [("lodz", "785197852"), ("wroclaw", "785197852")];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handled_links = HashSet::new();
    let client = Client::new();

    loop {
        sleep(time::Duration::from_secs(5));
        for chat_tuple in CHAT_SEARCH_LINKS.iter() {
            handle_otodom_posts(&client, &mut handled_links, chat_tuple);
            handle_olx_posts(&client, &mut handled_links, chat_tuple);
        }
    }
}

fn handle_otodom_posts(
    client: &Client,
    handled_links: &mut HashSet<String>,
    chat_tuple: &(&str, &str),
) {
    match get(format!("{}{}", OTODOM_SEARCH_URL, chat_tuple.0)) {
        Ok(resp) => {
            let body = resp.text().unwrap();
            let document = Html::parse_document(&body);
            let selector = Selector::parse(r#"a[data-cy="listing-item-link"]"#).unwrap();
            for element in document.select(&selector) {
                let link = OTODOM_BASE_URL.to_owned() + element.value().attr("href").unwrap();
                handle_parsed_link(client, handled_links, link, chat_tuple.1);
            }
        }
        Err(err) => {
            println!("Failure to send otodom request: {:?}", err);
        }
    }
}

fn handle_olx_posts(
    client: &Client,
    handled_links: &mut HashSet<String>,
    chat_tuple: &(&str, &str),
) {
    match get(format!("{}{}", OLX_SEARCH_URL, chat_tuple.0)) {
        Ok(resp) => {
            let body = resp.text().unwrap();
            let document = Html::parse_document(&body);
            let selector = Selector::parse(r#"a[data-cy="listing-ad-title"]"#).unwrap();
            for element in document.select(&selector) {
                let link = element.value().attr("href").unwrap();
                handle_parsed_link(client, handled_links, link.to_string(), chat_tuple.1);
            }
        }
        Err(err) => {
            println!("Failure to send olx request: {:?}", err);
        }
    }
}

fn handle_parsed_link(
    client: &Client,
    handled_links: &mut HashSet<String>,
    link: String,
    chat_id: &str,
) {
    if !handled_links.contains(&link) {
        println!("Link:{:#?}", &link);
        send_link_to_telegram(client, chat_id, &link);
        handled_links.insert(link);
    }
}

fn send_link_to_telegram(client: &Client, chat_id: &str, link: &str) {
    let params = [("chat_id", chat_id), ("text", link)];
    match client.post(TELEGRAM_SEND_MSG_URL).form(&params).send() {
        Ok(res) => println!("Telegram response:{:#?}", res.status()),
        Err(e) => println!("Error sending message to Telegram: {}", e),
    }
}
