#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};
use cortex_m::interrupt;
use stm32f1xx_hal::gpio::gpioc::*;

#[rtic::app(device = stm32f1xx_hal::stm32, peripherals = true)]
mod app {
    use cortex_m::{asm, singleton};

    use embedded_hal::digital::v2::OutputPin;
    use stm32f1xx_hal::{
        gpio::{Output, PushPull, PC13},
        prelude::*,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led_usr: PC13<Output<PushPull>>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        asm::bkpt();
        let device = cx.device;
        let mut gpioc = device.GPIOC.split();

        let mut led_usr = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        loop {
            let _ = led_usr.set_high();
            for _ in 0..100000 {
                asm::nop()
            }
            let _ = led_usr.set_low();
            for _ in 0..100000 {
                asm::nop()
            }
        }

        (Shared {}, Local { led_usr }, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    interrupt::disable();

    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
