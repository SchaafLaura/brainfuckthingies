mod data;

use data::Data;
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

fn parse(instructions: String) {
    let mut data = Data::new();
    let mut inst_ptr: usize = 0;

    let inst: Vec<u8> = instructions.chars().map(|x| x as u8).collect();

    while inst_ptr < instructions.len() {
        match inst[inst_ptr] as char {
            '+' => data += 1,
            '-' => data -= 1,
            '>' => data >>= 1,
            '<' => data <<= 1,
            '.' => print!("{}", data),
            ',' => {
                use std::io::Read;

                let input = std::io::stdin().bytes().next().and_then(|r| r.ok());
                if let Some(b) = input {
                    data.set(b);
                }

                // Test code:
                // A,!>[B...-?]<>[C...--?]<Z
                // expected behavior:
                // when input is 'B' it prints "B" 3 times and goes back to the beginning
                // when input is 'C' it prints "C" 3 times and goes back to the beginning
                // when input is 'Z' the program exits
                //
                // actual behavior:
                // when input is 'B' it crashes :(
                // doesn't even print stuff, but that might be rust weirdness. It does jump back to the start
                // but then skips the input and tries to jump to (13) - the ascii code for carriage return

                todo!("get rid of carriage return (ascii code: 13) in stdin, it messes up the next input 
                    or clear the stream or smthn..idfk :(");
            }
            '[' => {
                if data == 0 {
                    jump(true, &mut inst_ptr, &inst)
                }
            }
            ']' => {
                if data != 0 {
                    jump(false, &mut inst_ptr, &inst)
                }
            }
            '!' => goto(true, data.get(), &mut inst_ptr, &inst), // not in spec, but cool?
            '?' => goto(false, data.get(), &mut inst_ptr, &inst),
            _ => {
                /*eprintln!(
                    "unrecognized character in the input at {}: {}: `{}`",
                    inst_ptr,
                    inst[inst_ptr],
                    whitespace_escape(inst[inst_ptr] as char)
                );*/
            }
        }
        inst_ptr += 1
    }
}

fn goto(dir: bool, func_id: u8, ptr: &mut usize, inst: &[u8]) {
    println!("jumping to: {}", func_id);
    while inst[*ptr] != func_id {
        incr(dir)(ptr);
    }
    println!("jumped successfully - ptr at: {}", ptr);
}

fn jump(dir: bool, ptr: &mut usize, inst: &[u8]) {
    let brace = (if dir { '[' } else { ']' }) as u8;
    let other_brace = (if dir { ']' } else { '[' }) as u8;
    let mut depth = 0;
    let increment = incr(dir);
    increment(ptr);
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

/*
fn whitespace_escape(c: char) -> String {
    if c.is_ascii_whitespace() {
        match c as u8 {
            0 => "<null>",
            10 | 15 => "<new_line>",
            9 | 11 => "<tab>",
            32 => "<space>",
            _ => {
                eprintln!("{}", c as u8);
                unreachable!();
            }
        }
        .to_string()
    } else {
        c.to_string()
    }
}
*/
