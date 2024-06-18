use std::io;
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

fn main() {
    read_code(String::from(",[>+++<-]>."), false);
}


fn read_code(code: String, debug: bool) {
    let bytes = code.as_bytes();

    let mut memory: Vec<i32> = vec![0];
    let mut ignore: bool = false;
    let mut pointer: usize = 0;

    let mut position_loop_started: Vec<i32> = vec![0];

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
            println!("{}", memory[pointer]);
        }
        else if byte == COMMA {
            let mut number = String::new();
            io::stdin().read_line(&mut number).expect("Failed");
            let number: usize = number.trim().parse().expect("Failed again");
            memory[pointer] = number as i32;
        }
        else if byte == LBRACKET {
            position_loop_started[0] = i as i32;
        }
        else if byte == RBRACKET {
            if memory[pointer] != 0 {
                i = position_loop_started[0] as usize;
            }
            
        }

        if debug {
            println!("Action: {}. Vec: {:?}", byte ,memory);
        }

        i+=1;
    }
}



fn update_vec(vector: &mut Vec<i32>, index: usize){
    while vector.len() <= index {
        vector.push(0);
    }
}
