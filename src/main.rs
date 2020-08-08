use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Version};
use hyper::service::{make_service_fn, service_fn};

fn version_str(ver: &Version) -> &str {
  match *ver {
    Version::HTTP_09 => "HTTP/0.9",
    Version::HTTP_10 => "HTTP/1.0",
    Version::HTTP_11 => "HTTP/1.1",
    Version::HTTP_2 => "HTTP/2.0",
    Version::HTTP_3 => "HTTP/3.0",
    _ => ""
  }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
  println!("{} {} {}", req.method().as_str(), req.uri(), version_str(&req.version()));
  for header in req.headers() {
    println!("{}: {}", header.0, header.1.to_str().unwrap());
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
  
  // We'll bind to 127.0.0.1:${PORT:-8080}
  let addr = SocketAddr::from(([127, 0, 0, 1], port));

  let make_svc = make_service_fn(|_conn| async {
    Ok::<_, Infallible>(service_fn(handle_request))
  });

  let server = Server::bind(&addr).serve(make_svc);

  println!("Server is running at http://127.0.0.1:{}", port);

  // Run this server for... forever!
  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
}