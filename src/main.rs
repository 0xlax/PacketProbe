use std::env;
use std::net::{IpAddr, TCPStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
const MAX_PORT: u16 = 65535;
// const MIN_PORT: u16 = 1;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments provided");
        } else if args.len() > 4 {
            return Err("Too many arguments provided");
        }

        let flag = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Arguments {
                flag,
                ipaddr,
                threads: 4, // Default value
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && flag.len() == 2 {
                println!("Usage: -j to select how many threads you want to use, \r\n -h or -help for help");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments provided");
            } else if flag.contains("-j") {
                if args.len() == 4 {
                    let threads = args[2].parse::<u16>().map_err(|_| "Invalid thread count provided")?;
                    let ipaddr = IpAddr::from_str(&args[3]).map_err(|_| "Invalid IP address format")?;

                    return Ok(Arguments {
                        flag,
                        ipaddr,
                        threads,
                    });
                } else {
                    return Err("Invalid number of arguments provided");
                }
            }
            return Err("Invalid flag provided");
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, ipaddr: IpAddr, threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TCPStream::connect((ipaddr, port)) {
            Ok(_) => {
                println!("Port {} is open on {}", port, ipaddr);
                tx.send(port).unwrap();
            }
            Err(_) => {
                // println!("Port {} is closed on {}", port, ipaddr);
            }
        }
        
        if port >= MAX_PORT {
            break;
        }
        port += threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            println!("Usage: {} <flag> [threads] <ipaddr>", program);
            std::process::exit(0);
        } else {
            eprintln!("Error: {} problem parsing arguments: {}", program, err);
            std::process::exit(1);
        }
    });

    let (ipaddr, threads) = (arguments.ipaddr, arguments.threads)

    let (tx, rx) = channel();

    for i n 0..threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, ipaddr, threads)
        })
    }



    // Now you can use the arguments struct
    println!("Successfully parsed arguments:");
    println!("IP Address: {}", arguments.ipaddr);
    println!("Threads: {}", arguments.threads);
}