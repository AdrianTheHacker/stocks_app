use yew::prelude::*;

pub mod binance;


#[derive(Properties, PartialEq)]
struct CoinListProps {
    coin_list: Vec<binance::CryptoData>,
}


#[function_component(CoinList)]
fn coin_list(CoinListProps { coin_list }: &CoinListProps) -> Html {
    coin_list.iter().map(|coin| html! {
        <tr>
            <td id="coin-symbol">{ &coin.symbol }</td>
            <td id="coin-price">{ &coin.price }</td>
        </tr>
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
                let fetched_coin_list: Vec<binance::CryptoData> = Vec::from([binance::grab_crypto_data("BTC".to_string()).await,
                                                                             binance::grab_crypto_data("ETH".to_string()).await,
                                                                             binance::grab_crypto_data("DOGE".to_string()).await]);
                    
                coin_list.set(fetched_coin_list);
            });
            || ()}, ());
    }

    html! {
        <>  
            <div id="content-box">
                <h1>{ "Crypto Ship" }</h1>
                <div id="coin-table">
                    <table>
                        <tr>
                            <th>{ "Coin" }</th>
                            <th>{ "Value (USDT)" }</th>
                        </tr>
                        <CoinList coin_list={(*coin_list).clone()}/>
                    </table>
                </div>
            </div>
        </>
    }
}


fn main() {
    yew::start_app::<App>();
}
