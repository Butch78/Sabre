
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket::State;

use sqlx::FromRow;

use rocket_dyn_templates::{Template, context};

use crate::MyState;


#[derive(Deserialize)]
pub struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub note: String,
}



#[get("/<id>")]
pub async fn retrieve(id: i32, state: &State<MyState>) -> Result<Json<Todo>, BadRequest<String>> {
    let todo = sqlx::query_as("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(todo))
}


async fn get_todos(state: &State<MyState>) -> Result<Vec<Todo>, BadRequest<String>> {
    let todos = sqlx::query_as("SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(todos)
}

#[get("/")]
pub async fn retrieve_all(state: &State<MyState>) -> Result<Json<Vec<Todo>>, BadRequest<String>> {
    let todos = get_todos(&state).await?;

    Ok(Json(todos))
}



#[get("/hello/<name>")]
pub async fn hello(
    name: &str, 
    state: &State<MyState>) -> Result<Template, BadRequest<String>> {

    let todos = get_todos(&state).await?;

    Ok(Template::render(
        "tera/todo",
        context! {
            title: "Todos",
            name: Some(name),
            todos: todos,
        },
    ))

}




#[post("/", data = "<data>")]
pub async fn add(
    data: Json<TodoNew>,
    state: &State<MyState>,
) -> Result<Json<Todo>, BadRequest<String>> {
    let todo = sqlx::query_as("INSERT INTO todos(note) VALUES ($1) RETURNING id, note")
        .bind(&data.note)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok(Json(todo))
}



