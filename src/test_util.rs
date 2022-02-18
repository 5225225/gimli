#![allow(missing_docs)]

use crate::Format;
use test_assembler::{Label, Section};

pub trait GimliSectionMethods {
    fn sleb(&mut self, val: i64) -> &mut Self;
    fn uleb(&mut self, val: u64) -> &mut Self;
    fn initial_length(&mut self, format: Format, length: &Label, start: &Label) -> &mut Self;
    fn word(&mut self, size: u8, val: u64) -> &mut Self;
    fn word_label(&mut self, size: u8, val: &Label) -> &mut Self;
}

impl GimliSectionMethods for Section {
    fn sleb(&mut self, mut val: i64) -> &mut Self {
        while val & !0x3f != 0 && val | 0x3f != -1 {
            self.D8(val as u8 | 0x80);
            val >>= 7;
        }
        self.D8(val as u8 & 0x7f)
    }

    fn uleb(&mut self, mut val: u64) -> &mut Self {
        while val & !0x7f != 0 {
            self.D8(val as u8 | 0x80);
            val >>= 7;
        }
        self.D8(val as u8)
    }

    fn initial_length(&mut self, format: Format, length: &Label, start: &Label) -> &mut Self {
        match format {
            Format::Dwarf32 => self.D32(length).mark(start),
            Format::Dwarf64 => self.D32(0xffff_ffff).D64(length).mark(start),
        }
    }

    fn word(&mut self, size: u8, val: u64) -> &mut Self {
        match size {
            4 => self.D32(val as u32),
            8 => self.D64(val),
            _ => panic!("unsupported word size"),
        }
    }

    fn word_label(&mut self, size: u8, val: &Label) -> &mut Self {
        match size {
            4 => self.D32(val),
            8 => self.D64(val),
            _ => panic!("unsupported word size"),
        }
    }
}
