use cty::*;
​
// This is where you should define the types shared by the kernel and user
// space, eg:
//
#[repr(C)]
#[derive(Debug)]
pub struct HTTPBlocked {
    pub saddr: u32,
    pub daddr: u32,
    pub sport: u32,
}
