use crate::components::organisms::Header::Header;
use crate::components::organisms::InputArea::InputArea;
use crate::components::organisms::TodoList;
use crate::models::{Message, TodoModel};
use yew::prelude::*;

#[function_component]
pub fn TemplateViewTodoList() -> Html {
    let todos = use_state(Vec::<TodoModel>::new);
    let on_add = {
        let todos = todos.clone();
        Callback::from(move |message: Message| {
            let mut newtodos = (*todos).clone();
            match message {
                Message::New(todo) => {
                    newtodos.push(todo);
                }
                Message::Done(id) => {
                    newtodos[id].has_done = true;
                }
                Message::Doing(id) => {
                    newtodos[id].has_done = false;
                }
            };
            todos.set(newtodos);
        })
    };

    html! {
        <>
            <Header/>
            <InputArea on_add={on_add.clone()}/>
            <TodoList todos={(*todos).clone()} done={on_add}/>
        </>
    }
}
