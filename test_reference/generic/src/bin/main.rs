use test_pac::*;
fn main() -> ! {
    unsafe {
        // Verbose read modify
        let register = TIMER.bitfield_reg();
        let register_value = register.read();
        let _bitfield_value = register_value.bitfieldr().get();
        // Modify value of register using fluent api
        let new_register_value = register_value.bitfieldrw().set(23).bitfieldw().set(23);
        register.write(new_register_value);

        // Create a register value from raw value
        let register_value = timer::BitfieldReg::new(0x123);
        register.write(register_value);

        // Register without bitfield
        let register_value = TIMER.nobitfield_reg().read();
        let _numeric_value = register_value.get();
        let modified_register_value = register_value.set(23);
        TIMER.nobitfield_reg().write(modified_register_value);

        // Modify
        TIMER.bitfield_reg().modify(|f| {
            f.bitfieldenumerated()
                .set(timer::bitfield_reg::BitfieldEnumerated::GPIOA_0)
                .bitfieldw()
                .set(3)
        });

        // init
        TIMER.bitfield_reg().init(|f| {
            f.bitfieldenumerated()
                .set(timer::bitfield_reg::BitfieldEnumerated::GPIOA_0)
                .bitfieldw()
                .set(3)
        });

        // write raw
        TIMER.bitfield_reg().write_raw(0x123);

        // Array of register bitfields
        let mut a = TIMER.bitfield_reg().read();
        for x in 0..2 {
            a = a
                .fieldarray(x)
                .set(timer::bitfield_reg::FieldArray::FALLING);
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

        // Raw bitfield write. How to write an enumerated bitfield by passing an integer literal
        TIMER
            .bitfield_reg()
            .modify(|f| f.bitfieldenumerated().set(3.into()).bitfieldw().set(3));

        // Use register marked with alternateGroup
        TIMER.bitfield_reg_alt_group().modify(|f| f.set(32));

        // Set and get raw values
        TIMER.bitfield_reg().modify(|f| f.set_raw(32));
        let _: u32 = TIMER.bitfield_reg().read().get_raw();

        // Get mask and offset for a register bitfield
        let register_bitfield = TIMER.bitfield_reg().read().bitfieldr();
        let _offset = register_bitfield.offset();
        let _mask = register_bitfield.mask();

        // Get mask and offset for a register bitfield fro boolean
        let register_bitfield = TIMER.bitfield_reg().read().boolr();
        let _offset = register_bitfield.offset();
        let _mask = register_bitfield.mask();

        // Test correct handling of SVD names starting with non XID_Start
        // characters or collide with Rust keywords.
        let value = FOO.r#in().read();
        FOO.r#in()
            .write(value._self().set(foo::r#in::_Self::_1_VALUE));

        // Test 64Bit register
        TIMER
            .register64bit()
            .modify(|r| r.boolean().set(crate::timer::register64bit::Boolean::FALSE));

        // Test cluster array
        TIMER.clusterdim()[0].cr().modify(|r| r.field1().set(0));
        for elem in TIMER.clusterdim() {
            elem.cr().modify(|r| r.field1().set(1));
        }

        // Demonstrating the usage of enumerated values with read-only, write-only, and read-write usage
        let reg_value = UART[1].regenumvalue().read();
        let _bitfield_value: uart::regenumvalue::OnlyReadEnumRead =
            reg_value.only_read_enum().get();
        let _ = reg_value.only_read_enum().set(1);
        let _ = reg_value
            .only_write_enum()
            .set(uart::regenumvalue::OnlyWriteEnumWrite::VALUE_1);
        let _: u8 = reg_value.only_write_enum().get();
        let _: uart::regenumvalue::ReadWriteEnumSplitBinaryRead =
            reg_value.read_write_enum_split_binary().get();
        let _ = reg_value
            .read_write_enum_split_binary()
            .set(uart::regenumvalue::ReadWriteEnumSplitBinaryWrite::VALUE_0);
        let _: uart::regenumvalue::ReadWriteEnumSplitBinaryRead =
            reg_value.read_write_enum_split_binary().get();

        // Demonstrating usage of register array with dimIndex tag.
        DIMINDEXPERI
            .clu3st()
            .aregd()
            .modify(|f| f.arraybitfield_c().set(0x2));
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
