use crate::numbers::*;

pub enum ChopSeeds {
	Straight = 0xf329ab19
}

/// Will hash a string into a 32 bit 
pub fn chop_u32(data:&str) -> u32 {
	let mut block = ByteInt32::from(ChopSeeds::Straight as u32);
	let mut state = 0;
	for b in data.as_bytes() {
		block[state] ^= b;
		if state == 3 {
			state = 0;
		} else {
			state += 1;
		}
	}
	return block.to_u32();
}

#[cfg(test)]
mod chops_tests {
    // import all local module components
    use crate::chops::*;
    #[test]
    fn chop_u32_works() {
        let a1 = chop_u32("hello!");
        let a2 = chop_u32("World!");
        assert!(a1 != a2);
    }

}
