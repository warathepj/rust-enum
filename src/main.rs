use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template; // This is correct, but we need to ensure the crate is properly configured

// Define your enum
#[derive(Debug, Clone)]
enum TaskCategory {
    Work,
    Personal,
    Hobby,
}

// Add methods to the enum
impl TaskCategory {
    fn class_name(&self) -> &'static str {
        match self {
            TaskCategory::Work => "work-task",
            TaskCategory::Personal => "personal-task",
            TaskCategory::Hobby => "hobby-task",
        }
    }
}

// Implement Display for pretty printing
impl std::fmt::Display for TaskCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TaskCategory::Work => "Work",
                TaskCategory::Personal => "Personal",
                TaskCategory::Hobby => "Hobby",
            }
        )
    }
}

// Define task structure
#[derive(Debug, Clone)]
struct Task {
    title: String,
    category: TaskCategory,
}

// Template structure
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    tasks: Vec<Task>,
}

// Handler function
#[get("/")]
async fn index() -> impl Responder {
    let tasks = vec![
        Task {
            title: "Finish project".to_string(),
            category: TaskCategory::Work,
        },
        Task {
            title: "Buy groceries".to_string(),
            category: TaskCategory::Personal,
        },
        Task {
            title: "Play guitar".to_string(),
            category: TaskCategory::Hobby,
        },
    ];

    let template = IndexTemplate { tasks };
    match template.render() {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}