use tokio::{io::BufReader, net::{TcpListener, TcpStream}};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use http_server::Page;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7070").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { handle_connection(socket).await; });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next_line().await.unwrap().unwrap();

    let (status_line, path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let page = Page::new(path).await;

    let response =
        format!("{status_line}\r\nContent-Length: {}\r\n\r\n{}", page.length, page.contents);

    stream.write_all(response.as_bytes()).await.unwrap();
    // println!("handled connection")
}