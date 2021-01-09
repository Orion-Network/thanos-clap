pub mod buildpack;

use text_io::read;

pub fn choose_action() {
    println!("1)Build Pack\n2)Check Pack\n3)Get Info on Pack\n4)Help");
    let cat:u32 = read!();

    println!("{}", cat);
}