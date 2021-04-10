//Kinesys Port Sniffer.rs

use std::env;
use std::io::{self, }
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Argumants {

    flag: String,
    ipaddr: IpAddr,
    threads: u16,

}

impl Argumants {

    fn new(args: &[String]) -> Result<Argumants, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {

            return Ok(Argumants {flag: String::from(" "), ipaddr, threads: 4});

        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want     \r\n    -h or -help to show this help message.");

                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a vaild IPADDR; must be IPv4 or IPv6")
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
                
                return Ok(Argumants{threads, flag, ipaddr})

            } else {
                return Err("invailed synrax");
            }
        }
    }

}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    
    let mut port: u16 = start_port + 1;

    loop {

        match TcpStream::connect((addr, port) {
            Ok(_) => {
                print!(".");
                
                io::stdout().flush().unwrap();

                tx.send(port).unwrap();

            }
        })

    }

}

fn main() {

    let mut args: Vec<String> = env::args().collect();
    let mut program = args[0].clone();
    let arguments = Argumants::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let mut num_threads = arguments.threads;
    let mut (tx, rx) = channel();
    
    for x in 0..num_threads {

        let mut tx = tx.clone();
    

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        })
    }

}
