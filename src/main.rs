use std::env;
use std::error::Error;
use std::io::{BufReader, BufWriter, Read, stderr, stdin, Write};
use std::net::TcpStream;
use std::process::exit;

struct Args {
    host: String,
}

fn parse_args() -> Args {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        writeln!(stderr(), "Usage: ./cmd host:port").unwrap();
        exit(1);
    }
    Args {
        host: args[0].to_string(),
    }
}


fn close(err: &Error) {
    writeln!(stderr(), "{}", err).unwrap();
    exit(1);
}

fn main() {
    let args = parse_args();
    let mut writer = match TcpStream::connect(args.host) {
        Ok(s) => BufWriter::new(s),
        Err(e) => return close(&e),
    };
    let mut buf: [u8; 128] = [0; 128];
    let mut reader = BufReader::new(stdin());
    loop {
        match reader.read(&mut buf) {
            Ok(s) => {
                if let Err(e) = writer.write_all(&buf[..s]) {
                    return close(&e);
                }
            }
            Err(e) => return close(&e),
        }
    }
}
