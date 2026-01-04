use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn handle_connection(mut socket: tokio::net::TcpStream) -> io::Result<()> {
	socket.write_all(b"Who are you?\n").await?;
	let mut buf = vec![0; 1024];
	let name_size = socket.read(&mut buf).await?;
	let name = std::str::from_utf8(&buf[..name_size]).unwrap().trim();
	let reply = format!("Thanks for dialing in, {name}!\n");
	socket.write_all(reply.as_bytes()).await?;
	Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:0").await?;
	println!("listening on port {}", listener.local_addr()?.port());

	loop {
		let (socket, addr) = listener.accept().await?;

		println!("connection from {addr:?}");

		tokio::spawn(async move {
			handle_connection(socket).await.unwrap();
		});
	}
}

