use std::time::{SystemTime};

pub fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[cfg(test)]
mod nutils_tests {
	use std::thread::sleep;
	use std::time::{Duration};
	use crate::nutils::get_sys_time_in_secs;
	#[test]
	fn get_sys_time_in_secs_works() {
		let t1 = get_sys_time_in_secs();
		sleep(Duration::new(1, 0));
		let t2 = get_sys_time_in_secs();
		assert!(t1 != t2);
	}
}
