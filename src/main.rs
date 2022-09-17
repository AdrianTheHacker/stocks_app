#[macro_use] extern crate rocket;
use rocket_dyn_templates;

mod binance;


#[get("/")]
async fn index() -> rocket_dyn_templates::Template {
    let binance_data = binance::grab_crypto_data("BTC").await;

    rocket_dyn_templates::Template::render("index", rocket_dyn_templates::context! { bitcoin: binance_data.symbol, 
                                                                                                    bitcoin_price: binance_data.price })
}


#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index])
                   .attach(rocket_dyn_templates::Template::fairing())
}
