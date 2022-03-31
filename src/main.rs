use scraper::{Html, Selector};

struct Post {
    title: String,
    link: String,
    area: String,
    price: String,
    rooms: String,
    size: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.otodom.pl/pl/oferty/wynajem/mieszkanie/lodz")?;
    let body = resp.text()?;

    let document = Html::parse_document(&body);  // Parse the HTML document
    let selector = Selector::parse(r#"a[data-cy="listing-item-link"]"#).unwrap();
    for element in document.select(&selector) {
        println!("------------------------------");
        println!("{:#?}", element.value());
        println!("Title:{:#?}", element.select(&Selector::parse(r#"article h3[data-cy="listing-item-title"]"#).unwrap()).next().unwrap().inner_html());
        println!("Link:{:#?}", element.value().attr("href").unwrap());
        let article_selector = Selector::parse(r#"article p"#).unwrap();
        let mut article_iterator = element.select(&article_selector);
        println!("Area:{:#?}", article_iterator.next().unwrap().inner_html());
        println!("Price:{:#?}", article_iterator.next().unwrap().inner_html());
        let span_selector = Selector::parse(r#"span"#).unwrap();
        let mut span_iterator = article_iterator.next().unwrap().select(&span_selector);
        println!("Rooms:{:#?}", span_iterator.next().unwrap().inner_html());
        println!("Size:{:#?}", span_iterator.next().unwrap().inner_html());
    }

    Ok(())
}