use actix_files as fs;
use actix_web::{
    web, App, HttpResponse, HttpServer, Responder, middleware,
};
use askama::Template;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;
use log::{error, info};

// Constants
const UPLOAD_DIR: &str = "./uploads/";
const THUMB_DIR: &str = "./thumbs/";

// Template Structures

#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate<'a> {
    threads: &'a [Thread],
    current_page: i32,
    total_pages: i32,
}

// Thread view template
#[derive(Template)]
#[template(path = "thread.html")]
struct ThreadTemplate<'a> {
    thread: &'a Thread,
    replies: &'a [Reply],
}

// Models for threads, replies, and forms

#[derive(Serialize, Deserialize, Clone)]
struct Thread {
    id: i32,
    title: String,
    message: String,
    last_updated: i64, // Unix timestamp
}

#[derive(Serialize, Deserialize)]
struct Reply {
    id: i32,
    message: String,
}

#[derive(Deserialize)]
struct NewThreadForm {
    title: String,
    message: String,
}

#[derive(Deserialize)]
struct NewReplyForm {
    parent_id: i32,
    message: String,
}

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<i32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init();

    // Ensure directories exist
    for dir in &[UPLOAD_DIR, THUMB_DIR] {
        if !std::path::Path::new(dir).exists() {
            std::fs::create_dir_all(dir).unwrap();
            info!("Created directory: {}", dir);
        }
    }

    // Initialize sled database
    let sled_db = Arc::new(sled::open("sled_db").expect("Failed to open sled database"));

    // Start Actix server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sled_db.clone()))
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(homepage))
            .route("/thread/{id}", web::get().to(view_thread))
            .route("/thread", web::post().to(create_thread))
            .route("/reply", web::post().to(create_reply))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// Homepage handler
async fn homepage(
    db: web::Data<Arc<Db>>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page_size = 10;
    let page_number = query.page.unwrap_or(1);

    // Fetch all threads and sort by `last_updated`
    let mut threads = get_all_threads(&db);
    threads.sort_by(|a, b| b.last_updated.cmp(&a.last_updated));

    // Calculate pagination
    let total_threads = threads.len() as i32;
    let total_pages = (total_threads as f64 / page_size as f64).ceil() as i32;

    // Validate page_number
    let page_number = if page_number < 1 {
        1
    } else if page_number > total_pages && total_pages > 0 {
        total_pages
    } else {
        page_number
    };

    // Apply pagination to threads
    let start_index = ((page_number - 1) * page_size) as usize;
    let end_index = (start_index + page_size as usize).min(threads.len());
    let threads = &threads[start_index..end_index];

    let tmpl = HomepageTemplate {
        threads,
        current_page: page_number,
        total_pages,
    };

    match tmpl.render() {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(e) => {
            error!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Error rendering page")
        }
    }
}

// Fetch all threads from sled
fn get_all_threads(db: &Db) -> Vec<Thread> {
    db.scan_prefix(b"thread_")
        .filter_map(|res| {
            if let Ok((_, value)) = res {
                serde_json::from_slice(&value).ok()
            } else {
                None
            }
        })
        .collect()
}

// Count total number of threads in sled
fn count_threads(db: &Db) -> i32 {
    db.scan_prefix(b"thread_").count() as i32
}

// Thread viewing handler
async fn view_thread(
    db: web::Data<Arc<Db>>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    let thread_id = path.into_inner().0;
    let thread_key = format!("thread_{}", thread_id).into_bytes();
    let thread: Option<Thread> = db.get(&thread_key).ok().flatten().and_then(|value| {
        serde_json::from_slice(&value).ok()
    });

    if thread.is_none() {
        return HttpResponse::NotFound().body("Thread not found");
    }

    let thread = thread.unwrap();
    let replies = get_replies(&db, thread_id);

    let tmpl = ThreadTemplate {
        thread: &thread,
        replies: &replies,
    };

    match tmpl.render() {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(e) => {
            error!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Error rendering page")
        }
    }
}

// Create thread handler
async fn create_thread(
    db: web::Data<Arc<Db>>,
    form: web::Form<NewThreadForm>,
) -> impl Responder {
    let thread_id = count_threads(&db) + 1;
    let thread = Thread {
        id: thread_id,
        title: form.title.clone(),
        message: form.message.clone(),
        last_updated: Utc::now().timestamp(),
    };

    let key = format!("thread_{}", thread_id).into_bytes();
    let value = serde_json::to_vec(&thread).expect("Failed to serialize thread");

    if db.insert(key, value).is_ok() {
        HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .finish()
    } else {
        error!("Failed to insert thread into sled db");
        HttpResponse::InternalServerError().body("Failed to create thread")
    }
}

// Create reply handler
async fn create_reply(
    db: web::Data<Arc<Db>>,
    form: web::Form<NewReplyForm>,
) -> impl Responder {
    let reply_id = count_replies(&db, form.parent_id) + 1;
    let reply = Reply {
        id: reply_id,
        message: form.message.clone(),
    };

    let key = format!("reply_{}_{}", form.parent_id, reply_id).into_bytes();
    let value = serde_json::to_vec(&reply).expect("Failed to serialize reply");

    if db.insert(key, value).is_ok() {
        // Update thread's last_updated
        let thread_key = format!("thread_{}", form.parent_id).into_bytes();
        if let Some(thread_bytes) = db.get(&thread_key).ok().flatten() {
            if let Ok(mut thread) = serde_json::from_slice::<Thread>(&thread_bytes) {
                thread.last_updated = Utc::now().timestamp();
                let updated = serde_json::to_vec(&thread).expect("Failed to serialize updated thread");
                db.insert(thread_key, updated).ok();
            }
        }

        HttpResponse::SeeOther()
            .append_header(("Location", format!("/thread/{}", form.parent_id)))
            .finish()
    } else {
        error!("Failed to insert reply into sled db");
        HttpResponse::InternalServerError().body("Failed to post reply")
    }
}

// Fetch replies for a thread from sled
fn get_replies(db: &Db, parent_id: i32) -> Vec<Reply> {
    db.scan_prefix(format!("reply_{}", parent_id).as_bytes())
        .filter_map(|res| {
            if let Ok((_, value)) = res {
                serde_json::from_slice(&value).ok()
            } else {
                None
            }
        })
        .collect()
}

// Count total number of replies for a thread in sled
fn count_replies(db: &Db, parent_id: i32) -> i32 {
    db.scan_prefix(format!("reply_{}", parent_id).as_bytes()).count() as i32
}

