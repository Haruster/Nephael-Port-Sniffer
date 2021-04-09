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
