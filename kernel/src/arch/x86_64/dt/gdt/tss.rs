//! Task State Segment

use core::ptr::addr_of;

static mut TASK_STATE_SEGMENT: TSS = TSS::null();

pub fn get_addr() -> u64 {
    addr_of!(TASK_STATE_SEGMENT) as *const TSS as u64
}

#[repr(C, packed)]
pub struct TSS {
    reserved0: u32,

    rsp: [u64; 3],

    reserved1: u64,

    ist: [u64; 7],

    reserved2: u64,
    reserved3: u16,

    io_map_base_address: u16,
}
impl TSS {
    const fn null() -> Self {
        Self {
            reserved0: 0,
            rsp: [0; 3],
            reserved1: 0,
            ist: [0; 7],
            reserved2: 0,
            reserved3: 0,
            io_map_base_address: 0,
        }
    }
}
