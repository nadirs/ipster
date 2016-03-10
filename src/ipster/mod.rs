pub mod files;

use std::cmp;
use std::iter;
use std::io::Write;

const MAX_PATCH_LEN: usize = 0xFFFF;

#[derive(Debug)]
pub struct Ips {
    buffer: Vec<u8>
}

impl Ips {
    pub fn new(buffer: &Vec<u8>) -> Self {
        Ips {
            buffer: buffer.clone()
        }
    }

    pub fn diff(&self, change: &Vec<u8>) -> Vec<Patch> {
        let (change_within, additional) = change.split_at(self.buffer.len());
        let mut pairs = (0..).zip(self.buffer.iter().zip(change_within));

        let mut patches: Vec<Patch> = vec![];
        let mut index = 0;
        let mut data = vec![];

        while let Some((offset, (&before, &after))) = pairs.next() {
            if before != after {
                if data.is_empty() {
                    index = offset;
                }
                data.push(after);
            } else if ! data.is_empty() {
                let patch = Patch::new(index, data);
                patches.push(patch);
                data = vec![];
            }
        };

        for chunk in additional.chunks(MAX_PATCH_LEN) {
            let patch = Patch::new(self.buffer.len() as u32, chunk.to_vec());
            patches.push(patch);
        }

        patches
    }

    pub fn patch(&self, change: &Vec<Patch>) -> Vec<u8> {
        let total_size = cmp::max(self.buffer.len(), patch_max_len(&change).unwrap());

        let mut output = Vec::with_capacity(total_size);
        output.write(&self.buffer);

        for patch in change {
            {
                let (_, mut skipped) = output.split_at_mut(patch.addr as usize);
                skipped.write(&patch.data())
            }.map(|written_len| {
                let data = patch.data();
                let (_, unwritten_data) = data.split_at(written_len);
                for &b in unwritten_data {
                    output.push(b);
                }
            });
        }
        output
    }

}

pub fn patch_max_len(patch: &[Patch]) -> Option<usize> {
    patch.iter().map(|x| x.addr as usize + x.data().len()).max()
}

pub fn serialize_patches(patches: Vec<Patch>) -> Vec<u8> {
    let patch_contents: Vec<u8> = patches.iter().flat_map(|p| p.bytes()).collect();
    let mut binary: Vec<u8> = b"PATCH".to_vec();
    binary.extend(patch_contents);
    binary.extend(b"EOF".to_vec());
    binary
}

pub fn unserialize_patches(binary: Vec<u8>) -> Option<Vec<Patch>> {
    let mut patches: Vec<Patch> = Vec::new();

    let slice = &binary;
    let (header, mut slice) = slice.split_at(5);

    if header != b"PATCH" {
        return None;
    }

    loop {
        // TODO support RLE'd patches
        let (addr_slice, rest) = slice.split_at(3);
        let (len_slice, rest) = rest.split_at(2);

        if (! addr_slice.is_empty()) && len_slice.len() < 2 {
            // Malformed data
            return None;
        }

        let mut addr_array = [0; 3];
        addr_array.clone_from_slice(addr_slice);
        let addr = Patch::unserialize_addr_array(addr_array);

        let mut len_array = [0; 2];
        len_array.clone_from_slice(len_slice);
        let len = Patch::unserialize_len(len_array);

        if rest.len() < len {
            // Malformed data
            return None;
        }

        let (patch, rest) = if len == 0 {
            // should decode RLE
            let (rle_len_slice, rest) = rest.split_at(2);

            let mut rle_len_array = [0; 2];
            rle_len_array.clone_from_slice(rle_len_slice);
            let rle_len = Patch::unserialize_len(rle_len_array);

            let (rle_val_slice, rest) = rest.split_at(1);
            let rle_val = rle_val_slice[0];

            let patch = Patch::new(addr, from_rle(rle_len, rle_val));
            (patch, rest)
        } else {
            let (data, rest) = rest.split_at(len);

            (Patch::new(addr, data.to_vec()), rest)
        };

        patches.push(patch);

        if rest.len() < 3 {
            // Malformed data
            return None;
        }

        let (possible_eof, _) = rest.split_at(3);
        if possible_eof == b"EOF" {
            // We are done here
            break;
        }

        slice = rest;
    }

    Some(patches)
}

pub fn into_rle(data: Vec<u8>) -> (usize, u8) {
    (data.len(), data[0])
}

pub fn from_rle(len: usize, val: u8) -> Vec<u8> {
    iter::repeat(val).take(len).collect()
}

#[derive(Debug, PartialEq, Clone)]
pub struct Patch {
    addr: u32,
    data: Vec<u8>
}

impl Patch {
    pub fn new(addr: u32, data: Vec<u8>) -> Self {
        assert!(addr <= 0xFFFFFF);
        Patch {
            addr: addr,
            data: data
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.serialize_addr();
        let len = self.serialize_len();
        bytes.extend(len);
        bytes.extend(self.data());
        bytes
    }

    pub fn unserialize_addr_array(addr: [u8; 3]) -> u32 {
        ((addr[0] as u32) << 16) | ((addr[1] as u32) << 8) | addr[2] as u32
    }

    pub fn serialize_addr(&self) -> Vec<u8> {
        vec![(self.addr >> 16) as u8, (self.addr >> 8) as u8, self.addr as u8]
    }

    pub fn unserialize_len(addr: [u8; 2]) -> usize {
        (((addr[0] as u32) << 8) | addr[1] as u32) as usize
    }

    pub fn serialize_len(&self) -> Vec<u8> {
        vec![(self.data().len() >> 8) as u8, self.data().len() as u8]
    }
}
