use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Iniciar el servidor
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server started at port 8080");

    // Escuchar por conexiones
    listener.incoming().for_each(|stream| {
        let stream = stream.unwrap();
        println!("Connection established! {}", stream.peer_addr().unwrap());

        handle_connection(stream);
    });
}

// Manejar la conexion
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Stream received: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        send_to_client(stream);
    } else {
        send_404(stream);
    }
}

// Enviar respuesta exitosa al cliente
fn send_to_client(stream: TcpStream) {
    let content = fs::read_to_string("index.html").unwrap();
    let response = build_response(content);

    send_stream(stream, response);
}

// Enviar respuesta de error 404 al cliente
fn send_404(stream: TcpStream) {
    let content = fs::read_to_string("404.html").unwrap();
    let response = build_response(content);

    send_stream(stream, response);
}

// Construir la respuesta HTTP
fn build_response(content: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}

// Enviar la respuesta al cliente
fn send_stream(mut stream: TcpStream, response: String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
 