use futures_util::future;
use http::{header, StatusCode};
use http::response::Builder as ResponseBuilder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use hyper_staticfile::Static;
use std::io::Error as IoError;
use std::path::Path;

async fn handle_request<B>(req: Request<B>, static_: Static) -> Result<Response<Body>, IoError> {
    if req.uri().path() == "/" {
        let res = ResponseBuilder::new()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(header::LOCATION, "/index.html")
            .header(header::CACHE_CONTROL, "no-store")
            .body(Body::empty())
            .expect("unable to build response");
        Ok(res)
    } else {
        static_.clone().serve(req).await
    }
}

pub async fn run_server() {
    let mut static_ = Static::new(Path::new("./"));

    static_.cache_headers(Some(0));

    let make_service = make_service_fn(|_| {
        let static_ = static_.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| handle_request(req, static_.clone())))
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = hyper::Server::bind(&addr).serve(make_service);
    eprintln!("Game running on http://{}/index.html", addr);
    webbrowser::open("http://127.0.0.1:3000/index.html").expect("could not open webbrowser");
    server.await.expect("Server failed");
}
