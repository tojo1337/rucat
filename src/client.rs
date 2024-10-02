use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    sync::Arc,
};

pub fn handdle_read_client(conn: Arc<TcpStream>) {
    // Add reading properties
    let mut buf = String::new();
    let mut buf_reader = BufReader::new(conn.as_ref());

    loop {
        while buf_reader.read_line(&mut buf).unwrap() > 0 {
            print!("{}", buf);
            buf.clear();
        }
    }
}

pub fn handdle_write_client(conn: Arc<TcpStream>) {
    // Add writing properties
    let mut buf = String::new();
    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        conn.as_ref().write_all(buf.as_bytes()).unwrap();

        conn.as_ref().flush().unwrap();
    }
}
