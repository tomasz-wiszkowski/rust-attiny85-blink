#![no_std]
#![no_main]
#![feature(core_intrinsics)]

extern crate panic_halt;
use avrd::current::*;
use core::intrinsics::volatile_store;

#[arduino_hal::entry]
fn main() -> ! {
    // 64 evenly sampled sine values in range 0..200.
    let points: [u8; 64] = [
        100, 110, 120, 129, 138, 147, 156, 163, 171, 177, 183, 188, 192, 196, 198, 200, 200, 200,
        198, 196, 192, 188, 183, 177, 171, 163, 156, 147, 138, 129, 120, 110, 100, 90, 80, 71, 62,
        53, 44, 37, 29, 23, 17, 12, 8, 4, 2, 0, 0, 0, 2, 4, 8, 12, 17, 23, 29, 37, 44, 53, 62, 71,
        80, 90,
    ];

    // All bits are outputs.
    unsafe {
        volatile_store(DDRB, 0x1f);
    }

    loop {
        points.iter().for_each(|p| {
            (0..100).for_each(|_| {
                // Set all pins low.
                unsafe {
                    volatile_store(PORTB, 0);
                }
                arduino_hal::delay_us((*p).into());

                // Set all pins high.
                unsafe {
                    volatile_store(PORTB, 0x1f);
                }
                arduino_hal::delay_us((200 - p).into());
            });
        });
    }
}
