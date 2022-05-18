use rand::Rng;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use reqwest;
use serde_json::Value;

fn main() {
    yew::start_app::<Root>();
}

#[derive(PartialEq, Debug, Clone)]
struct Pokemon {
    id: i32,
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
            let mut rng = rand::thread_rng();
            let id: i32 = rng.gen_range(1..=100);


            let response = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{id}"))
                .await
                .unwrap();

            let text = response.text().await.unwrap();

            let v: Value = serde_json::from_str(&text).unwrap();
            let name = v["name"].as_str().unwrap();
            let image_src = v["sprites"]["front_default"].as_str().unwrap();

            let pokemon = Pokemon {
                id,
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
            <ViewGuess guess={(*guess_state_outer).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ViewGuessProps {
    guess: Option<Guess>,
}

#[function_component(ViewGuess)]
fn view_guess(props: &ViewGuessProps) -> Html {
    let text = match &props.guess {
        None => return html!{},
        Some(Guess::Correct) => "Yes! You did it!",
        Some(Guess::Incorrect) => "No, that's wrong :(",
    };

    html! {
        <div>{ text }</div>
    }
}
