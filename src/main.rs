mod utils;
mod words;

use crate::utils::get_random_element;
use crate::words::{FIRST_PARTS, SECOND_PARTS, THIRD_PARTS};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::spawn;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream
        .read(&mut buffer)
        .await
        .expect("Failed to read from stream");

    if bytes_read > 0 {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";

        let contents: String = (0..10)
            .map(|_| {
                let first_part = get_random_element(&FIRST_PARTS);
                let second_part = get_random_element(&SECOND_PARTS);
                let third_part = get_random_element(&THIRD_PARTS);

                format!("{} {} {}\r\n", first_part, second_part, third_part)
            })
            .collect();

        let response = format!("{}{}", status_line, contents);
        stream
            .write_all(response.as_bytes())
            .await
            .expect("Failed to write to stream");
        stream.flush().await.expect("Failed to flush stream");
    }
}
