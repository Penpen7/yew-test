use crate::components::molecules::TodoListItem::TodoListItem;
use crate::models::TodoModel;
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct TodoListProps {
    pub todos: Vec<TodoModel>,
}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    html! {
        <ul>
        {props.todos.iter().map(|x| html!{
           <TodoListItem title={x.title.clone()} is_checked={x.has_done}/>
       }).collect::<Html>()}
        </ul>
    }
}
