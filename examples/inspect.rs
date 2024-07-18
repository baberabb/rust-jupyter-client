extern crate env_logger;
extern crate jupyter_client;

use jupyter_client::commands::{Command, DetailLevel};
use jupyter_client::responses::{Response, ShellResponse};
use jupyter_client::Client;
use std::collections::HashMap;
use std::fs::File;

fn main() {
    env_logger::init();

    let files = File::open("/Users/baber/Library/Jupyter/runtime/kernel-9afc7afd-9807-48e4-8183-68eaa038e1c9.json").expect("file not found");
    let client = Client::from_reader(files).expect("creating jupyter connection");


    // Set up some previous code
    let code = r#"class Foo(object):
    """Foo class"""
    pass
"#
    .to_string();
    let prep_cmd = Command::Execute {
        code: code,
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };

    client
        .send_shell_command(prep_cmd)
        .expect("sending command");

    let prep_cmd = Command::Execute {
        code: "a = Foo()".to_string(),
        silent: false,
        store_history: true,
        user_expressions: HashMap::new(),
        allow_stdin: true,
        stop_on_error: false,
    };

    client
        .send_shell_command(prep_cmd)
        .expect("sending command");

    let command = Command::Inspect {
        code: "a".to_string(),
        cursor_pos: 1,
        detail_level: DetailLevel::Zero,
    };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);

    // Get some more detail and print the help
    if let Response::Shell(ShellResponse::Inspect { content, .. }) = response {
        if content.found {
            println!(
                "\nHelp:\n\n{}",
                content.data["text/plain"].as_str().unwrap()
            );
        }
    }
}
