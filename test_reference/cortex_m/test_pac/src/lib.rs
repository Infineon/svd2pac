/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 15:58:05 +0000
#![no_std]
#![allow(non_camel_case_types)]
#![doc = "SVD Test for Rust PAC generator"]
#[doc(hidden)]
pub mod common;
#[doc(hidden)]
pub use crate::common::{
    AsPtr as _, Modify as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
    ResetValue as _, Write as _,
};

#[cfg(feature = "derivedtest")]
pub mod derivedtest;
#[cfg(feature = "dimindexperi")]
pub mod dimindexperi;
#[cfg(feature = "escapetest")]
pub mod escapetest;
#[cfg(feature = "foo")]
pub mod foo;
#[cfg(feature = "hdrstrderhdrstruct")]
pub mod hdrstructderivedhdr;
#[cfg(feature = "hdrstrder")]
pub mod hdrstructderivedp33;
#[cfg(feature = "hasheaderstruct")]
pub mod headerstruct;
#[cfg(feature = "p33")]
pub mod p33;
#[cfg(feature = "timer")]
pub mod timer;
#[cfg(feature = "uart")]
pub mod uart;

#[cfg(feature = "timer")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer {
    ptr: *mut u8,
}
#[cfg(feature = "timer")]
pub const TIMER: self::Timer = self::Timer {
    ptr: 0x40010000u32 as _,
};
#[cfg(feature = "uart")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
#[cfg(feature = "uart")]
pub const UART: [self::Uart; 3] = [
    self::Uart {
        ptr: 0x50000000u32 as _,
    },
    self::Uart {
        ptr: 0x50001004u32 as _,
    },
    self::Uart {
        ptr: 0x50002008u32 as _,
    },
];
#[cfg(feature = "foo")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Foo {
    ptr: *mut u8,
}
#[cfg(feature = "foo")]
pub const FOO: self::Foo = self::Foo {
    ptr: 0x60000000u32 as _,
};
#[cfg(feature = "escapetest")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EscapeTest {
    ptr: *mut u8,
}
#[cfg(feature = "escapetest")]
pub const ESCAPETEST: self::EscapeTest = self::EscapeTest {
    ptr: 0x70000000u32 as _,
};
#[cfg(feature = "derivedtest")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DerivedTest {
    ptr: *mut u8,
}
#[cfg(feature = "derivedtest")]
pub const DERIVEDTEST: self::DerivedTest = self::DerivedTest {
    ptr: 0xa0000u32 as _,
};
#[cfg(feature = "p33")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P33 {
    ptr: *mut u8,
}
#[cfg(feature = "p33")]
pub const P33: self::P33 = self::P33 {
    ptr: 0x70100000u32 as _,
};
#[cfg(feature = "derivedperipheral")]
pub const DERIVEDPERIPHERAL: self::P33 = self::P33 {
    ptr: 0x70200000u32 as _,
};
#[cfg(feature = "hasheaderstruct")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HeaderStruct {
    ptr: *mut u8,
}
#[cfg(feature = "hasheaderstruct")]
pub const HASHEADERSTRUCT: self::HeaderStruct = self::HeaderStruct {
    ptr: 0x70300000u32 as _,
};
#[cfg(feature = "hdrstrder")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HdrStructDerivedP33 {
    ptr: *mut u8,
}
#[cfg(feature = "hdrstrder")]
pub const HDRSTRDER: self::HdrStructDerivedP33 = self::HdrStructDerivedP33 {
    ptr: 0x70400000u32 as _,
};
#[cfg(feature = "hdrstrderhdrstruct")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HdrStructDerivedHdr {
    ptr: *mut u8,
}
#[cfg(feature = "hdrstrderhdrstruct")]
pub const HDRSTRDERHDRSTRUCT: self::HdrStructDerivedHdr = self::HdrStructDerivedHdr {
    ptr: 0x70500000u32 as _,
};
#[cfg(feature = "dimindexperi")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DimIndexPeri {
    ptr: *mut u8,
}
#[cfg(feature = "dimindexperi")]
pub const DIMINDEXPERI: self::DimIndexPeri = self::DimIndexPeri {
    ptr: 0x70700000u32 as _,
};

pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub mod interrupt_handlers {
    extern "C" {
        pub fn TIMER0();
        pub fn UARTINT();
        pub fn INT_FOO();
        pub fn INTERRUPT();
    }
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 43] = [
    Vector {
        _handler: interrupt_handlers::TIMER0,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: interrupt_handlers::UARTINT,
    },
    Vector {
        _handler: interrupt_handlers::INT_FOO,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: interrupt_handlers::INTERRUPT,
    },
];
#[doc = "Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "Timer 0 interrupt"]
    TIMER0 = 0,

    #[doc = "Uart interrupt"]
    UARTINT = 2,

    #[doc = "Foo interrupt"]
    INT_FOO = 3,

    #[doc = "\\[\\]\\\"😀\"\\n\\a\n\t\t\t\t\t\tmulti-line\n\t\t\t\t\t\t\\r\'𒀀𒀽"]
    INTERRUPT = 42,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}

#[allow(non_snake_case)]
/// Required for compatibility with RTIC and other frameworks
pub struct Peripherals {
    #[cfg(feature = "timer")]
    pub TIMER: self::Timer,
    #[cfg(feature = "uart")]
    pub UART: [self::Uart; 3],
    #[cfg(feature = "foo")]
    pub FOO: self::Foo,
    #[cfg(feature = "escapetest")]
    pub ESCAPETEST: self::EscapeTest,
    #[cfg(feature = "derivedtest")]
    pub DERIVEDTEST: self::DerivedTest,
    #[cfg(feature = "p33")]
    pub P33: self::P33,
    #[cfg(feature = "derivedperipheral")]
    pub DERIVEDPERIPHERAL: self::P33,
    #[cfg(feature = "hasheaderstruct")]
    pub HASHEADERSTRUCT: self::HeaderStruct,
    #[cfg(feature = "hdrstrder")]
    pub HDRSTRDER: self::HdrStructDerivedP33,
    #[cfg(feature = "hdrstrderhdrstruct")]
    pub HDRSTRDERHDRSTRUCT: self::HdrStructDerivedHdr,
    #[cfg(feature = "dimindexperi")]
    pub DIMINDEXPERI: self::DimIndexPeri,
}

impl Peripherals {
    /// Returns Peripheral struct multiple times
    /// Required for compatibility with RTIC and other frameworks
    #[inline]
    pub fn take() -> Option<Self> {
        Some(Self::steal())
    }

    /// Returns Peripheral struct multiple times
    /// Required for compatibility with RTIC and other frameworks
    #[inline]
    pub fn steal() -> Self {
        Peripherals {
            #[cfg(feature = "timer")]
            TIMER: crate::TIMER,
            #[cfg(feature = "uart")]
            UART: crate::UART,
            #[cfg(feature = "foo")]
            FOO: crate::FOO,
            #[cfg(feature = "escapetest")]
            ESCAPETEST: crate::ESCAPETEST,
            #[cfg(feature = "derivedtest")]
            DERIVEDTEST: crate::DERIVEDTEST,
            #[cfg(feature = "p33")]
            P33: crate::P33,
            #[cfg(feature = "derivedperipheral")]
            DERIVEDPERIPHERAL: crate::DERIVEDPERIPHERAL,
            #[cfg(feature = "hasheaderstruct")]
            HASHEADERSTRUCT: crate::HASHEADERSTRUCT,
            #[cfg(feature = "hdrstrder")]
            HDRSTRDER: crate::HDRSTRDER,
            #[cfg(feature = "hdrstrderhdrstruct")]
            HDRSTRDERHDRSTRUCT: crate::HDRSTRDERHDRSTRUCT,
            #[cfg(feature = "dimindexperi")]
            DIMINDEXPERI: crate::DIMINDEXPERI,
        }
    }
}
