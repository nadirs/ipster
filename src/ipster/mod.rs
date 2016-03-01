pub mod files;

#[derive(Debug)]
pub struct Ips {
    buffer: Vec<u8>
}

macro_rules! copy {
    ($a:expr, $v:expr) => {
        for (to, from) in $a.iter_mut().zip(&$v as &Vec<u8>) {
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
        let output = Vec::new();
        for patch in change {
        }
        output
    }

    pub fn unserialize_patches(&self, binary: Vec<u8>) -> Option<Vec<Patch>> {
        let mut patches: Vec<Patch> = Vec::new();

        let header: Vec<u8> = binary.iter().take(5).cloned().collect();

        if &header != b"PATCH" {
            return None;
        }

        let mut rest: Vec<u8> = binary.iter().skip(5).cloned().collect();

        loop {
            if rest.len() <= 5 {
                break;
            }
            println!("{:?} bytes", rest.len());

            let mut addr_be = [0; 3];
            copy!(addr_be, rest.iter().take(3).cloned().collect::<Vec<u8>>());
            let addr = Patch::unserialize_addr_array(addr_be);
            rest = rest.iter().skip(3).cloned().collect();
            println!("addr: {:?}", addr);

            let mut len_be: [u8; 2] = [0; 2];
            copy!(len_be, rest.iter().take(2).cloned().collect());
            let len = Patch::unserialize_len(len_be);
            rest = rest.iter().skip(2).cloned().collect();
            println!("len: {:?}", len);

            let data = rest.iter().take(len).cloned().collect();
            patches.push(Patch {
                addr: addr,
                data: data
            });
            rest = rest.iter().skip(len).cloned().collect();
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
}
