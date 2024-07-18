extern crate env_logger;
extern crate jupyter_client;

use jupyter_client::commands::{Command, HistoryAccessType};
use jupyter_client::Client;
use std::collections::HashMap;
use std::fs::File;

fn main() {
    env_logger::init();

    let files = File::open("/Users/baber/Library/Jupyter/runtime/kernel-9afc7afd-9807-48e4-8183-68eaa038e1c9.json").expect("file not found");
    let client = Client::from_reader(files).expect("creating jupyter connection");


    // Set up some previous code
    let code = r#"class Foo(object):
    def bar(self):
        return 10

    def baz(self):
        return 20
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

    // tail history
    let command = Command::History {
        output: true,
        raw: false,
        hist_access_type: HistoryAccessType::Tail { n: 10 },
        unique: false,
    };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);

    // range history
    let command = Command::History {
        output: true,
        raw: false,
        hist_access_type: HistoryAccessType::Range {
            session: -2,
            start: 0,
            stop: 15,
        },
        unique: false,
    };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);

    // search history
    let command = Command::History {
        output: true,
        raw: false,
        hist_access_type: HistoryAccessType::Search {
            pattern: "def bar*".to_string(),
        },
        unique: false,
    };
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);
}
