use std::io::{self, Read, Write};
use std::env;
use std::fs::File;
extern crate time;
use time::PreciseTime;


fn parse(s : String, p: &mut usize, ar: &mut[u8; 64]) {
    let mut is_cycled = true;
    let mut skip = 0;
    while is_cycled {
        is_cycled = false;
        for i in 0..s.len() {                               //Iterate all instructions
            if skip == 0 {                                  //If don't need to skip
                match s.chars().skip(i).next().unwrap() {   //Watch on curr instruction
                    '>' => *p += 1,                         //Increment pointer
                    '<' => {                                //Decrement pointer
                        if *p > 0 {
                            *p -= 1;
                        }
                    },
                    '+' => ar[*p] += 1,                     //Increment value
                    '-' => {                                //Decrement value
                        if ar[*p] > 0 {
                            ar[*p] -= 1;
                        }
                    },
                    '.' => print!("{}", ar[*p] as char),    //Print value
                    ',' => {
                        print!(">>>");
                        match io::stdout().flush() {
                            Err(e) => {
                                println!("Bad stdout");
                                return;
                            },
                            _ => {},
                        }
                        let mut buf = [0u8, 1];
                        let stdin = io::stdin();
                        let mut handl = stdin.lock();
                        match handl.read_exact(&mut buf) {
                            Err(e) => {
                                println!("Failed to read from stdin: {}", e);
                                return;
                            },
                            _ => {},
                        }
                        ar[*p] = buf[0];
                    },
                    '[' =>  {                               //Start loop if current value is not eq to 0
                        if ar[*p] == 0 {
                            let mut c = 0;
                            for j in i..s.len() {   //Search for closing bracket
                                match s.chars().skip(j).next().unwrap() {
                                    '[' => c += 1,
                                    ']' => {
                                        if c > 0 {
                                            c -= 1;
                                        }
                                        else if j >= i{
                                            skip = j - i;
                                            break;  //Found bracket
                                        }
                                    },
                                    _ => {},
                                }
                            }
                            if c != 0 {
                                println!("Can't find closing bracket for instruction {}", i);
                                return;             //Can't find bracket
                            }
                        }
                    },
                    ']' => {                                //End of loop
                        if ar[*p] != 0 {
                            is_cycled = true;
                            let mut c = 0;
                            for j in (0..i).rev() { //Search for opening bracket to create loop
                                match s.chars().skip(j).next().unwrap() {
                                    '[' => {
                                        if c > 0 {
                                            c -= 1;
                                        }
                                        else {
                                            skip = s.len() - i + j;
                                            break; //Found
                                        }
                                    },
                                    ']' => {
                                        c += 1;
                                    },
                                    _   => {},
                                }
                            }
                            if c != 0 {
                                println!("Can't find opening bracket for instruction {}", i);
                                return;           //Can't find
                            }
                        }
                    },
                    _ => {},
                }
            }
            else {
                skip -= 1;                              //Instruction skipped
            }
        }
    }
}


fn main() -> io::Result<()> {
    let helpmsg = "\trustfuck iterpreter help:\n-h\t\tshow this help\n-s\t\tread from stdin\n-o <file>\tread from file";

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);                                 //Remove call of rustfuck

    let mut buf = String::new();                    //String for user input

    if args.contains(&String::from("-h")) {
                                                    //Show help message
        println!("{}", helpmsg);
    }
    else if args.contains(&String::from("-i")) {
                                                    //Read from stdin
        let stdin = io::stdin();
        let mut handl = stdin.lock();
        handl.read_to_string(&mut buf)?;
    }
    else if args.contains(&String::from("-o")) {
                                                    //Read from file
        let filedir = args.get(args.iter().position(|x| x == "-o").unwrap() + 1).unwrap();
        if filedir.len() == 0 {                     //Check if file does not specified
            println!("No file specified!");
            return Ok(());
        }
        let file = File::open(filedir);
        match file {
            Ok(mut f) => {
                match f.read_to_string(&mut buf) {
                    Ok(_) => {},
                    Err(e) => {                     //If can't open file
                        println!("Can't open file {}\n{}", filedir, e);
                        return Ok(());
                    },
                }
            },
            Err(e) => {
                println!("Can't open file {}\n{}", filedir, e);
                return Ok(());
            },
        }
    }
    let mut code = String::new();                   //String for instructions
    for i in 0..buf.len() {                         //Remove all shit from input
        match buf.chars().skip(i).next().unwrap() {
            '+' | '-' | '[' | ']' | '>' | '<' | '.' | ',' => code.push(buf.chars().skip(i).next().unwrap()),
            _ => {},
        }
    }

    let mut array: [u8; 64] = [0; 64];              //Array for values
    let mut pointer = 0;                            //Current pointer
    let start = PreciseTime::now();                 //Save current time for a nice statistics
    parse(code, &mut pointer, &mut array);
    let end = PreciseTime::now();                   //Save time of ending parse work
    println!("\n\nDone in {} milliseconds", start.to(end).num_milliseconds());  //print nice statistics

    Ok(())
}