extern crate ipster;
use ipster::{Ips, Patch};

#[test]
fn diff() {
    let orig = b"foobar".to_vec();
    let change = b"fOObar".to_vec();

    assert_eq!(ipster::Ips::new(&orig).diff(&change), vec![Patch::new(1, vec![b'O', b'O'])]);
}

#[test]
fn symmetry_between_diff_and_patch() {
    let orig = b"foobar".to_vec();
    let change = b"fOObar".to_vec();

    let ips = Ips::new(&orig);
    let diff = ips.diff(&change);

    assert_eq!(change, ips.patch(&diff));
}
