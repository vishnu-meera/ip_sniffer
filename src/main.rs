use std::io::Write;
use std::net::TcpStream;
use std::{env, net::IpAddr, str::FromStr};
use std::{process, io};
use std::sync::mpsc::{Sender,channel};
use std::thread;

const MAX: u16 = 65535;
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2 {
            return Err("not enough argumentts");
        } else if args.len() > 4 {
            return Err("too many arguments");
        } 

        let f: String = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments{flag: String::from(""), ipaddr, threads: 4});
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("show help");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP address")
                };

                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Not valid thread number")
                };

                return Ok(Arguments{threads,flag,ipaddr})
            } else {
                return Err("some error invalid syntax");
            }

        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        }else {
            eprintln!("problem parsing {} {} ",program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addrr = arguments.ipaddr;
    let (tx, rx) = channel();

    for i in 0..num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addrr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("----------------------------------------------------------");

    out.sort();
    for v in out {
        println!("{} is opnen", v);
    }
}

fn scan(tx: Sender<u16>, star_port: u16, addr:IpAddr, num_threads: u16) {
    let mut port = star_port + 1;
    loop {
        match TcpStream::connect((addr,port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {

            }
        }

        if MAX - port  < num_threads {
            break;
        }
        port+=num_threads;
    }
}