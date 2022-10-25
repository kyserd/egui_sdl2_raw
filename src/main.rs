use std::io;
use std::net::TcpListener;
use tracing::info;
use tracing::metadata::LevelFilter;

fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .init();

    info!("Waiting for connection");

    let listener = TcpListener::bind("127.0.0.1:7331")?;

    let (mut stream, addr) = listener.accept()?;

    info!("Connected to {}", addr);

    let mut stdout = io::stdout();
    loop {
        io::copy(&mut stream, &mut stdout)?;
    }
}
