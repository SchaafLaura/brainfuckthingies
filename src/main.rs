use std::env; 
fn main(){
    let args: Vec<_> = env::args().collect();
    match args.len() {
        0 | 1 => {eprintln!("no instructions found");},
        _ => parse(args[1].clone())
    }
}

fn parse(instructions: String){
    let mut data: Vec<u8> = vec![0; 30_000];
    let mut data_ptr: i32 = 0;
    let mut inst_ptr: i32 = 0;
    while inst_ptr < instructions.len() as i32{

    }
}