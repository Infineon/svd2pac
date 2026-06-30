/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 15:59:10 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common;
#[allow(unused_imports)]
use crate::common::{
    AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
    Write as _,
};
#[doc = r"Fake peripheral containing register with characters that may need to be escaped (and some UTF-8) when documentation is inserted."]
unsafe impl ::core::marker::Send for super::EscapeTest {}
unsafe impl ::core::marker::Sync for super::EscapeTest {}
impl super::EscapeTest {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "\\[\\]\\\"😀\"\\n\\a\\r\'𒀀𒀽"]
    #[inline(always)]
    pub fn register(&self) -> &'static self::RegisterT {
        unsafe { self::RegisterT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }
}

#[doc = "\\[\\]\\\"😀\"\\n\\a\\r\'𒀀𒀽"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Register {
    pub(crate) data: u8,
    pub(crate) mask: u8,
}

impl crate::common::RegisterValue for Register {
    type DataType = u8;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct RegisterT;
unsafe impl crate::common::AsPtr for RegisterT {}
impl crate::common::Reg<Register> for RegisterT {}

unsafe impl crate::common::Read<Register> for RegisterT {}
unsafe impl crate::common::Write<Register> for RegisterT {}
impl Register {
    #[doc = "\\[\\]\\\"😀\"\\n\\a\\r\'𒀀𒀽"]
    #[inline(always)]
    pub fn field(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        register::Field,
        register::Field,
        Register,
        common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            register::Field,
            register::Field,
            Register,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Register> for RegisterT {
    #[inline(always)]
    fn reset_value(&self) -> Register {
        Register::new(0)
    }
}
pub mod register {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Field(u8);

    impl Field {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Field {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Field {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Field> for u64 {
        #[inline(always)]
        fn from(value: Field) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Field {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Field {
        #[doc = "\\[\\]\\\"😀\"\\n\\a\\r\'𒀀𒀽"]
        pub const ENUM_VALUE: Self = Self(0);
    }
}
