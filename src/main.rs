mod model;

use model::{ Blockchain };

fn main() {
    let mut chain = Blockchain::new(4);
    for i in 1..5 {
        chain.new_block(
            String::from(format!("Block{}",i))
        );
    }
    let confirm: String;
    match chain.is_chain_valid() {
        true => confirm = String::from("Yes Sir !"),
        false => confirm = String::from("No, it is broken...")
    }
    println!("Is this blockchain valid ? {} ",confirm);
}
