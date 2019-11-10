
use cty::*;

pub struct HTTPBlocked {
    pub saddr: u32,
    pub daddr: u32,
    pub sport: u16,
    pub dport: u16,
    pub header: [u8; 16],
}


#[cfg(not(feature = "probes"))]
impl HTTPBlocked {
    pub fn data(&self) -> &[u8] {
        unsafe {
            let base = self.data.as_ptr().add(self.offset as usize);
            core::slice::from_raw_parts(base, self.size as usize)
        }
    }
}

// This is where you should define the types shared by the kernel and user
// space, eg:
//
// #[repr(C)]
// #[derive(Debug)]
// pub struct SomeEvent {
//     pub pid: c_ulonglong,
//     ...
// }
