#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use std::env;
use sqlx::PgPool;
use rocket::fs::NamedFile;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewActivity {
    name: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
struct Activity {
    id: Uuid,
    name: String,
}

#[post("/add", format = "json", data = "<activity>")]
async fn add_activity(db: &rocket::State<PgPool>, activity: Json<NewActivity>) -> Json<Activity> {
    let record = sqlx::query_as::<_, Activity>(
        r#"
        INSERT INTO activities (name)
        VALUES ($1)
        RETURNING id, name
        "#
    )
    .bind(&activity.name)
    .fetch_one(db.inner())
    .await
    .expect("Failed to insert activity");

    Json(record)
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/list")]
async fn list_activities(db: &rocket::State<sqlx::PgPool>) -> Json<Vec<Activity>> {
    let rows = sqlx::query_as::<_, Activity>(
        r#"
        SELECT id, name FROM activities
        ORDER BY name
        "#
    )
    .fetch_all(db.inner())
    .await
    .expect("Failed to fetch activities");

    Json(rows)
}

#[launch]
async fn rocket() -> _ {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    rocket::build()
        .manage(db)
        .mount("/", routes![index, add_activity, list_activities])
}

