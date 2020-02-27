use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

struct ThreadPool;

impl ThreadPool {
  fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    ThreadPool
  }
  fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {}
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
  };
  let contents = fs::read_to_string(filename).unwrap();
  let response = format!("{}{}", status_line, contents);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7000").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    pool.execute(|| { handle_connection(stream) })
  }
}
  
