use crate::models::{Message, TodoModel};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputAreaProps {
    pub on_add: Callback<Message>,
}

#[function_component]
pub fn InputArea(props: &InputAreaProps) -> Html {
    let title = use_state(|| "".to_string());
    let on_changed = {
        let title = title.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let input = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                title.set(input.value());
            }
        })
    };
    let on_add = props.on_add.clone();

    html! {
        <>
            <input type="text" value={(*title).clone()} onchange={on_changed}/>
            <button onclick={move|_:MouseEvent| on_add.emit(Message::New(TodoModel::new((*title).clone())))}>{"追加する"}</button>
        </>
    }
}
