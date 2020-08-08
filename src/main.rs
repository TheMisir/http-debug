use std::env;
use tiny_http::{Server, Response};

fn get_port() -> u16 {
  env::var("PORT")
    .or(Ok("8080".to_string()))
    .and_then(|s| s.parse::<u16>())
    .or::<u16>(Ok(8080))
    .unwrap()
}

fn main() {
  let port = get_port();
  let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

  println!("Server is running at {}", server.server_addr());

  for request in server.incoming_requests() {
    // convert headers to key: value format
    let headers: Vec<_> = request.headers()
      .into_iter()
      .map(|header| format!("{}: {}",
        header.field.as_str(),
        header.value.as_str()
      ))
      .collect();

    // format body
    let body = &format!("{} {} HTTP/{}\n{}\n",
      request.method(),
      request.url(),
      request.http_version(),
      headers.join("\n")
    );

    // create response from body
    let response = Response::from_string(body);

    // print and respond
    println!("{}", body);
    request.respond(response).unwrap();
  }
}
