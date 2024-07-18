extern crate env_logger;
extern crate jupyter_client;

use std::fs::File;
use jupyter_client::Client;

fn main() {
    env_logger::init();

    // let files = File::open("/Users/baber/Library/Jupyter/runtime/kernel-8bba2dea-da2e-4782-81f6-1f3ce1268cff.json").expect("file not found");
    let client = Client::existing().expect("creating jupyter connection");


    let receiver = client.iopub_subscribe().unwrap();
    for msg in receiver {
        println!("{:#?}", msg);
    }
}
