use serde::Deserialize;
use gloo_net::http::Request;


#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct CryptoData {
    pub symbol: String,
    pub price: String
}


pub async fn grab_crypto_data(coin: String) -> CryptoData {
    let url: String = format!("https://api.binance.com/api/v3/ticker/price?symbol={}USDT", coin);       // URL to used for making API requests.

    let response: CryptoData = Request::get(url.as_str())       // Used to make the HTTP requests.
                .send()                                         // This works similiar to reqwests::blocking::get.
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

    response        // Returns the HTTP request as CryptoData.
}
