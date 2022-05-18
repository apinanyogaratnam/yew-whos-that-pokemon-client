use web_sys::HtmlInputElement;
use yew::prelude::*;
use reqwest;
use serde_json::Value;

fn main() {
    yew::start_app::<Root>();
}

#[derive(PartialEq, Debug, Clone)]
struct Pokemon {
    id: usize,
    name: String,
    image_src: String,
}

#[function_component(Root)]
fn root() -> Html {
    let pokemon_state = use_state_eq::<Option<Pokemon>, _>(|| None);
    web_sys::console::log_1(&format!("{:?}", pokemon_state).into());
    let pokemon_state_outer = pokemon_state.clone();

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
            <ViewPokemon pokemon={(*pokemon_state_outer).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ViewPokemonProps {
    pokemon: Option<Pokemon>,
}

#[derive(PartialEq, Debug, Clone)]
enum Guess {
    Correct,
    Incorrect,
}

#[function_component(ViewPokemon)]
fn view_pokemon(props: &ViewPokemonProps) -> Html {
    let pokemon = match &props.pokemon {
        Some(p) => p,
        None => return html!{},
    };

    let guess_state = use_state_eq::<Option<Guess>, _>(|| None);
    let guess_state_outer = guess_state.clone();

    let name = pokemon.name.clone();
    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();
    let onclick = Callback::from(move |_| {
        let input: HtmlInputElement = input_ref.cast::<HtmlInputElement>().unwrap();
        let guess = input.value().to_lowercase();

        if guess == name {
            guess_state.set(Some(Guess::Correct));
        } else {
            guess_state.set(Some(Guess::Incorrect));
        }

        web_sys::console::log_1(&guess.into());
    });

    html! {
        <div>
            <img src={pokemon.image_src.clone()} />
            <input ref={input_ref_outer.clone()} type="text" />
            <button {onclick}>{"submit"}</button>
            { format!("{:?}", guess_state_outer) }
        </div>
    }
}
