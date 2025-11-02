#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action::{self},
    macros::xdp,
    programs::XdpContext,
};
use nano_block_ebpf::check_packet;

#[xdp]
pub fn nano_block_example(ctx: XdpContext) -> u32 {
    match try_nano_block_example(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_nano_block_example(ctx: XdpContext) -> Result<u32, ()> {
    let result = check_packet(&ctx);

    Ok(result)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
