use yew::prelude::*;

pub mod binance;


#[derive(Properties, PartialEq)]
struct CoinListProps {
    coin_list: Vec<binance::CryptoData>,
}


#[function_component(CoinList)]
fn coin_list(CoinListProps { coin_list }: &CoinListProps) -> Html {
    coin_list.iter().map(|coin| html! {
        <p>{format!("{}: {}", coin.symbol, coin.price)}</p>
    }).collect()
}


#[function_component(App)]
fn app() -> Html {
    let coin_list = use_state(|| vec![]);
    {
        let coin_list = coin_list.clone();

        yew::use_effect_with_deps(move |_| {
            let coin_list = coin_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_coin_list: Vec<binance::CryptoData> = [binance::grab_crypto_data("BTC".to_string()).await,
                                                                   binance::grab_crypto_data("ETH".to_string()).await,
                                                                   binance::grab_crypto_data("DOGE".to_string()).await].to_vec();
                    
                coin_list.set(fetched_coin_list);
            });
            || ()}, ());
    }

    html! {
        <>
            <h1>{ "Crypto Ship" }</h1>
            <div>
                <CoinList coin_list={(*coin_list).clone()}/>
            </div>
        </>
    }
}


fn main() {
    yew::start_app::<App>();
}
