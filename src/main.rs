use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
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
    let todos = use_state(|| todo_list);
    html! {
        <>
            <Header/>
            <InputArea on_add={let todos = todos.clone();Callback::from(move |e:MouseEvent| {let mut newtodos = (*todos).clone(); newtodos.append(TodoModel{title:x, has_done:false});todos.set(newtodos);})}/>
            <TodoList todos={(*todos).clone()}/>
        </>
    }
}

#[function_component]
fn Header() -> Html {
    html! {
        <h1>{ "Todo" }</h1>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct InputAreaProps {
    on_add: Callback<MouseEvent>,
}

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
            <input type="text" value={(*title).clone()} onchange={on_changed}/>
            <button onclick={props.on_add.clone()}>{"追加する"}</button>
        </>
    }
}

#[derive(Clone, PartialEq)]
struct TodoModel {
    pub title: String,
    pub has_done: bool,
}

#[derive(Properties, PartialEq, Clone)]
struct TodoListProps {
    todos: Vec<TodoModel>,
}

#[function_component]
fn TodoList(props: &TodoListProps) -> Html {
    html! {
        <ul>
        {props.todos.iter().map(|x| html!{
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
