use yew::prelude::*;

fn main() {
    yew::start_app::<Root>();
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <div>
            <h1>{"Hello, world!"}</h1>
        </div>
    }
}
