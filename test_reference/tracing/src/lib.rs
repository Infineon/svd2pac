/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.7.0 on Fri, 8 May 2026 12:39:15 +0000
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "SVD Test for Rust PAC generator"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

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
