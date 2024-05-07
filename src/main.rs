use std::env; 
fn main(){
    let args: Vec<_> = env::args().collect();
    let mut data: Vec<u8> = vec![0; 30_000];
    let mut data_ptr: i32 = 0;
    let mut inst_ptr: i32 = 0;

    let instructions: String = if args.len() > 1 {
        args[1].clone()
    } else {
        panic!("no instructions found");
    };

    while inst_ptr < instructions.len() as i32{

    }
    
}