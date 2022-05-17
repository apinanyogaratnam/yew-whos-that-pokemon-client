use yew::prelude::*;

fn main() {
    yew::start_app::<Root>();
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <div>
            <button>{"get pokemon"}</button>
        </div>
    }
}
