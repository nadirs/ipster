extern crate ipster;

#[test]
fn it_works() {
}

#[test]
fn symmetry() {
    let orig = b"foobar";
    let change = b"fOObar";
    ipster::patch(ipster::Ips::new(orig).diff(change.iter().collect()));
}
