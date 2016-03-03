pub mod files;

use std::cmp;
use std::io::Write;

#[derive(Debug)]
pub struct Ips {
    buffer: Vec<u8>
}

macro_rules! copy {
    ($a:expr, $v:expr) => {
        for (to, from) in $a.iter_mut().zip($v) {
            *to = *from;
        }
    }
}

impl Ips {
    pub fn new(buffer: Vec<u8>) -> Self {
        Ips {
            buffer: buffer
        }
    }

    pub fn diff(&self, change: Vec<u8>) -> Vec<Patch> {
        let mut pairs = (0..).zip(self.buffer.iter().zip(change));

        let mut patches: Vec<Patch> = vec![];
        let mut index = 0;
        let mut data = vec![];
        while let Some((offset, (&before, after))) = pairs.next() {
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
        patches
    }

    pub fn patch(&self, change: Vec<Patch>) -> Vec<u8> {
        let total_size = cmp::max(self.buffer.len(), change.iter().max_by_key(|x| x.addr as usize + x.data.len()).unwrap().addr as usize);
        let mut output = Vec::with_capacity(total_size);
        output.extend(&self.buffer);

        println!("before: {:?}", output);
        for patch in change {
            println!("{:?}", patch);
            let (_, mut skipped) = output.split_at_mut(patch.addr as usize);
            skipped.write(&patch.data);
        }
        println!("after: {:?}", output);
        output
    }

    pub fn unserialize_patches(&self, binary: Vec<u8>) -> Option<Vec<Patch>> {
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
            copy!(addr_array, addr_slice);
            let addr = Patch::unserialize_addr_array(addr_array);

            let mut len_array = [0; 2];
            copy!(len_array, len_slice);
            let len = Patch::unserialize_len(len_array);

            let (data, rest) = rest.split_at(len);

            if data.len() < len {
                // Malformed data
                return None;
            }

            let patch = Patch {
                addr: addr,
                data: data.to_vec()
            };

            patches.push(patch);

            slice = rest;

            let possible_eof: Vec<u8> = rest.iter().take(3).cloned().collect();
            if &possible_eof == b"EOF" {
                // We are done here
                break;
            }
        }

        Some(patches)
    }

    pub fn serialize_patches(&self, patches: Vec<Patch>) -> Vec<u8> {
        let patch_contents: Vec<u8> = patches.iter().flat_map(|p| p.bytes()).collect();
        let mut binary: Vec<u8> = "PATCH".bytes().collect();
        binary.extend(patch_contents);
        binary.extend("EOF".bytes().collect::<Vec<_>>());
        binary
    }
}

#[derive(Debug)]
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

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.serialize_addr();
        let len = self.serialize_len();
        bytes.extend(len);
        bytes.extend(&self.data);
        bytes
    }

    pub fn unserialize_addr_array(addr: [u8; 3]) -> u32 {
        ((addr[0] as u32) << 16) | ((addr[1] as u32) << 8) | addr[2] as u32
    }

    pub fn unserialize_addr_bytes(a3: u8, a2: u8, a1: u8) -> u32 {
        ((a3 as u32) << 16) | ((a2 as u32) << 8) | a1 as u32
    }

    pub fn serialize_addr(&self) -> Vec<u8> {
        vec![(self.addr >> 16) as u8, (self.addr >> 8) as u8, self.addr as u8]
    }

    pub fn unserialize_len(addr: [u8; 2]) -> usize {
        (((addr[0] as u32) << 8) | addr[1] as u32) as usize
    }

    pub fn serialize_len(&self) -> Vec<u8> {
        vec![(self.data.len() >> 8) as u8, self.data.len() as u8]
    }

    //pub fn apply_to(&self, &[mut u8]
}
