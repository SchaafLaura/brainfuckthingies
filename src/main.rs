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
            _ => {
                eprintln!(
                    "unrecognized character in the input at {}: {}: `{}`",
                    inst_ptr,
                    inst[inst_ptr],
                    whitespace_escape(inst[inst_ptr] as char)
                );
                // exit when encountering a wrong character?
                // or ignore and continue?
                // std::process::exit(1);
            }
        }
        inst_ptr += 1
    }
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
