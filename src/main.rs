use std::io;
use std::env;
use std::fs::File;
use std::io::Read;

const ADD: u8 = b'+';
const SUBSTRACT: u8 = b'-';
const LBRACKET: u8 = b'[';
const RBRACKET: u8 = b']';
const POINT: u8 = b'.';
const COMMA: u8 = b',';
const RIGHT: u8 = b'>';
const LEFT: u8 = b'<';
const COMMENT: u8 = b'/';
const SPACE: u8 = b' ';
const NEWLINE: u8 = b'\n';
const PRINTOPTION: u8 = b';';

// File reading
fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        let text = args[1].clone();
        println!("{}", text);

        let mut file = File::open(&text)?;
        
        let mut code = String::new();
        file.read_to_string(&mut code)?;

        //println!("{}", code);
        read_code(&code, false);
    }
    Ok(())
}



fn read_code(code: &String, debug: bool) {
    let bytes = code.as_bytes();

    let mut memory: Vec<i32> = vec![0];
    let mut ignore: bool = false;
    let mut pointer: usize = 0;

    let mut position_loop_started: Vec<i32> = vec![0];
    let mut loop_counter: usize = 0;

    let mut print_in_letters = false;

    let mut i: usize = 0;
    while i < bytes.len() {
        let byte = bytes[i];

        if byte == COMMENT {
            ignore = !ignore;
        }

        if ignore {
            i+=1;
            continue;
        }
        if byte == NEWLINE || byte == SPACE {}
        else if byte == PRINTOPTION {
            print_in_letters = !print_in_letters;
        }
        else if byte == ADD {
            memory[pointer] += 1;
        }
        else if byte == SUBSTRACT {
            memory[pointer] -= 1;
        }
        else if byte == RIGHT {
            pointer += 1;
            update_vec(&mut memory, pointer);
        }
        else if byte == LEFT {
            pointer -= 1;
        }
        else if byte == POINT {
            if print_in_letters {
                print!("{}", memory[pointer] as u8 as char);
                io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
            }
            else {
                print!("{} ", memory[pointer]);
                io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
            }
        }
        else if byte == COMMA {
            let mut number = String::new();
            io::stdin().read_line(&mut number).expect("Failed");
            let number: usize = number.trim().parse().expect("Failed again");
            memory[pointer] = number as i32;
        }
        else if byte == LBRACKET {
            loop_counter += 1;
            update_vec(&mut position_loop_started, loop_counter-1);
            position_loop_started[loop_counter - 1] = i as i32;
        }
        else if byte == RBRACKET {
            if memory[pointer] != 0 {
                i = position_loop_started[loop_counter - 1] as usize;
            } else {
                loop_counter -= 1;
            }
            
        }

        if debug {
            println!("Action: {}. Vec: {:?}", byte, memory);
            println!("loop_counter: {}. position_loop_started: {:?}", loop_counter, position_loop_started);
        }

        i+=1;
    }
    println!("");
}



fn update_vec(vector: &mut Vec<i32>, index: usize){
    while vector.len() <= index {
        vector.push(0);
    }
}
