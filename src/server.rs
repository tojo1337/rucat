use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    sync::Arc,
};

pub fn handle_read(stream: Arc<TcpStream>) {
    let mut buf_reader = BufReader::new(stream.as_ref());
    let mut buf = String::new();

    while buf_reader.read_line(&mut buf).unwrap() > 0 {
        print!("{}", buf);
        buf.clear();
    }
}

pub fn handle_write(stream: Arc<TcpStream>) {
    let mut buf = String::new();

    loop {
        io::stdin().read_line(&mut buf).unwrap();
        stream.as_ref().write_all(buf.as_bytes()).unwrap();

        // Clean up part
        buf.clear();
        stream.as_ref().flush().unwrap();
    }
}
