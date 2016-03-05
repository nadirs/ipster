use std::io::prelude::*;
use std::io::Result;
use std::fs::File;
use std::fmt::Display;

use ipster;
use ipster::{Ips, Patch};

// from http://stackoverflow.com/a/27590832/1376657
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

fn log<T: Display>(e: T) -> () {
    println_stderr!("{}", e);
}

pub fn diff_files(orig_file: &str, change_file: &str) -> Option<Vec<u8>> {
    with_files(orig_file, change_file, |orig, change| {
        Some(diff(orig, change))
    })
}

pub fn diff(orig: Vec<u8>, change: Vec<u8>) -> Vec<u8> {
    let ips = Ips::new(&orig);
    let patches = ips.diff(&change);
    ips.serialize_patches(patches)
}

pub fn patch_files(orig_file: &str, change_file: &str) -> Option<Vec<u8>> {
    with_files(orig_file, change_file, |orig, change| {
        patch(orig, change)
    })
}

pub fn patch(orig: Vec<u8>, change: Vec<u8>) -> Option<Vec<u8>> {
    let ips = Ips::new(&orig);
    ipster::unserialize_patches(change)
        .map(|patches| ips.patch(&patches))
}

pub fn with_file<F, T>(filename: &str, mut callback: F) -> Option<T>
  where F: FnMut(Vec<u8>) -> Option<T> {

    match read_file(filename) {
        Ok(buffer) => Some(buffer),
        Err(e) => { log(e); None },
    }
    .and_then(callback)
}

fn with_files<F, T>(orig_file: &str, change_file: &str, mut callback: F) -> Option<T>
  where F: FnMut(Vec<u8>, Vec<u8>) -> Option<T> {

    match read_file(orig_file) {
        Ok(buffer) => Some(buffer),
        Err(e) => { log(e); None },
    }
    .and_then(|orig| {
        match read_file(change_file) {
            Ok(change) => {
                callback(orig, change)
            },
            Err(e) => { println_stderr!("{:?}", e); None },
        }
    })
}

pub fn read_file(filename: &str) -> Result<Vec<u8>> {
    let mut f = try!(File::open(filename));
    let mut buffer = Vec::new();

    try!(f.read_to_end(&mut buffer));

    Ok(buffer)
}

pub fn write_file(filename: &str, data: &[u8]) -> Result<()> {
    let mut f = try!(File::create(filename));
    try!(f.write_all(data));

    Ok(())
}
