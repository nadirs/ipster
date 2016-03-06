extern crate ipster;
use ipster::*;

#[test]
fn diff_simple() {
    let orig = b"foobar".to_vec();
    let change = b"fOObar".to_vec();

    assert_eq!(
        ipster::Ips::new(&orig).diff(&change),
        vec![Patch::new(1, vec![b'O', b'O']),
    ]);
}

#[test]
fn diff_extended() {
    let orig = b"foobar".to_vec();
    let change = b"fOObarBAZ".to_vec();

    assert_eq!(
        ipster::Ips::new(&orig).diff(&change),
        vec![
            Patch::new(1, vec![b'O', b'O']),
            Patch::new(6, vec![b'B', b'A', b'Z'])
        ]);
}
#[test]
fn patched_max_len() {
    let patch = vec![
        Patch::new(1, vec![b'O', b'O']),
        Patch::new(6, vec![b'B', b'A', b'Z'])
    ];

    assert_eq!(patch_max_len(&patch).unwrap(), 9);
}

#[test]
fn patch_extended() {
    let orig = b"foobar".to_vec();
    let patch = vec![
        Patch::new(1, vec![b'O', b'O']),
        Patch::new(6, vec![b'B', b'A', b'Z'])
    ];

    let change = b"fOObarBAZ".to_vec();

    assert_eq!(change, ipster::Ips::new(&orig).patch(&patch));
}

#[test]
fn symmetry_between_diff() {
    let orig = b"foobar".to_vec();
    let change = b"fOObar".to_vec();

    let ips = Ips::new(&orig);
    let diff = ips.diff(&change);

    assert_eq!(change, ips.patch(&diff));
}

#[test]
fn symmetry_between_diff_and_patch_extended() {
    let orig = b"foobar".to_vec();
    let change = b"fOObarBAZ".to_vec();

    let ips = Ips::new(&orig);
    let diff = ips.diff(&change);

    assert_eq!(change, ips.patch(&diff));
}
