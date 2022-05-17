use yew::prelude::*;
use reqwest;
use serde_json::Value;

fn main() {
    yew::start_app::<Root>();
}

#[derive(PartialEq)]
struct Pokemon {
    id: usize,
    name: String,
    image_src: String,
}

#[function_component(Root)]
fn root() -> Html {
    let pokemon_state = use_state_eq::<Option<Pokemon>, _>(|| None);

    let onclick = Callback::from(move |mouse_event: MouseEvent| {
        web_sys::console::log_1(&mouse_event);

        let pokemon_state = pokemon_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get("https://pokeapi.co/api/v2/pokemon/22")
                .await
                .unwrap();

            let text = response.text().await.unwrap();

            let v: Value = serde_json::from_str(&text).unwrap();
            let name = v["name"].as_str().unwrap();
            let image_src = v["sprites"]["front_default"].as_str().unwrap();

            let pokemon = Pokemon {
                id: 22,
                name: name.to_string(),
                image_src: image_src.to_string(),
            };
            pokemon_state.set(Some(pokemon));

            web_sys::console::log_2(&name.into(), &image_src.into());
        });
    });

    html! {
        <div>
            <button {onclick}>{"get pokemon"}</button>
        </div>
    }
}
