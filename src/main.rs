use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut file: File = File::open("hello-world.bf")?;

    let mut bf_code = String::new();
    file.read_to_string(&mut bf_code)?;

    let output = execute_brainfuck(&bf_code);
    println!("{}", output);

    Ok(())
}

fn execute_brainfuck(code: &str) -> String{
    let mut memory: Vec<u8> = vec![0u8; 30000];
    let mut pointer: usize = 0usize;
    let mut loop_stack: Vec<usize> = Vec::new();
    let mut output: String = String::new();

    let code_byte: &[u8] = code.as_bytes();
    let mut code_pointer: usize = 0;

    while code_pointer < code_byte.len() {
        match code_byte[code_pointer] as char {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            '[' => {
                if memory[pointer] == 0 {
                    let mut nested: i32 = 1;
                    while nested > 0 {
                        code_pointer += 1;
                        if code_byte[code_pointer] as char == '[' {
                            nested += 1;
                        } else if code_byte[code_pointer] as char == ']' {
                            nested -= 1;
                        }
                    }
                } else {
                    loop_stack.push(code_pointer);
                }
            },
            ']' => {
                if memory[pointer] != 0 {
                    if let Some(&last_open_bracket) = loop_stack.last() {
                        code_pointer = last_open_bracket - 1;
                    }
                } else {
                    loop_stack.pop();
                }
            },
            '.' => {
                let ascii_code: u8 = memory[pointer];
                let character: char = char::from(ascii_code);

                output.push(character);
            }
            _ => {} // Ignorer les autres caractères
        }
        code_pointer += 1;
    }
    output
}
