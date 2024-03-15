use shut_up::{cpp_start, trust_me};

fn main() {
    cpp_start! {
        println!("trust me, this is safe!");
    }
    trust_me! {
        println!("trust me, this is safe!");
    }
}