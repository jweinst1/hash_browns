use crate::numbers::*;

pub struct MashPipe {
	or_byts:[u8;10],
	and_byts:[u8;10]
}

impl Default for MashPipe {
    fn default() -> Self {
    	MashPipe {
    		or_byts:[108, 92, 45, 183, 121, 8, 211, 165, 63, 243],
    		and_byts:[200, 34, 15, 131, 249, 48, 178, 39, 92, 191]
    	}
    }
}

/// This is a hash function that emphasizes piping bytes 
/// through a preset stream.
/// # Examples
/// Here is an example how to use it
/// ```rust
/// use hash_browns::pipes::pipe_u32;
/// println!("the hash of {} is {}", "foo", pipe_u32("foo"));
/// ```
pub fn pipe_u32(data:&str) -> u32 {
	let pipe:MashPipe = Default::default();
	let mut bit_bin:[u8;25] = [51;25];
	let mut state = 0;
	for by in data.as_bytes() {
		bit_bin[state] ^= !by;
		if state == 25 {
			state = 0;
		} else {
			state += 1;
		}
	}
	for bit in pipe.or_byts.iter() {
		bit_bin[state] |= bit;
		if state == 25 {
			state = 0;
		} else {
			state += 1;
		}
	}
	state = 0;

	for bit in pipe.and_byts.iter() {
		bit_bin[state] &= bit;
		if state == 25 {
			state = 0;
		} else {
			state += 1;
		}
	}
	return ((bit_bin[9] as u32) << 24) |
	       ((bit_bin[2] as u32) << 16) |
	       ((bit_bin[13] as u32) << 8) |
	       (bit_bin[21] as u32);
}

#[cfg(test)]
mod pipes_tests {
    // import all local module components
    use crate::pipes::*;
    #[test]
    fn pipe_u32_works() {
        let a1 = pipe_u32("hello!");
        let a2 = pipe_u32("World!");
        println!("hash a {} hash b {}", a1, a2);
        assert!(a1 != a2);
    }

}

