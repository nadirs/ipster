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
fn unserialize_rle_patch() {
    let mut patch_binary = b"PATCH".to_vec();
    patch_binary.extend(vec![0, 0, 1, 0, 0, 0, 2, b'O']);
    patch_binary.extend(vec![0, 0, 6, 0, 3, b'B', b'A', b'Z']);
    patch_binary.extend(b"EOF");

    let patch = vec![
        Patch::new(1, vec![b'O', b'O']),
        Patch::new(6, vec![b'B', b'A', b'Z'])
    ];

    assert_eq!(patch, unserialize_patches(patch_binary).unwrap());
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

#[test]
fn no_patch_at_0x454f46() {
    let orig = vec![1; 0x455500];
    let mut change = orig.clone();
    change[0x454f46] = 5;
    let change_vec = change.to_vec();

    let ips = Ips::new(&orig);
    let diff = ips.diff(&change);

    assert_eq!(diff, vec![Patch::new(0x454f45, vec![0, 5])]);
}
