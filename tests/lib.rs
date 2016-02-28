extern crate ipster;
use ipster;

#[test]
fn it_works() {
}

#[test]
fn symmetry() {
    let orig = b"foobar";
    let change = b"fOObar";
    ipster::patch(ipster::diff(orig.iter().collect(), change.iter().collect()));
}
