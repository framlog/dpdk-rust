#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(concat_idents, thread_local)]

include!(concat!(env!("OUT_DIR"), "/dpdk.rs"));

#[macro_export]
macro_rules! rte_per_lcore {
    ($id: ident) => { unsafe { concat_idents!(per_lcore_, $id) } };
}

#[inline]
fn rte_get_master_lcore() -> u32 {
    let cfg = unsafe { *rte_eal_get_configuration() };
    cfg.master_lcore
}

#[inline]
fn rte_lcore_is_enabled(id: u32) -> bool {
    if id >= RTE_MAX_LCORE {
        return false;
    }
    let cfg = unsafe { *rte_eal_get_configuration() };
    cfg.lcore_role[id as usize] != rte_lcore_role_t_ROLE_OFF
}

#[inline]
pub fn rte_lcore_foreach<F>(mut f: F) where F: FnMut(u32) {
    for i in 0u32..RTE_MAX_LCORE {
        if rte_lcore_is_enabled(i) {
            f(i);
        }
    }
}

#[inline]
pub fn rte_lcore_foreach_slave<F>(mut f: F) where F: FnMut(u32) {
    let master_lcore_id = rte_get_master_lcore();
    for i in 0u32..RTE_MAX_LCORE {
        if rte_lcore_is_enabled(i) && i != master_lcore_id {
            f(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::os::raw::{c_int, c_char, c_void};
    use std::{mem, ptr};
    use super::*;

    extern fn lcore_hello(_arg: *mut c_void) -> c_int {
            let lcore_id = rte_per_lcore!(_lcore_id);
            println!("hello from core {}", lcore_id);
            0
    }

    #[test]
    fn dpdk_works() {
        let mut argv: *mut c_char = unsafe { mem::uninitialized() };
        let ret = unsafe { rte_eal_init(0, &mut argv) };
        if ret < 0 {
            panic!("Cannot init EAL: {}", ret);
        }

        rte_lcore_foreach_slave(|lcore_id| {
            unsafe { rte_eal_remote_launch(Some(lcore_hello), ptr::null_mut(), lcore_id) };
        });

        lcore_hello(ptr::null_mut());
        unsafe { rte_eal_mp_wait_lcore() };
    }
}
