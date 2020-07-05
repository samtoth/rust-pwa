use web_view::*;
use std::{borrow::Cow, sync::mpsc, thread};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use actix_web::{body::Body, web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(RustEmbed)]
#[folder = "frontend/static"]
struct Asset;

fn assets(req: HttpRequest) -> HttpResponse {
    let path = if req.path() == "/" {
        // if there is no path, return default file
        "index.html"
    } else {
        // trim leading '/'
        &req.path()[1..]
    };

    // query the file from embedded asset with specified path
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
#[actix_rt::main]
async fn main() {
    let (server_tx, server_rx) = mpsc::channel();
    let (port_tx, port_rx) = mpsc::channel();

    // start actix web server in separate thread
    thread::spawn(move || {
        let sys = actix_rt::System::new("actix-example");

        let server = HttpServer::new(|| App::new().route("*", web::get().to(assets)))
            .bind("127.0.0.1:0")
            .unwrap();

        let port = server.addrs().first().unwrap().port();
        let server = server.run();

        let _ = port_tx.send(port);
        let _ = server_tx.send(server);
        let _ = sys.run();
    });

    let port = port_rx.recv().unwrap();
    let server = server_rx.recv().unwrap();

    let addr = format!("http://127.0.0.1:{}", port);
    println!("serving on {}", addr);

    // start web view in current thread
    // and point it to a port that was bound
    // to the server
    web_view::builder()
        .title("webview yew todomvc")
        .content(Content::Url(addr))
        .size(600, 400)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
    
    let _ = server.stop(true).await;
}

