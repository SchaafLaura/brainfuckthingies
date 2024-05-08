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

    let inst: Vec<u8> = instructions.chars().map(|x| x as u8).collect();

    while inst_ptr < instructions.len() {
        match inst[inst_ptr] as char {
            '+' => data[data_ptr] += 1,
            '-' => data[data_ptr] -= 1,
            '>' => data_ptr += 1,
            '<' => data_ptr -= 1,
            '.' => print!("{}", data[data_ptr] as char),
            ',' => todo!(), // read user input
            '[' => {
                if data[data_ptr] == 0 {
                    jump(true, &mut inst_ptr, &inst)
                }
            }
            ']' => {
                if data[data_ptr] != 0 {
                    jump(false, &mut inst_ptr, &inst)
                }
            }
            _ => todo!(),
        }
        inst_ptr += 1
    }
}

fn jump(dir: bool, ptr: &mut usize, inst: &Vec<u8>) {
    let brace = (if dir { '[' } else { ']' }) as u8;
    let other_brace = (if dir { ']' } else { '[' }) as u8;
    let mut depth = 0;
    let increment = incr(dir);
    loop {
        if inst[*ptr] == other_brace {
            if depth == 0 {
                return;
            } else {
                depth -= 1;
            }
        } else if inst[*ptr] == brace {
            depth += 1;
        }
        increment(ptr);
    }

    fn incr(dir: bool) -> fn(ptr: &mut usize) {
        if dir {
            |p: &mut usize| {
                *p += 1;
            }
        } else {
            |p: &mut usize| {
                *p -= 1;
            }
        }
    }
}
