#[macro_use] extern crate rocket;
mod binance;

#[get("/")]
async fn index() -> &'static str {
    binance::grab_bitcoin().await;

    return {
        "Hello, world!"
    };
}


#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index])
}
