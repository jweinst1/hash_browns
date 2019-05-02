use std::ops::{Index, IndexMut};


pub trait ToU32Int {
    fn to_u32(&self) -> u32;
}

pub trait ToU64Int {
    fn to_u32(&self) -> u64;
}

pub struct ByteInt32 {
    bytes:[u8;4]
}

pub struct ByteInt64 {
    bytes:[u8;8]
}

impl From<u32> for ByteInt32 {
    fn from(num:u32) -> ByteInt32 {
        ByteInt32 {
            bytes:[((num >> 24) & 0xFF) as u8, 
                   ((num >> 16) & 0xFF) as u8, 
                   ((num >> 8) & 0xFF) as u8, 
                   (num & 0xFF) as u8]
        }
    }
}

impl From<u64> for ByteInt64 {
    fn from(num:u64) -> ByteInt64 {
        ByteInt64 {
            bytes:[((num >> 56) & 0xFF) as u8,
                   ((num >> 48) & 0xFF) as u8,
                   ((num >> 40) & 0xFF) as u8,
                   ((num >> 32) & 0xFF) as u8,
                   ((num >> 24) & 0xFF) as u8, 
                   ((num >> 16) & 0xFF) as u8, 
                   ((num >> 8) & 0xFF) as u8, 
                   (num & 0xFF) as u8]
        }
    }
}

impl Index<usize> for ByteInt32 {
    type Output = u8;
    fn index<'a>(&'a self, index: usize) -> &'a u8 {
        & self.bytes[index]
    }
}

impl IndexMut<usize> for ByteInt32 {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.bytes[index]
    }
}

impl ToU32Int for ByteInt32 {
    fn to_u32(&self) -> u32 {
        return ((self.bytes[0] as u32) << 24) |
        ((self.bytes[1] as u32) << 16) |
        ((self.bytes[2] as u32) << 8)  |
        (self.bytes[3] as u32);
    }
}

#[cfg(test)]
mod nutils_tests {
    // import all local module components
    use crate::numbers::*;
    #[test]
    fn byte_int_32_to_u32_works() {
        let c = ByteInt32::from(78);
        assert_eq!(c.to_u32(), 78);
    }
}
