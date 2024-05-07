use std::env;

fn main(){
    let args: Vec<_> = env::args().collect();
    let instructions: String;
    if args.len() > 1 {
        instructions = args[1].clone();
    } else {
        println!("no instructions found");
        return;
    }
    // this is a comment
    for (i, c) in instructions.chars().enumerate() {
        println!("{}, {}", i, c);
    }
    
}