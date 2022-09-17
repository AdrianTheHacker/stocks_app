use reqwest;
use serde::{Serialize, Deserialize};

 
#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceResponse {
    pub symbol: String,
    pub price: String
}


pub async fn grab_crypto_data(coin: &str) -> BinanceResponse {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}USDT", coin);       // URL used to grab data of a given crypto currency from binance API.

    let binance_response: BinanceResponse = tokio::task::spawn_blocking(move || {               // Allows rocket to still run without breaking runtime.
        let res: BinanceResponse = reqwest::blocking::get(url).expect("Can't access api")       // Makes request to the API.
                                                              .json()                           // Coverts the data to a JSON format.
                                                              .unwrap();                        // Allow the data to be written as a BinanceResponse.
        res
    }).await.unwrap();

    println!("{:?}", &binance_response);        // Prints the data stored in binance_response to the console.

    return binance_response;        // Returns the data as a BinanceResponse.
}
