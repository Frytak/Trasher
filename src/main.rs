use std::fs::OpenOptions;
use std::io::{stdin, Write};

const OPERATORS: [char; 4] = ['+', '-', '*', '/'];
const TRASH_WIDTH: usize = 7;
const TRASH_HEIGHT: usize = 6;
const LINE_WIDTH: usize = TRASH_WIDTH * 15;
const AMOGUS: [[char; TRASH_WIDTH]; TRASH_HEIGHT] = [
    [' ','@','@','@',' ',' ',' '],
    ['@',' ',' ',' ','@',' ',' '],
    ['@','@','@','@','@','@',' '],
    ['@','@','@','@','@','@',' '],
    ['@',' ',' ',' ','@',' ',' '],
    [' ',' ',' ',' ',' ',' ',' '],
];

fn eval(string: &String) -> String {
    let mut sliced: Vec<String> = vec![String::new()];
    let mut i: usize = 0;
    for c in string.chars() {
        if OPERATORS.contains(&c) {
            sliced.push(c.to_string());
            sliced.push(String::new());
            i += 2;
            continue;
        }

        sliced[i].push(c);
    }

    let mut current_value: isize = sliced[0].parse::<isize>().unwrap();

    for (i, op) in sliced.clone().into_iter().enumerate() {
        match op.as_str() {
            "+" => { current_value += sliced[i+1].parse::<isize>().unwrap(); }
            "-" => { current_value -= sliced[i+1].parse::<isize>().unwrap(); }
            "*" => { current_value *= sliced[i+1].parse::<isize>().unwrap(); }
            "/" => { current_value /= sliced[i+1].parse::<isize>().unwrap(); }
            _ => {}
        }
    }

    current_value.to_string()
}

fn main() -> std::io::Result<()> {
    'main: loop {
        let mut raw_input = String::new();
        let mut output = String::new();

        // Getting the input
        println!("Enter the number of bytes to generate: ");
        stdin().read_line(&mut raw_input)?;
        let forrmated_input: String = raw_input.chars().filter(|c| !c.is_whitespace()).collect();

        // Checking for the strings correctness
        for c in forrmated_input.chars() {
            if !c.is_ascii_digit() && !OPERATORS.contains(&c) {
                println!("Wrong format. Only numbers and operations are accepted. The string you provided was: '{}'", raw_input.trim_end());
                continue 'main;
            }
        }
        let file_size = eval(&forrmated_input).parse::<usize>().unwrap();

        // String for the file
        for y in 0..=file_size/LINE_WIDTH {
            for x in 0..LINE_WIDTH {
                if y*LINE_WIDTH+x == file_size { break; }
                if x == LINE_WIDTH-1 {
                    output.push('\n');
                    continue;
                }
                output.push(AMOGUS[y%TRASH_HEIGHT][x%TRASH_WIDTH]);
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("trash.txt")
            .unwrap();
        print!("\n\n");

    file.write_all(output.as_bytes())?;
    }
}
