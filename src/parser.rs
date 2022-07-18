use std::fs;
use std::mem;

use nano_leb128::ULEB128;

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

    pub fn is_finished(&mut self) -> bool {
        self.data.len() == self.pos
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
        Short::from_le_bytes(
            self.get_block(mem::size_of::<Short>())
                .try_into()
                .expect("Error in getting Short from osu!.db"),
        )
    }

    pub fn get_int(&mut self) -> Int {
        Int::from_le_bytes(
            self.get_block(mem::size_of::<Int>())
                .try_into()
                .expect("Error in getting Int from osu!.db"),
        )
    }

    pub fn get_long(&mut self) -> Long {
        Long::from_le_bytes(
            self.get_block(mem::size_of::<Long>())
                .try_into()
                .expect("Error in getting Long from osu!.db"),
        )
    }

    pub fn get_byte(&mut self) -> Byte {
        Byte::from_le_bytes(
            self.get_block(mem::size_of::<Byte>())
                .try_into()
                .expect("Error in getting Byte from osu!.db"),
        )
    }

    pub fn get_single(&mut self) -> Single {
        Single::from_le_bytes(
            self.get_block(mem::size_of::<Single>())
                .try_into()
                .expect("Error in getting Singe from osu!.db"),
        )
    }

    pub fn get_double(&mut self) -> Double {
        Double::from_le_bytes(
            self.get_block(mem::size_of::<Double>())
                .try_into()
                .expect("Error in getting Double from osu!.db"),
        )
    }

    pub fn get_bool(&mut self) -> Bool {
        Bool::from_le_bytes(
            self.get_block(mem::size_of::<Bool>())
                .try_into()
                .expect("Error in getting Bool from osu!.db"),
        )
    }

    pub fn get_datetime(&mut self) -> DateTime {
        DateTime::from_le_bytes(
            self.get_block(mem::size_of::<DateTime>())
                .try_into()
                .expect("Error in getting DateTime from osu!.db"),
        )
    }

    pub fn get_string(&mut self) -> String {
        let is_present = self.get_bool();
        if is_present == 0 {
            return "".to_string();
        }

        let (uleb128, shift) =
            ULEB128::read_from(&self.data[self.pos..self.data.len()]).expect("Reading ULEB128");

        let str_size = u64::from(uleb128) as usize;

        let r = String::from_utf8_lossy(&self.data[self.pos + shift..self.pos + shift + str_size])
            .into_owned();

        self.pos += shift + str_size;
        r
    }

    fn get_int_double_pair(&mut self) -> IntDoublePair {
        IntDoublePair {
            flag: self.get_byte(),
            int: self.get_int(),
            flag2: self.get_byte(),
            double: self.get_double(),
        }
    }

    pub fn get_int_double_pairs(&mut self) -> Vec<IntDoublePair> {
        let mut v = Vec::default();
        let p_num = self.get_int();
        for _ in 0..p_num {
            v.push(self.get_int_double_pair())
        }
        v
    }

    pub fn get_timing_points(&mut self) -> Vec<TimingPoint> {
        let mut v = Vec::default();
        let tp_num = self.get_int();
        for _ in 0..tp_num {
            v.push(TimingPoint {
                bpm: self.get_double(),
                offset: self.get_double(),
                is_inherited: self.get_bool(),
            })
        }
        v
    }
}
