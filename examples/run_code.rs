extern crate env_logger;
extern crate jupyter_client;
// extern crate structopt;

use jupyter_client::commands::Command;
use jupyter_client::responses::{Response, ShellResponse, Status};
use jupyter_client::Client;
use std::collections::HashMap;
use std::fs::File;
// use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// struct Opt {
//     #[structopt(name = "command", short = "c")]
//     command: String,
// }

fn main() {
    env_logger::init();

    // let args = Opt::from_args();
    // let files = File::open("/Users/baber/Library/Jupyter/runtime/kernel-8bba2dea-da2e-4782-81f6-1f3ce1268cff.json").expect("file not found");
    let client = Client::existing().expect("creating jupyter connection");



    let command = Command::Execute {
        code: r#"
print("HELOOOoOOOOOOooOOOOOO")"#.to_string(),
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };
    let response = client.send_shell_command(command).expect("sending command");
    if let &Response::Shell(ShellResponse::Execute { ref content, .. }) = &response {
        match content.status {
            Status::Ok | Status::Abort => println!("Response: {:#?}", response),
            Status::Error => {
                eprintln!("Error: {}", content.evalue.as_ref().unwrap());
                for line in content.traceback.as_ref().unwrap() {
                    eprintln!("{}", line);
                }
            }
        }
    } else {
        panic!("unexpected response type");
    }
}
