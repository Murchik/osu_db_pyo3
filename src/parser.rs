use std::fs;
use std::mem;

use crate::osu_types::*;

pub struct Parser {
    data: Vec<u8>,
    pos: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            pos: Default::default(),
        }
    }

    pub fn read(&mut self, path: String) {
        self.data = fs::read(path).expect("Error reading a file");
        self.pos = 0;
    }

    fn get_block(&mut self, size: usize) -> &[u8] {
        let r = &self.data[self.pos..self.pos + size];
        self.pos += size;
        &r
    }

    pub fn get_short(&mut self) -> Short {
        Short::from_le_bytes(self.get_block(mem::size_of::<Short>()).try_into().unwrap())
    }

    pub fn get_int(&mut self) -> Int {
        Int::from_le_bytes(self.get_block(mem::size_of::<Int>()).try_into().unwrap())
    }

    pub fn get_long(&mut self) -> Long {
        Long::from_le_bytes(self.get_block(mem::size_of::<Long>()).try_into().unwrap())
    }

    pub fn get_byte(&mut self) -> Byte {
        Byte::from_le_bytes(self.get_block(mem::size_of::<Byte>()).try_into().unwrap())
    }

    pub fn get_single(&mut self) -> Single {
        Single::from_le_bytes(self.get_block(mem::size_of::<Single>()).try_into().unwrap())
    }

    pub fn get_double(&mut self) -> Double {
        Double::from_le_bytes(self.get_block(mem::size_of::<Double>()).try_into().unwrap())
    }

    pub fn get_bool(&mut self) -> Bool {
        Bool::from_le_bytes(self.get_block(mem::size_of::<Bool>()).try_into().unwrap())
    }

    pub fn get_datetime(&mut self) -> DateTime {
        DateTime::from_le_bytes(
            self.get_block(mem::size_of::<DateTime>())
                .try_into()
                .unwrap(),
        )
    }

    pub fn get_string(&mut self) -> String {
        let is_present = self.get_bool();
        if is_present == 0 {
            return "".to_string();
        }
        let str_size = self.get_byte() as usize;
        let r = String::from_utf8(self.data[self.pos..self.pos + str_size].to_vec()).unwrap();
        self.pos += str_size;
        r
    }
}
