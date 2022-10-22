use std::fs;

const PATCH_START: [u8; 5] = [0x50, 0x41, 0x54, 0x43, 0x48];
const PATCH_END: [u8; 3] = [0x45, 0x4f, 0x46];

#[derive(Debug)]
pub enum Record {
    Standard {
        offset: u32,
        size: u16,
        data: Vec<u8>,
    },
    RLE {
        offset: u32,
        length: u16,
        value: u8,
    },
}

#[derive(Default)]
pub struct Patcher {
    rom_data: Vec<u8>,
    patch_data: Vec<u8>,
    patch_pointer: usize,
    records: Vec<Record>,
}

impl Patcher {
    pub fn patch(&mut self, rom_path: String, patch_path: String, output_path: String) {
        self.rom_data = fs::read(rom_path).expect("Failed to open rom file");
        self.patch_data = fs::read(patch_path).expect("Failed to open patch files");

        self.verify_patch().expect("Patch file is invalid");

        self.read_records();

        for record in &self.records {
            match record {
                Record::Standard { offset, size, data } => {
                    for i in 0..(*size) {
                        self.rom_data[(offset + i as u32) as usize] = data[i as usize];
                    }
                }
                Record::RLE {
                    offset,
                    length,
                    value,
                } => {
                    for i in 0..(*length) {
                        self.rom_data[(offset + i as u32) as usize] = *value;
                    }
                }
            }
        }

        fs::write(output_path, &self.rom_data).expect("Failed to write output patched file");
    }

    fn read_records(&mut self) {
        self.patch_pointer = 5;
        while let Some(rec) = self.read_record() {
            self.records.push(rec);
        }

        let last_offset: u32 = match &self.records[self.records.len() - 1] {
            Record::Standard {
                offset,
                size,
                data: _,
            } => *offset + (*size as u32),
            Record::RLE {
                offset,
                length,
                value: _,
            } => *offset + (*length as u32),
        };

        if (last_offset + 1) as usize > self.rom_data.len() {
            self.rom_data
                .extend(vec![0; last_offset as usize - self.rom_data.len()]);
        }
    }

    fn verify_patch(&self) -> Result<(), &str> {
        if self.patch_data.len() < 5 {
            return Err("Patch file is too small");
        }

        if self.patch_data[0..5] != PATCH_START {
            return Err("Patch file beginning is invalid");
        }

        if self.patch_data[self.patch_data.len() - 3..] != PATCH_END {
            return Err("Patch file ending is invalid");
        }

        Ok(())
    }

    fn read_record(&mut self) -> Option<Record> {
        if self.patch_pointer + 3 > self.patch_data.len()
            || self.patch_data[self.patch_pointer..] == PATCH_END
        {
            return None;
        }

        let mut offset: u32 = (self.patch_data[self.patch_pointer] as u32) << 16;
        self.patch_pointer += 1;
        offset |= (self.patch_data[self.patch_pointer] as u32) << 8;
        self.patch_pointer += 1;
        offset |= self.patch_data[self.patch_pointer] as u32;
        self.patch_pointer += 1;

        let mut size: u16 = (self.patch_data[self.patch_pointer] as u16) << 8;
        self.patch_pointer += 1;
        size |= self.patch_data[self.patch_pointer] as u16;
        self.patch_pointer += 1;

        let is_rle = size == 0;
        if is_rle {
            let mut length: u16 = (self.patch_data[self.patch_pointer] as u16) << 8;
            self.patch_pointer += 1;
            length |= self.patch_data[self.patch_pointer] as u16;
            self.patch_pointer += 1;

            let value = self.patch_data[self.patch_pointer];
            self.patch_pointer += 1;

            Some(Record::RLE {
                offset,
                length,
                value,
            })
        } else {
            let mut data: Vec<u8> = Vec::with_capacity(size as usize);
            for _ in 0..size {
                data.push(self.patch_data[self.patch_pointer]);
                self.patch_pointer += 1;
            }

            Some(Record::Standard { offset, size, data })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_patch() {
        let patcher = Patcher {
            patch_data: vec![0x50, 0x41, 0x54, 0x43, 0x48, 0x00, 0x00, 0x45, 0x4f, 0x46],
            ..Default::default()
        };
        assert!(patcher.verify_patch().is_ok());
    }

    #[test]
    fn test_invalid_oatch() {
        // Small file
        let mut patcher = Patcher {
            patch_data: vec![0x50, 0x41, 0x51],
            ..Default::default()
        };
        assert!(patcher.verify_patch().is_err());

        // Invalid header
        patcher.patch_data = vec![0x50, 0x41, 0x51, 0x43, 0x48, 0x00];
        assert!(patcher.verify_patch().is_err());

        // Invalid EOF
        patcher.patch_data = vec![0x50, 0x41, 0x51, 0x43, 0x48, 0x00, 0x43, 0x4f, 0x46];
        assert!(patcher.verify_patch().is_err());
    }
}
