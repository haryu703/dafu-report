#[macro_use]
extern crate clap;

use easy_scraper::Pattern;

#[derive(Debug)]
struct Ticket {
    id: String,
    price: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app_from_crate!()
        .arg(clap::Arg::from_usage("--url=<URL> '検索画面のURL'").required(true))
        .arg(
            clap::Arg::from_usage("--min=<MIN> '表示するチケットの最低価格'")
                .default_value("0")
                .required(false),
        )
        .get_matches();

    let pat = Pattern::new(
        r#"
        <li class="list-ticket-price">
            <span class="ticket-price">￥{{price}}</span>
        </li>
        <li class="list-ticket-order">
            <div class="watch-star">
                <span class="js-watchlists-buttons watchlists-buttons" js-ticket_id="{{ticket_id}}"></span>
            </div>
        </li>
    "#,
    )?;

    let url = app.value_of("url").unwrap();
    let min_price: u32 = app.value_of("min").unwrap().parse()?;

    let body = reqwest::get(url).await?.text().await?;

    let result = pat.matches(&body).into_iter().filter_map(|map| {
        let id = map.get("ticket_id")?.to_string();
        let price = map.get("price")?.replace(",", "").parse().ok()?;

        if price < min_price {
            return None;
        }

        Some(Ticket { id, price })
    });

    for t in result {
        println!("掲載番号: {} 価格: ￥{}", t.id, t.price);
    }

    Ok(())
}
