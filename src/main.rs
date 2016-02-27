mod ipster;

use std::io::prelude::*;
use std::io::Result;
use std::fs::File;
use ipster::*;

fn read_file(filename: &str) -> Result<Vec<u8>> {
    let mut f = try!(File::open(filename));
    let mut buffer = Vec::new();

    try!(f.read_to_end(&mut buffer));

    Ok(buffer)
}

fn main () {
    match read_file("test.bin") {
        Ok(buffer) => Some(buffer),
        Err(e) => { println!("{}", e); None },
    }
    .map(|orig| {
        match read_file("test-edited.bin") {
            Ok(edited) => {
                println!("{:?}", {
                    let ips = Ips::new(orig);
                    let patches = ips.diff(edited);
                    let bs: Vec<u8> = patches.iter().flat_map(|p| p.bytes()).collect();
                    let mut x: Vec<u8> = "PATCH".bytes().collect();
                    x.extend(bs);
                    x.extend("EOF".bytes().collect::<Vec<_>>());
                    x
                });
            },
            Err(e) => println!("{:?}", e),
        };
    });
}
