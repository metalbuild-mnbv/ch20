use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs::read_to_string, time::Duration, thread};
use ch20::Threadpool;
fn main() {
    let listener = TcpListener::bind("0.0.0.0:45678").unwrap();
    let pool = Threadpool::new(4);
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute( || {handle_connection(stream)});
        
        }
    
    
        println!("Main thread Shutting down.");
    
}

fn handle_connection(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);
    let request_line = buf.lines().next().unwrap().unwrap();
    
    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
        let content = read_to_string(file_name).unwrap();
        let len = content.len();
        let response = format!(
            "{status_line}\r\nContent length: {len}\r\n\r\n{content}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    
}