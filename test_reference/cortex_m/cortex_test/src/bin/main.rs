#![no_main]
#![no_std]

#[rtic::app(device = test_pac,peripherals=true,dispatchers = [UARTINT])]
mod app {
    use test_pac::*;
    use cortex_m;

    #[shared]
    struct Shared {}
    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let device: test_pac::Peripherals = ctx.device;
        unsafe {
            // it is possible to use standard way of accessing register through Peripheral type
            device.TIMER.bitfield_reg().init(|f| {
                f.bitfieldenumerated()
                    .set(timer::bitfield_reg::BitfieldEnumerated::GPIOA_0)
                    .bitfieldw()
                    .set(3)
            });
        }

        (
            Shared {},
            // initial values for the `#[local]` resources
            Local {},
        )
    }
    #[idle()]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = INT_FOO, local = [times: u32 = 0])]
    fn set_some_regs(_cx: set_some_regs::Context) {
        // Peripheral type is not required
        unsafe {
            TIMER.bitfield_reg().modify(|f| {
                f.bitfieldenumerated()
                    .set(timer::bitfield_reg::BitfieldEnumerated::GPIOA_0)
                    .bitfieldw()
                    .set(3)
            });
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
