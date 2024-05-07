use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        0 | 1 => {
            eprintln!("no instructions found");
        }
        _ => parse(args[1].clone()),
    }
}

// brb

/*
> 	Increment the data pointer by one (to point to the next cell to the right).
< 	Decrement the data pointer by one (to point to the next cell to the left).
+ 	Increment the byte at the data pointer by one.
- 	Decrement the byte at the data pointer by one.
. 	Output the byte at the data pointer.
, 	Accept one byte of input, storing its value in the byte at the data pointer.
[ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
] 	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.[a]
*/

fn parse(instructions: String) {
    let mut data: Vec<u8> = vec![0; 30_000];
    let mut data_ptr: usize = 0;
    let mut inst_ptr: usize = 0;

    let isnt: Vec<u8> = instructions.chars().map(|x| x as u8).collect();

    while inst_ptr < instructions.len() {
        match isnt[inst_ptr] as char {
            '+' => data[data_ptr] += 1,
            '-' => data[data_ptr] -= 1,
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '.' => print!("{}", data[data_ptr] as char),
            ',' => todo!(), // read user input
            '[' => jump(true),
            ']' => jump(false),
            _ => todo!(),
        }
        inst_ptr += 1
    }
}

fn jump(ptr: &i32, inst: &Vec<u8>) {
    if inst[*ptr as usize] as char == '[' {
        *ptr + inst
            .iter()
            .skip((ptr - 1) as usize)
            .fold((0, 0), |(inc, depth), el| match *el as char {
                '[' => (inc + 1, depth + 1),
                ']' if depth == 0 => (inc + 1, depth),
                ']' => (inc + 1, depth - 1),
                _ => (inc, depth),
            })
            .0;
    } else {
    }
}
