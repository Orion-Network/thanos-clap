use text_io::read;

use dotenv::dotenv;
use std::env;

mod packbuilder;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }

    loop {
        println!("1) Pack Operations");

        let cat: u32 = read!();

        match cat {
            1 => packbuilder::choose_action(),
            _ => {
                println!("A plus tard !");
                break;
            }
        }
    }
}
