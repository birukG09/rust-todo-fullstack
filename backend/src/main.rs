use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    description: String,
    done: bool,
    priority: u8,
}

struct AppState {
    tasks: Mutex<Vec<Task>>,
}

impl AppState {
    fn load_tasks() -> Vec<Task> {
        if let Ok(file) = File::open("tasks.json") {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        }
    }

    fn save_tasks(tasks: &Vec<Task>) {
        if let Ok(file) = OpenOptions::new().write(true).create(true).truncate(true).open("tasks.json") {
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, tasks).expect("Failed to write tasks");
        }
    }
}

#[derive(Deserialize)]
struct CreateTask {
    description: String,
    priority: u8,
}

async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(&*tasks)
}

async fn add_task(data: web::Data<AppState>, new_task: web::Json<CreateTask>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let id = if let Some(last) = tasks.last() {
        last.id + 1
    } else {
        1
    };
    let task = Task {
        id,
        description: new_task.description.clone(),
        done: false,
        priority: new_task.priority,
    };
    tasks.push(task.clone());
    AppState::save_tasks(&tasks);
    HttpResponse::Ok().json(task)
}

async fn toggle_done(data: web::Data<AppState>, web::Path(id): web::Path<usize>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = !task.done;
        AppState::save_tasks(&tasks);
        HttpResponse::Ok().json(task.clone())
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

async fn remove_task(data: web::Data<AppState>, web::Path(id): web::Path<usize>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let len_before = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() < len_before {
        AppState::save_tasks(&tasks);
        HttpResponse::Ok().body("Task removed")
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tasks = AppState::load_tasks();
    let app_state = web::Data::new(AppState {
        tasks: Mutex::new(tasks),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/tasks", web::get().to(list_tasks))
            .route("/tasks", web::post().to(add_task))
            .route("/tasks/{id}/toggle", web::post().to(toggle_done))
            .route("/tasks/{id}", web::delete().to(remove_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
