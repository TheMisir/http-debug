use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Version};
use hyper::service::{make_service_fn, service_fn};

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let uri = req.uri();
  let method = req.method().as_str();
  let version = match req.version() {
    Version::HTTP_09 => "HTTP/0.9",
    Version::HTTP_10 => "HTTP/1.0",
    Version::HTTP_11 => "HTTP/1.1",
    Version::HTTP_2 => "HTTP/2.0",
    Version::HTTP_3 => "HTTP/3.0",
    _ => "HTTP"
  };

  println!("{} {} {}", method, uri, version);

  for (key, value) in req.headers() {
    println!("{}: {}", key, value.to_str().unwrap());
  }
  
  println!();
  
  Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
  let port = std::env::var("PORT")
    .or(Ok("8080".to_string()))
    .and_then(|st| {
      st.parse::<u16>()
    })
    .or::<u16>(Ok(8080))
    .unwrap();
  
  let addr = SocketAddr::from(([127, 0, 0, 1], port));
  let make_svc = make_service_fn(|_conn| async {
    Ok::<_, Infallible>(service_fn(handle_request))
  });

  let server = Server::bind(&addr).serve(make_svc);

  println!("Server is running at http://127.0.0.1:{}", port);

  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
}