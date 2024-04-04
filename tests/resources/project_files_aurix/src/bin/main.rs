#![no_std]
#![no_main]

use arm_example::{timer::bitfield_reg, *};
use tc162_rt::entry;
entry!(main);
fn main() -> ! {
    unsafe {
        //Verbose read modify
        let register = TIMER.bitfield_reg();
        let register_value = register.read();
        let _bitfield_value = register_value.bitfieldr().get();
        // Modify value of register using fluent api
        let new_register_value = register_value.bitfieldrw().set(23).bitfieldw().set(23);
        register.write(new_register_value);

        // Register without bitfield
        let register_value = TIMER.nobitfield_reg().read();
        let _numeric_value = register_value.get();
        let _modified_register_value = register_value.set(23);

        // Modify
        TIMER.bitfield_reg().modify(|f| {
            f.bitfieldenumerated()
                .set(bitfield_reg::BitfieldEnumerated::GPIOA_0)
                .bitfieldw()
                .set(3)
        });

        // init
        TIMER.bitfield_reg().init(|f| {
            f.bitfieldenumerated()
                .set(bitfield_reg::BitfieldEnumerated::GPIOA_0)
                .bitfieldw()
                .set(3)
        });

        // Modify atomic only 32bit registers
        TIMER.bitfield_reg().modify_atomic(|f| {
            f.bitfieldenumerated()
                .set(bitfield_reg::BitfieldEnumerated::GPIOA_0)
                .bitfieldw()
                .set(25)
        });

        // Array of register bitfields
        let mut a = TIMER.bitfield_reg().read();
        for x in 0..2 {
            a = a.fieldarray(x).set(bitfield_reg::FieldArray::FALLING);
        }
        TIMER.bitfield_reg().write(a);

        // Array of registers
        let r = TIMER.arrayreg();
        for r in r.iter().take(2) {
            let reg_val = r.read();
            let old_val = reg_val.get();
            r.write(reg_val.set(old_val + 1));
        }

        TIMER.arrayreg()[2].modify_atomic(|f| f.set(24));

        // Array of peripherals
        for peri in UART {
            peri.reg16bitenum().modify(|f| {
                f.bitfield9bitsenum()
                    .set(uart::reg16bitenum::Bitfield9BitsEnum::VAL_0)
            });
        }

        //Raw bitfield write. How to write an enumerated bitifield with a value listed in the enumeration
        TIMER.bitfield_reg().modify(|f| {
            f.bitfieldenumerated()
                .set(bitfield_reg::BitfieldEnumerated::new(3))
                .bitfieldw()
                .set(3)
        });

        //Tests related to csfr registers
        let register = CSFR_CPU.biv();
        let register_value = register.read();
        let _biv_value = register_value.biv().get();
        
        // Modify value of register using fluent api
        let new_register_value = register_value.biv().set(23).vss().set(true);
        register.write(new_register_value);

         // Modify
         CSFR_CPU.biv().modify(|f| {
            f.vss()
                .set(true)
                .biv()
                .set(3)
        });
    }
    loop {}
}
