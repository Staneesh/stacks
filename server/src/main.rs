use actix_web::{
    get,
    web::{self, Data},
    App, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

type TimePoint = String;

#[derive(Serialize, Deserialize, Clone)]
struct TimeBox {
    start: TimePoint,
    end: TimePoint,
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    uuid: Uuid,
    title: String,
    description: String,
    scheduled: Vec<TimeBox>,
    deadline: Option<TimePoint>,
}

struct ServerData {
    tasks: Mutex<Vec<Task>>,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, _server_data: Data<Arc<ServerData>>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/tasks_all")]
async fn tasks_all(server_data: Data<Arc<ServerData>>) -> impl Responder {
    let tasks_lock = server_data.tasks.lock().await;
    let tasks_json = serde_json::to_string(&*tasks_lock).unwrap();
    std::mem::drop(tasks_lock);
    return tasks_json;
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let t1: Task = Task {
        uuid: Uuid::new_v4(),
        title: "Task 1".to_string(),
        description: "This is a task".to_string(),
        scheduled: vec![],
        deadline: Some("dzis".to_string()),
    };

    println!("{}", serde_json::to_string(&t1).unwrap());

    let server_data = Arc::new(ServerData {
        tasks: Mutex::new(vec![
            t1.clone(),
            t1.clone(),
            t1.clone(),
            t1.clone(),
            t1.clone(),
            t1.clone(),
            t1.clone(),
        ]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(server_data.clone()))
            .service(greet)
            .service(tasks_all)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
