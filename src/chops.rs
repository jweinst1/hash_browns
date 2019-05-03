use crate::numbers::*;

/// Enum that represents seeds for hashing functions. 
/// Such as,
/// ```
/// use hash_browns::chops::ChopSeeds;
/// let seed = ChopSeeds::Straight as u32;
/// ```
/// In the above example, the enum case can be cast as 
/// an unsigned 32bit integer.
pub enum ChopSeeds {
	Straight = 0xf329ab19,
	Straight64 = 0xd39cf32af329ab
}

/// Will hash a str into a 32 bit unsigned integer.
/// This is a fast hash function not intended for 
/// cryptographic purposes
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
/// Will hash a str into a 64 bit unsigned integer
pub fn chop_u64(data:&str) -> u64 {
	let mut block = ByteInt64::from(ChopSeeds::Straight64 as u64);
	let mut state = 0;
	for b in data.as_bytes() {
		block[state] ^= b;
		if state == 7 {
			state = 0;
		} else {
			state += 1;
		}
	}
	return block.to_u64();
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

    #[test]
    fn chop_u64_works() {
        let a1 = chop_u64("hello!");
        let a2 = chop_u64("World!");
        assert!(a1 != a2);
    }

}
