use yew::prelude::*;
mod components;
mod models;
use components::template::TemplateViewTodoList::TemplateViewTodoList;

#[function_component]
pub fn App() -> Html {
    html! {<TemplateViewTodoList/>}
}
