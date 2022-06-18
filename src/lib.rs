#[macro_use]
extern crate rocket;

mod tera;
mod todo;

use shuttle_service::SecretStore;
use shuttle_service::error::CustomError;
use rocket::response::status::BadRequest;
use rocket::State;

use sqlx::{Executor, PgPool};

use rocket_dyn_templates::Template;
use rocket::response::content::RawHtml;

pub struct MyState {
    pool: PgPool,
}

#[get("/secret")]
async fn secret(state: &State<MyState>) -> Result<String, BadRequest<String>> {
    // get secret defined in `Secrets.toml` file.
    state.pool.get_secret("MY_API_KEY").await.map_err(|e| BadRequest(Some(e.to_string())))
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"See <a href="/home">Home</a>"#)
}


#[shuttle_service::main]
async fn rocket(pool: PgPool) -> shuttle_service::ShuttleRocket {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = MyState { pool };
    let rocket = rocket::build()
        .mount("/", routes![secret, index])
        .mount("/todo", routes![todo::retrieve, todo::add, todo::hello, todo::retrieve_all])
        .mount("/tera", routes![tera::index, tera::hello, tera::about])
        .register("/tera", catchers![tera::not_found])
        .attach(Template::custom(|engines| {
            tera::customize(&mut engines.tera);
        }))
        .manage(state);

    Ok(rocket)
}




