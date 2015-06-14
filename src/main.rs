extern crate getopts;
extern crate rand;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::default::Default;
use rand::Rng;

struct State {
    program: [[char; 80]; 25],
    instruction_pointer: Point,
    stack: Vec<i32>,
    direction: Direction,
    running: bool,
}

impl Default for State {
    fn default() -> State {
        State {
            program: [[' '; 80]; 25], // 32 is the ASCII code for space
            stack: Vec::new(),
            instruction_pointer: Point::default(),
            direction: Direction::default(),
            running: true,
        }
    }
}

#[derive(Default)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn move_point(&mut self, dir: &Direction) {
        // Move instruction pointer
        let mut new_x = (self.x as i32) + dir.x;
        let mut new_y = (self.y as i32) + dir.y;

        if new_x < 0 {
            new_x = new_x + 80;
        }

        if new_y < 0 {
            new_y = new_y + 25;
        }

        self.x = (new_x % 80) as usize;
        self.y = (new_y % 25) as usize;
    }
}

struct Direction {
    x: i32,
    y: i32,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction {
            x: 1,
            y: 0,
        }
    }
}

fn print_usage(program: &str, options: Options) {
    let brief = format!("Usage: {} [options] filename", program);
    print!("{}", options.usage(&brief));
}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Initialize command line options
    let mut options = Options::new();
    options.optflag("h", "help", "print this help menu");
    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, options);
        return;
    }

    // Get filename from command line options
    let filename = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, options);
        return;
    };

    // Open source file
    let program_file = match File::open(filename) {
        Ok(file) => file,
        Err(..) => panic!("Could not open file!")
    };

    let mut reader = BufReader::new(&program_file);
    let program_string = &mut String::new();

    // Read Befunge source string from file
    let _ = reader.read_to_string(program_string);

    // Break program into lines
    let program_lines: Vec<&str> = program_string.lines().collect();

    // Initialize program state
    let mut state = State::default();

    // Read program characters into array
    for (row, line) in program_lines.iter().enumerate() {
        for (column, character) in line.chars().enumerate() {
            if row >= 80 || row >= 25 {
                panic!("Invalid source file!");
            }
            state.program[column][row] = character;
        }
    }

    let mut rng = rand::thread_rng();

    // Run program
    while state.running {
        match state.program[state.instruction_pointer.x][state.instruction_pointer.y] {
            '@' => state.running = false, // Stop running
            '<' => state.direction = Direction { x: -1, y: 0 }, // Move left
            '^' => state.direction = Direction { x: 0, y: 1 }, // Move up
            '>' => state.direction = Direction { x: 1, y: 0 }, // Move right
            'v' => state.direction = Direction { x: 0, y: -1 }, // Move down
            number_char @ '0' ... '9' => {
                // Push number on stack
                let number = number_char as i32;
                state.stack.push(number);
            },
            '+' => {
                // Add items on top of stack
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(a + b);
            }
            '-' => {
                // Subtract items on top of stack
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(b - a);
            },
            '*' => {
                // Multiply items on top of stack
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(a * b);
            },
            '/' => {
                // Integer division
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                if a == 0 {
                    panic!("Division by zero detected, and user input not supported yet!");
                }
                state.stack.push(b / a);
            },
            '%' => {
                // Modulo items on top of stack
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(b % a);
            },
            '$' => {
                // Pop stack and discard
                let _ = state.stack.pop();
            },
            '!' => {
                // Logical NOT
                let top_value = state.stack.pop().unwrap();
                match top_value {
                    0 => state.stack.push(1),
                    _ => state.stack.push(0),
                }
            },
            '`' => {
                // Greater than
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                if b > a {
                    state.stack.push(1);
                } else {
                    state.stack.push(0);
                }
            },
            ':' => {
                // Duplicate top of stack
                let top_value = state.stack.last().unwrap().clone();
                state.stack.push(top_value);
            },
            '?' => {
                // Move in random cardinal direction
                let random = rng.gen_range::<i32>(0, 4);
                match random {
                    0 => state.direction = Direction { x: 0, y: 1 },
                    1 => state.direction = Direction { x: 1, y: 0 },
                    2 => state.direction = Direction { x: 0, y: -1 },
                    3 => state.direction = Direction { x: -1, y: 0 },
                    _ => {},
                }
            },
            _ => {},
        }

        state.instruction_pointer.move_point(&(state.direction));
    }
}
