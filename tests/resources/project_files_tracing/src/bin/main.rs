use test_pac::{timer::bitfield_reg, tracing, *};
fn read_fn(addr: usize, len: usize) -> u64 {
    println!("r addr=0x{:X}\tlen={:?}", addr, len);
    0x0
}
fn write_fn(addr: usize, len: usize, val: u64) {
    println!("w addr=0x{:X}\tlen={:?}\tval=0x{:X}", addr, len, val);
}
fn main() -> ! {
    let _ = tracing::set_read_fn(read_fn);
    let _ = tracing::set_write_fn(write_fn);
    unsafe {
        // Verbose read modify
        let register = TIMER.bitfield_reg();
        let register_value = register.read();
        let _bitfield_value = register_value.bitfieldr().get();
        // Modify value of register using fluent api
        let new_register_value = register_value.bitfieldrw().set(23).bitfieldw().set(23);
        register.write(new_register_value);

        // Register without bitfield
        let register_value = TIMER.nobitfield_reg().read();
        let _numeric_value = register_value.get();
        let modified_register_value = register_value.set(23);
        TIMER.nobitfield_reg().write(modified_register_value);

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

        // write rawG
        TIMER.bitfield_reg().write_raw(0x123);

        // Array of register bitfields
        let mut a = TIMER.bitfield_reg().read();
        for x in 0..2 {
            a = a.fieldarray(x).set(bitfield_reg::FieldArray::FALLING);
        }
        TIMER.bitfield_reg().write(a);

        // Array of registers
        let reg_array = TIMER.arrayreg();
        for reg in reg_array {
            let reg_val = reg.read();
            let old_val = reg_val.get();
            reg.write(reg_val.set(old_val + 1));
        }

        // Array of peripherals
        for peri in UART {
            peri.reg16bitenum().modify(|f| {
                f.bitfield9bitsenum()
                    .set(uart::reg16bitenum::Bitfield9BitsEnum::VAL_0)
            });
        }

        // Raw bitfield write. How to write an enumerated bitifield with a value listed in the enumeration
        TIMER.bitfield_reg().modify(|f| {
            f.bitfieldenumerated()
                .set(bitfield_reg::BitfieldEnumerated::new(3))
                .bitfieldw()
                .set(3)
        });
        {
            use tracing::insanely_unsafe;
            // Write a read-only register
            TIMER.sr().write_read_only(timer::Sr::default());

            // Read a write-only register
            let _ = TIMER.int().read_write_only();
        }
    }
    loop {}
}

#[cfg(test)]
mod test {
    use test::{reg_name::reg_name_from_addr, *};
    fn assert_regname(addr: usize, name_expected: &str) {
        match reg_name_from_addr(addr as u64) {
            Some(name) => {
                println!("name:{}, name_expected:{}", name, name_expected);
                assert!(name.contains(name_expected));
            }
            None => {
                panic!(
                    "Address: {} is not in map of register names. This should not happen.",
                    addr
                );
            }
        }
    }
    #[test]
    fn reg_name_test() {
        TIMER.arrayreg()[0].addr();
        assert_regname(TIMER.arrayreg()[0].addr(), "TIMER.arrayreg()[0]");
        assert_regname(TIMER.bitfield_reg().addr(), "TIMER.bitfield_reg()");
        assert_regname(TIMER.int().addr(), "TIMER.int()");
        assert_regname(TIMER.r#match().addr(), "TIMER.r#match()");
        assert_regname(TIMER.nobitfield_reg().addr(), "TIMER.nobitfield_reg()");
        assert_regname(TIMER.prescale_rd().addr(), "TIMER.prescale_rd()");
        assert_regname(TIMER.prescale_wr().addr(), "TIMER.prescale_wr()");
        assert_regname(TIMER.sr().addr(), "TIMER.sr()");
        assert_regname(UART[0].reg16bitenum().addr(), "UART[0].reg16bitenum()");
        assert_regname(UART[0].reg16bitraw().addr(), "UART[0].reg16bitraw()");
        assert_regname(UART[0].reg1_()[0].addr(), "UART[0].reg1_()[0]");
        assert_regname(UART[0].reg1_()[1].addr(), "UART[0].reg1_()[1]");
        assert_regname(UART[0].reg32bitraw().addr(), "UART[0].reg32bitraw()");
        assert_regname(UART[0].reg8bitraw().addr(), "UART[0].reg8bitraw()");
        assert_regname(UART[0].regbitfieldraw().addr(), "UART[0].regbitfieldraw()");
    }
}
