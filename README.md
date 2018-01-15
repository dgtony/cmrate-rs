## CoinMarket Rate

Rewrite of Haskell-based [utility](https://github.com/dgtony/cryptoconv-hs) in Rust. 
Obtain cryptocurrency rates for the given pair from [coinmarketcap.com](https://coinmarketcap.com).

## Install
```
cargo install
```

## Usage

Run converter with names of two crypto-currencies as parameters and it will calculate current ratio and USD prices.

```
> cmrate bitcoin ethereum

USD price ratio: 1 BTC = 10.703481229697852 ETH
BTC price ratio: 1 BTC = 10.60480249085601 ETH

Fiat prices:
1 BTC = 14201.7 USD
1 ETH = 1326.83 USD
>  
```
