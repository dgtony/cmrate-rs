
extern crate serde_json;

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::{Client, Body};
use hyper::client::HttpConnector;
use std::error::Error;
use std::io;
use std::fmt;


const ERR_NO_DATA: &'static str = "no data found";


#[derive(Debug)]
struct ProcError {
    desc: &'static str,
}


impl Error for ProcError {
    fn description(&self) -> &'static str {
        &self.desc
    }
}


impl fmt::Display for ProcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}


#[derive(Debug, Deserialize, Clone)]
struct RawAsset {
    id: String,
    symbol: String,
    price_usd: String,
    price_btc: String,
}


#[derive(Debug)]
pub struct Asset {
    pub id: String,
    pub symbol: String,
    pub price_usd: f64,
    pub price_btc: f64,
}


// HTTPS client
pub struct CMClient {
    core: Core,
    client: Client<HttpsConnector<HttpConnector>, Body>,
}


impl CMClient {
    // Create new HTTPS client
    pub fn new() -> Result<CMClient, Box<Error>> {
        let core = Core::new()?;
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle)?)
            .build(&handle);

        Ok(CMClient { core, client })
    }

    fn get_asset_info(&mut self, uri: &str) -> Result<RawAsset, Box<Error>> {
        let uri = uri.parse()?;
        let job = self.client.get(uri).and_then(|res| {
            res.body().concat2().and_then(move |body| {
                let v: Vec<RawAsset> = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, e)
                })?;
                Ok(v)
            })
        });

        let response = self.core.run(job)?;
        match response.first() {
            // FIXME clone
            Some(data) => Ok(data.clone()),
            None => Err(Box::new(ProcError { desc: ERR_NO_DATA })),
        }
    }

    fn raw_to_asset(asset: RawAsset) -> Result<Asset, Box<Error>> {
        let price_usd: f64 = asset.price_usd.parse()?;
        let price_btc: f64 = asset.price_btc.parse()?;

        Ok(Asset {
            id: asset.id,
            symbol: asset.symbol,
            price_usd,
            price_btc,
        })
    }

    // Get asset information from coinmarketcap
    pub fn get_asset(&mut self, currency_id: &str) -> Result<Asset, Box<Error>> {
        let uri = format!("https://api.coinmarketcap.com/v1/ticker/{}/", currency_id);
        let raw_asset = self.get_asset_info(&uri)?;
        CMClient::raw_to_asset(raw_asset)
    }
}


pub fn currency_ratio_usd(asset_from: &Asset, asset_to: &Asset) -> f64 {
    asset_from.price_usd / asset_to.price_usd
}


pub fn currency_ratio_btc(asset_from: &Asset, asset_to: &Asset) -> f64 {
    asset_from.price_btc / asset_to.price_btc
}
