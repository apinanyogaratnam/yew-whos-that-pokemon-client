use yew::prelude::*;

fn main() {
    yew::start_app::<Root>();
}

#[function_component(Root)]
fn root() -> Html {
    let onclick = Callback::from(|mouse_event: MouseEvent| {
        web_sys::console::log_1(&mouse_event);
    });

    html! {
        <div>
            <button {onclick}>{"get pokemon"}</button>
        </div>
    }
}
