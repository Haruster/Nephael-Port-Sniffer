//Kinesys Port Sniffer.rs

use std::env;
use std::net::IpAddr;
use std::str::FromStr;

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
                    Err(_) => return Err("not a vaild IPADDR; must be IPv4 or IPv6");
                };

                let threads = match args[2].parse::<u16>() {
                    
                }
        
            }
        }
    }

}


fn main() {

    let mut args: Vec<String> = env::args().collect();

    fot x in &args {

        println!("{}", x);

    }

    println!("{:?}", args);


}
