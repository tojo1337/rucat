use client::{handdle_read_client, handdle_write_client};
use server::{handle_read, handle_write};
use std::{
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};
use threadpool::ThreadPool;

mod client;
mod server;

pub fn client_connect(host: String, port: String) {
    // Add the client method in here
    let mut addr = String::new();
    addr.push_str(&host);
    addr.push(':');
    addr.push_str(&port);

    let con = TcpStream::connect(addr).unwrap();
    let conn = Arc::new(con);

    let reader = conn.clone();
    let writer = conn.clone();

    let pool = ThreadPool::new(5);

    pool.execute(move || {
        handdle_read_client(reader);
    });
    pool.execute(move || {
        handdle_write_client(writer);
    });

    loop {
        thread::park();
    }
}

pub fn server_listen(port: String) {
    // Add the server method in here
    let port = port.clone();
    let connector = format!("0.0.0.0:{}", port);
    let conn = TcpListener::bind(connector).unwrap();

    let pool = ThreadPool::new(5);

    for stream in conn.incoming() {
        let source = Arc::new(stream.unwrap());
        let reader = source.clone();
        let writer = source.clone();

        pool.execute(move || {
            handle_read(reader);
        });

        pool.execute(move || {
            handle_write(writer);
        });
    }
}
