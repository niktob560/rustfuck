use std::io::{self, Read};


fn parse(s : String, p: &mut usize, ar: &mut[u8; 1024]) -> Result<(), &'static str> {
    let mut is_cycled = true;
    let mut skip;
    while is_cycled {
        is_cycled = false;
        skip = 0;
        for i in 0..s.len() {
            if skip == 0 {
                match s.chars().skip(i).next().unwrap() {
                    '>' => *p += 1,
                    '<' => *p -= 1,
                    '+' => ar[*p] += 1,
                    '-' => ar[*p] -= 1,
                    '.' => print!("{}", ar[*p]),
                    '[' =>  {
                                if ar[*p] != 0 {
                                    let a = parse(String::from(s.split_at(i + 1).1), p, ar);
                                    match a {
                                        Ok(()) => {},
                                        Err(e) => {
                                                    println!("{}", e);
                                                    return Err(e)
                                                },
                                    }
                                }
                                skip = s.find(']').unwrap() - i;
                            },
                    ']' => {
                                if ar[*p] != 0 {
                                    is_cycled = true;
                                    skip = s.len() - i;
                                } 
                                else {
                                    return Ok(());
                                }
                            },
                            _ => {},
                }
            }
            else {
                skip -= 1;
            }
        }
    }
    Ok(())
}


fn main() -> io::Result<()> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handl = stdin.lock();
    handl.read_to_string(&mut buf)?;


    let mut array: [u8; 1024] = [0; 1024];
    let mut pointer = 0;

    let a = parse(buf, &mut pointer, &mut array);

    match a {
        Ok(_) => {},
        Err(e) => println!("Err: {}", e),
    }
    


    println!("");

    Ok(())
}
