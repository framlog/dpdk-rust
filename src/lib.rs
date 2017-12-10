#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/dpdk.rs"));

struct LCore {
    idx: u32,
}

impl From<u32> for LCore {
    fn from(idx: u32) -> Self {
        LCore { idx }
    }
}

impl Iterator for LCore {
    type Item = LCore;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_char;
    use std::mem;
    use super::*;

    #[test]
    fn dpdk_works() {
        let mut argv: *mut c_char = unsafe { mem::uninitialized() };
        let ret = unsafe { rte_eal_init(0, &mut argv) };
        if ret < 0 {
            panic!("Cannot init EAL: {}", ret);
        }
    }
}
