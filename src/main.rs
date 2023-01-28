use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! { <><Header/><InputArea/><TodoList/></> }
}

#[function_component]
fn Header() -> Html {
    html! {
        <h1>{ "Todo" }</h1>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct InputAreaProps {}

#[function_component]
fn InputArea(props: &InputAreaProps) -> Html {
    let title = use_state(|| "".to_string());
    let on_changed = {
        let title = title.clone();
        Callback::from(move |e: Event| {
            let input = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                title.set(input.value());
            }
        })
    };

    html! {
        <>
            <input type="text" value={(*title).clone()} onchange={on_changed}/><button>{"追加する"}</button>
        </>
    }
}

struct TodoModel {
    pub title: String,
    pub has_done: bool,
}

#[function_component]
fn TodoList() -> Html {
    let todo_list = vec![
        TodoModel {
            title: "ご飯を食べる".to_string(),
            has_done: false,
        },
        TodoModel {
            title: "薬を飲む".to_string(),
            has_done: true,
        },
    ];
    html! {
        <ul>
        {todo_list.iter().map(|x| html!{
           <TodoListItem title={x.title.clone()} is_checked={x.has_done}/>
       }).collect::<Html>()}
        </ul>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct TodoListItemProps {
    pub title: String,
    pub is_checked: bool,
}

#[function_component]
fn TodoListItem(props: &TodoListItemProps) -> Html {
    let is_checked = use_state(|| props.is_checked);
    let on_changed = {
        let is_checked = is_checked.clone();
        Callback::from(move |e: Event| {
            let input = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                is_checked.set(input.checked());
            }
        })
    };
    html! { <li><input type="checkbox" checked={*is_checked} onchange={on_changed}/>{props.title.clone()}</li>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}
