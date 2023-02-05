use crate::models::Message;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct TodoListItemProps {
    pub title: String,
    pub is_checked: bool,
    pub onChangeCheckbox: Callback<Message>,
    pub id: usize,
}

#[function_component]
pub fn TodoListItem(props: &TodoListItemProps) -> Html {
    log::info!("{:?}", props);
    let on_changed = {
        let onChangeCheckbox = props.onChangeCheckbox.clone();
        let id = props.id;
        Callback::from(move |e: Event| {
            e.prevent_default();
            let input = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                if input.checked() {
                    onChangeCheckbox.emit(Message::Done(id));
                } else {
                    onChangeCheckbox.emit(Message::Doing(id));
                }
            }
        })
    };

    html! {
        <li key={props.id}>
            <input type="checkbox" checked={props.is_checked} onchange={on_changed}/>
            {props.title.clone()}
        </li>
    }
}
