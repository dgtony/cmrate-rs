
extern crate cryptoconv;


use std::env;
use cryptoconv::api;


fn print_usage(prog_name: &str) {
    println!("Usage: {} <currency-id-1> <currency-id-2>", prog_name);
}


// Print ratio of one cryptocurrency to another using both USD and BTC prices
fn print_currency_rate(asset1: &api::Asset, asset2: &api::Asset) {
    let ratio_usd = api::currency_ratio_usd(&asset1, &asset2);
    let ratio_btc = api::currency_ratio_btc(&asset1, &asset2);
    println!(
        "USD price ratio: 1 {} = {} {}",
        &asset1.symbol,
        ratio_usd,
        &asset2.symbol
    );
    println!(
        "BTC price ratio: 1 {} = {} {}",
        &asset1.symbol,
        ratio_btc,
        &asset2.symbol
    );
}


// Print cryptocurrency fiat price (in USD)
fn print_currency_price_usd(asset: &api::Asset) {
    println!("1 {} = {} USD", asset.symbol, asset.price_usd);
}


fn compare_currencies(currency_id1: &str, currency_id2: &str) {
    let mut client = api::CMClient::new().unwrap();

    let asset_result1 = client.get_asset(currency_id1);
    let asset_result2 = client.get_asset(currency_id2);

    match (asset_result1, asset_result2) {
        (Err(_e1), _) => {
            println!(
                "cannot get information for {}",
                currency_id1,
            )
        }
        (_, Err(_e2)) => {
            println!(
                "cannot get information for {}",
                currency_id2,
            )
        }
        (Ok(asset1), Ok(asset2)) => {
            print_currency_rate(&asset1, &asset2);
            println!("\nFiat prices:");
            print_currency_price_usd(&asset1);
            print_currency_price_usd(&asset2);
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = &args[0];
    let cid1 = args.get(1);
    let cid2 = args.get(2);

    if let (Some(currency1), Some(currency2)) = (cid1, cid2) {
        compare_currencies(currency1, currency2);
    } else {
        print_usage(&prog_name);
    }
}
