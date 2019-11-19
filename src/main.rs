use std::io::{self, Read};
use std::env;
use std::fs::File;


fn parse(s : String, p: &mut usize, ar: &mut[u8; 64]) {
    // println!("\nin {} p {} curr {}", s, p, ar[*p]);
    let mut is_cycled = true;
    let mut skip = 0;
    while is_cycled {
        is_cycled = false;
        for i in 0..s.len() {
            if skip == 0 {
                // print!("{}", s.chars().skip(i).next().unwrap());
                match s.chars().skip(i).next().unwrap() {
                    '>' => *p += 1,
                    '<' => {
                        if *p > 0 {
                            *p -= 1;
                        }
                    },
                    '+' => ar[*p] += 1,
                    '-' => {
                        if ar[*p] > 0 {
                            ar[*p] -= 1;
                        }
                    },
                    '.' => print!("{}", ar[*p] as char),
                    '[' =>  {
                                if ar[*p] == 0 {
                                    let mut c = 0;
                                    for j in i..s.len() {
                                        match s.chars().skip(j).next().unwrap() {
                                            '[' => c += 1,
                                            ']' => {
                                                if c > 0 {
                                                    c -= 1;
                                                }
                                                else if j >= i{
                                                    skip = j - i;
                                                    // println!(">skip {} to {} {} chars", skip, s.chars().skip(skip).next().unwrap(), j);
                                                    break;
                                                }
                                            },
                                            _ => {},
                                        }
                                    }
                                    if c != 0 {
                                        // println!("Failed to find closing brackets");
                                        return;
                                    }
                                }
                            },
                    ']' => {
                                if ar[*p] != 0 {
                                    // println!("SKIP");
                                    is_cycled = true;
                                    // skip = s.len() - i;
                                    let mut c = 0;
                                    // print!("\nsearch i {}\t", i);
                                    for j in (0..i).rev() {
                                        // print!("_{}_", s.chars().skip(j).next().unwrap());
                                        match s.chars().skip(j).next().unwrap() {
                                            '[' => {
                                                // println!("\"[\"");
                                                if c > 0 {
                                                    c -= 1;
                                                }
                                                else {
                                                    skip = s.len() - i + j;
                                                    // println!("skip to {} {}, {}", j, s.chars().skip(j).next().unwrap(), skip);
                                                    break;
                                                }
                                            },
                                            ']' => {
                                                // println!("\"]\"");
                                                c += 1;
                                            },
                                            _   => {},
                                        }
                                    }
                                    // println!();
                                }
                                // else {
                                //     // println!("RET");
                                //     return;
                                // }

                            },
                            _ => {},
                }
            }
            else {
                skip -= 1;
                // println!("skip");
            }
        }
    }
}


fn main() -> io::Result<()> {
    let helpmsg = "\trustfuck iterpreter help:\n-h\t\tshow this help\n-s\t\tread from stdin\n-o <file>\tread from file";

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);                                 //Remove call of rustfuck

    let mut buf = String::new();

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
                    Err(e) => {
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
    let mut code = String::new();
    for i in 0..buf.len() {
        match buf.chars().skip(i).next().unwrap() {
            '+' | '-' | '[' | ']' | '>' | '<' | '.' | ',' => code.push(buf.chars().skip(i).next().unwrap()),
            _ => {},
        }
    }

    let mut array: [u8; 64] = [0; 64];
    let mut pointer = 0;

    parse(code, &mut pointer, &mut array);


    Ok(())
}