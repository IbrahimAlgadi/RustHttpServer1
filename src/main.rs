use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

fn main() {
    let host = "127.0.0.1";
    let port = "7878";
    let listener = TcpListener::bind(format!("{host}:{port}")).unwrap();
    println!("Listening on: http://{host}:{port}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(file_name).unwrap(); 
    let content_length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

}