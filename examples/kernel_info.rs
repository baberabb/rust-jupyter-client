use env_logger;
use std::fs::File;
use jupyter_client::commands::Command;
use jupyter_client::Client;

fn main() {
    env_logger::init();

    let files = File::open("/Users/baber/Library/Jupyter/runtime/kernel-7de34064-4eab-43dd-9da1-75fb3a8f9bc7.json").expect("file not found");
    let client = Client::existing().expect("creating jupyter connection");


    let command = Command::KernelInfo;
    let response = client.send_shell_command(command).expect("sending command");
    println!("Response: {:#?}", response);
}
