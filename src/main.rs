extern crate core;

mod cli;

fn main() {
    match cli::start() {
        Ok(_) => (),
        Err(err) => println!("{:?}", err)
    }
}
