//Kinesys Port Sniffer.rs

use std::env;
use std::net::IpAddr;

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
    }

}


fn main() {

    let mut args: Vec<String> = env::args().collect();

    fot x in &args {

        println!("{}", x);

    }

    println!("{:?}", args);


}
