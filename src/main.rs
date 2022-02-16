#![feature(option_result_contains)]

use std::io;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use cities::countries;

async fn find(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("").to_lowercase();

    let mut nodes = countries::get_data();
    nodes.retain(|node| node.name.to_lowercase().starts_with(&name));

    // limit to 5 results
    nodes.truncate(5);

    web::Json(nodes)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://0.0.0.0:30229");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(find))
            .route("/favicon.ico", web::get().to(|| async { HttpResponse::Ok().body("") }))
            .route("/{name}", web::get().to(find))
    })
        .bind(("0.0.0.0", 30229))?
        .run()
        .await?;

    Ok(())
}

/*
Use when you want to parse OSM file to JSON and then to Rust file.

fn main() -> io::Result<()> {
     cities::processor::to_static_file()?;
     Ok(())
}
*/