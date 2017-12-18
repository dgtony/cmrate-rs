
extern crate cryptoconv;


// Print ratio of one cryptocurrency to another using both USD and BTC prices
fn print_currency_rate(asset1: &cryptoconv::Asset, asset2: &cryptoconv::Asset) {
    let ratio_usd = cryptoconv::currency_ratio_usd(&asset1, &asset2);
    let ratio_btc = cryptoconv::currency_ratio_btc(&asset1, &asset2);
    println!("USD price ratio: 1 {} = {} {}", &asset1.symbol, ratio_usd, &asset2.symbol);
    println!("BTC price ratio: 1 {} = {} {}", &asset1.symbol, ratio_btc, &asset2.symbol);
}


// Print cryptocurrency fiat price (in USD)
fn print_currency_price_usd(asset: &cryptoconv::Asset) {
    println!("1 {} = {} USD", asset.symbol, asset.price_usd);
}


fn compare_currencies(currency_id1: &str, currency_id2: &str) {
    let mut client = cryptoconv::CMClient::new().unwrap();

    let asset_result1 = client.get_asset(currency_id1);
    let asset_result2 = client.get_asset(currency_id2);

    match (asset_result1, asset_result2) {
        (Err(e1), _) => println!("cannot get information for {} | error: {}", currency_id1, e1),
        (_, Err(e2)) => println!("cannot get information for {} | error: {}", currency_id2, e2),
        (Ok(asset1), Ok(asset2)) => {
            print_currency_rate(&asset1, &asset2);
            println!("\nFiat prices:");
            print_currency_price_usd(&asset1);
            print_currency_price_usd(&asset2);
        }
    }
}


fn main() {
    // todo get names from env

    compare_currencies("bitcoin", "ethereum");
}