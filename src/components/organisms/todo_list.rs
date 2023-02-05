use crate::components::molecules::TodoListItem::TodoListItem;
use crate::models::{Message, TodoModel};
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct TodoListProps {
    pub todos: Vec<TodoModel>,
    pub done: Callback<Message>,
}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    html! {
        <ul>
        {props.todos.iter().enumerate().map(|(i,x)| html!{
           <TodoListItem title={x.title.clone()} is_checked={x.has_done} onChangeCheckbox={props.done.clone()} id={i}/>
       }).collect::<Html>()}
        </ul>
    }
}
