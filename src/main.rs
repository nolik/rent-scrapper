use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.otodom.pl/pl/oferty/wynajem/mieszkanie/lodz")?;
    let body = resp.text()?;
    // println!("{:#?}", body);

    let document = Html::parse_document(&body);  // Parse the HTML document
    let selector = Selector::parse(r#"a[data-cy="listing-item-link"]"#).unwrap();
    // let search_listening_el = document.select(&selector).next().unwrap();
    // let title_selector = Selector::parse(r#"h3[data-cy="listing-item-title"]"#).unwrap();
    for element in document.select(&selector) {
        println!("------------------------------");
        println!("{:#?}", element.value());
        // let href_selector = Selector::parse("href").unwrap();
        println!("{:#?}", element.select(&Selector::parse(r#"article h3[data-cy="listing-item-title"]"#).unwrap()).next().unwrap().inner_html());
        println!("{:#?}", element.select(&Selector::parse(r#"article p span"#).unwrap()).next().unwrap().inner_html());
        println!("{:#?}", element.value().attr("href"));
    }

    // for element in document.select(&selector) {
    //     println!("------------------------------");
    //     println!("{:#?}", element.text());
    // }
    // for element in search_listening_el.select(&title_selector) {
    //     println!("{:#?}", element.text());
    // }
    Ok(())
}