use std::ops::{Index, IndexMut};

struct ByteInt32 {
    bytes:[u8;4]
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
