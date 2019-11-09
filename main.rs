#![no_builtins]
#![no_std]
#![no_main]

// block_http

use cty::*;

use redbpf_macros::*;
use redbpf_probes::bindings::*;
use redbpf_probes::xdp::PerfMap;
use redbpf_probes::xdp::{Transport, XdpAction, XdpContext};

use ingraind_probes::block_http::HTTPBlocked;

// Use the types you're going to share with userspace, eg:
// use ingraind-probes::block-http::SomeEvent;

program!(0xFFFFFFFE, "GPL");

// The maps and probe functions go here, eg:
//
#[map("events")]
static mut events: PerfMap<HTTPBlocked> = PerfMap::with_max_entries(1024);

#[xdp("block_http")]
pub extern "C" fn probe(ctx: *mut xdp_md) -> XdpAction {
    let ip = match (ctx.ip(), ctx.transport()) {
        (Some(i), Some(_t @ Transport::TCP(_))) => unsafe { *i },
        _ => return XdpAction::Pass,
    };

    let data = match ctx.data() {
        Some(data) => data,
        None => return XdpAction::Pass,
    };

    let http = ['H', 'T', 'T', 'P', '/', '1', '.', '1'];
    let iters = http.len();

    let mut decision = 1;
    if let Some(header) = data.slice(iters) {
        for i in 0..8 {
            if header[i] != http[i] as u8 {
                decision = 0;
            }
        }
    } else {
        decision = 0;
    };

    if decision == 1 {
        let event = HTTPBlocked {
            saddr: ip.saddr,
            daddr: ip.daddr,
            sport: 0,
        };
        unsafe { events.insert(&ctx, event, 0) };
    }

    if decision == 1 {
        XdpAction::Drop
    } else {
        XdpAction::Pass
    }
}
