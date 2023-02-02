use crate::components::organisms::Header::Header;
use crate::components::organisms::InputArea::InputArea;
use crate::components::organisms::TodoList;
use crate::models::TodoModel;
use yew::prelude::*;

#[function_component]
pub fn TemplateViewTodoList() -> Html {
    let todos = use_state(Vec::<TodoModel>::new);
    let on_add = {
        let todos = todos.clone();
        Callback::from(move |title: String| {
            let mut newtodos = (*todos).clone();
            newtodos.push(TodoModel {
                title,
                has_done: false,
            });
            todos.set(newtodos);
        })
    };

    let on_done = {
        let todos = todos.clone();
        Callback::from(move |title: String| {
            let mut newtodos = (*todos).clone();
            newtodos.push(TodoModel {
                title,
                has_done: false,
            });
            todos.set(newtodos);
        })
    };

    html! {
        <>
            <Header/>
            <InputArea on_add={on_add}/>
            <TodoList todos={(*todos).clone()}/>
        </>
    }
}
