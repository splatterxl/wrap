use std::{env, error::Error, fs, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let file = env::args().nth(1);

    match file {
        Some(file) => {
            if file == "--help" {
                println!(
                    "Wrap\n\
                    Copyright (C) 2020 Splatterxl\n\
                    \n\
                    A command-line utility for wrapping file contents\n\
                    \n\
                    Usage: wrap <file>"
                );
                exit(0)
            }
            let content = fs::read_to_string(file);

            match content {
                Err(err) => {
                    eprintln!("fatal: could not read file\n\n---Error---\n{:#?}", err);
                    exit(1)
                }
                Ok(content) => {
                    let mut limit = wrap::LIMIT;
                    match option_env!("WRAP_LINE_SIZE") {
                        Some(num) => {
                            let num = num.parse::<usize>();
                            match num {
                                Ok(n) => limit = n,
                                Err(_) => {
                                    eprintln!("fatal: invalid WRAP_LINE_SIZE provided");
                                    exit(1);
                                }
                            }
                        }
                        None => {}
                    }
                    println!("{}", wrap::wrap(content, Some(limit)));

                    Ok(())
                }
            }
        }
        None => {
            eprintln!(
                "fatal: no files provided\n\
                \n\
                Usage: wrap <file>\n\
                \n\
                Run `wrap --help` for more details."
            );
            exit(1);
        }
    }
}
