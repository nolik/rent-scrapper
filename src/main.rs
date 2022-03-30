use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.otodom.pl/pl/oferty/wynajem/mieszkanie/lodz")?;
    let body = resp.text()?;
    // println!("{:#?}", body);

    let document = Html::parse_document(&body);  // Parse the HTML document
    let selector = Selector::parse(r#"div[data-cy="search.listing"] li"#).unwrap();
    let search_listening_el = document.select(&selector).next().unwrap();
    // let title_selector = Selector::parse(r#"h3[data-cy="listing-item-title"]"#).unwrap();
    println!("{:#?}", search_listening_el);
    // for element in document.select(&selector) {
    //     println!("------------------------------");
    //     println!("{:#?}", element.text());
    // }
    // for element in search_listening_el.select(&title_selector) {
    //     println!("{:#?}", element.text());
    // }
    Ok(())
}