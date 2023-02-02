use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct TodoListItemProps {
    pub title: String,
    pub is_checked: bool,
}

#[function_component]
pub fn TodoListItem(props: &TodoListItemProps) -> Html {
    let is_checked = use_state(|| props.is_checked);
    let on_changed = {
        let is_checked = is_checked.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let input = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                is_checked.set(input.checked());
            }
        })
    };
    html! {
        <li>
            <input type="checkbox" checked={*is_checked} onchange={on_changed}/>
            {props.title.clone()}
        </li>
    }
}
