extern crate clap;
use clap::App;

fn main() {
    let _ = App::new("foo").version("0.1.0").get_matches();
    println!("Hello, world!");
}
