fn main() {
    let stocks = ["nvda", "aapl", "msft", "goog"];
    let capatalized_stocks = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect::<Vec<String>>();

    println!("{capatalized_stocks:?}");
}
