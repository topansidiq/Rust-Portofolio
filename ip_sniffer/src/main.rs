use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let first_arg = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&first_arg) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        }

        match first_arg.as_str() {
            "-h" | "-help" if args.len() == 2 => {
                println!(
                    "Usage: 
    -j <threads> <ipaddr>    Specify thread count and IP address
    -h or -help              Show this help message"
                );
                Err("help")
            }
            "-j" if args.len() == 4 => {
                let threads = args[2].parse::<u16>().map_err(|_| "Invalid thread count")?;
                if threads == 0 || threads > 1000 {
                    return Err("Thread count must be between 1 and 1000.");
                }
                let ipaddr = IpAddr::from_str(&args[3]).map_err(|_| "Invalid IP address")?;
                Ok(Arguments {
                    flag: first_arg,
                    ipaddr,
                    threads,
                })
            }
            _ => Err("Invalid syntax"),
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port;
    while port <= MAX {
        if let Ok(_) = TcpStream::connect((addr, port)) {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err == "help" {
            process::exit(0);
        } else {
            eprintln!("{}: error: {}", program, err);
            process::exit(1);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;

    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || scan(tx, i, addr, num_threads));
    }

    drop(tx);
    let mut out: Vec<u16> = rx.iter().collect();
    out.sort();

    println!("\nOpen ports:");
    for port in out {
        println!("Port {} is open", port);
    }
}
