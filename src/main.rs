// Librerías necesarias
use hyper::{
    body::Bytes,
    server::conn::http1,
    service::service_fn,
    {Request, Response, Method, StatusCode}
};

use http_body_util::{
    combinators::BoxBody, 
    BodyExt,
    Empty, 
    Full
};

use std::{
    io::{self, Error, ErrorKind}, 
    net::SocketAddr, 
    path::Path,
    fs
};

use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const INDEX: &str = "/home/archi/Codigos/RustHTTPServer/templates/index.html";


//use hyper::body::Frame;


// Funcion de ejemplo
async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::new(BoxBody::new(full("Hello, World!"))))

}

fn html_to_bytes(ruta:&Path) -> io::Result<String> {
    match fs::read_to_string(ruta)  {
        Ok(contenido_html) => Ok(contenido_html),
        Err(_) => Err(Error::new(ErrorKind::Other, "No se pudo cargar el documento HTML")),
    }
}


// Tabla para hacer el ruteo
async fn routing(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            match html_to_bytes(Path::new(&INDEX)) {
                Ok(html) => Ok(
                    Response::builder()
                        .header("Content-Type", "text/html; charset=utf-8")
                        .status(StatusCode::OK)
                        .body(full(html))
                        .unwrap()
                ),
                
                Err(error) => Ok(
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(full(error.to_string()))
                        .unwrap()
                ),
            }
        },

        (&Method::GET, "/hello") => hello(req).await,

        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
  

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(routing))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
