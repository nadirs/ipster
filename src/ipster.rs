#[derive(Debug)]
pub struct Ips {
    buffer: Vec<u8>
}

impl Ips {
    pub fn new(buffer: Vec<u8>) -> Self {
        Ips {
            buffer: buffer
        }
    }

    pub fn diff(&self, edited: Vec<u8>) -> Vec<Patch> {
        let mut pairs = (0..).zip(self.buffer.iter().zip(edited));

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
        let mut bytes: Vec<u8> = vec![(self.addr >> 16) as u8, (self.addr >> 8) as u8, self.addr as u8];
        bytes.extend(&self.data);
        bytes
    }
}
