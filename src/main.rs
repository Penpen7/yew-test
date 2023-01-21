use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! { <div> { "hello world" } </div>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
