use clap::{CommandFactory, Parser};
use rucat::{client_connect, server_listen};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[arg(long, short)]
    server: Option<String>,
    #[arg(long, short, num_args=1..)]
    client: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    if let Some(port) = args.server {
        // Add the configuration for server
        server_listen(port);
    } else if let Some(arr) = args.client {
        // Add configuration for client
        let host = &arr[0];
        let port = &arr[1];
        client_connect(host.to_owned(), port.to_owned());
    } else {
        let help_text = Args::command().render_long_help().to_string();
        println!("{}", help_text);
    }
}
