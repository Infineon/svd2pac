/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 14:40:30 +0000

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
#[doc = r"Fake peripheral containing registers with names starting with non XID_Start characters that cannot be directly used as Rust identifier names."]
unsafe impl ::core::marker::Send for super::Foo {}
unsafe impl ::core::marker::Sync for super::Foo {}
impl super::Foo {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "FOO Input Register"]
    #[inline(always)]
    pub fn r#in(&self) -> &'static self::InT {
        unsafe { self::InT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }
}

#[doc = "FOO Input Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In {
    pub(crate) data: u8,
    pub(crate) mask: u8,
}

impl crate::common::RegisterValue for In {
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
pub struct InT;
unsafe impl crate::common::AsPtr for InT {}
impl crate::common::Reg<In> for InT {}

unsafe impl crate::common::Read<In> for InT {}
unsafe impl crate::common::Write<In> for InT {}
impl In {
    #[doc = "SELF element of FOO"]
    #[inline(always)]
    pub fn _self(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, r#in::_Self, r#in::_Self, In, common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,r#in::_Self,r#in::_Self,In,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<In> for InT {
    #[inline(always)]
    fn reset_value(&self) -> In {
        In::new(0)
    }
}
pub mod r#in {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct _Self(u8);

    impl _Self {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for _Self {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for _Self {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<_Self> for u64 {
        #[inline(always)]
        fn from(value: _Self) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for _Self {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl _Self {
        #[doc = "Input is on low level."]
        pub const _0_VALUE: Self = Self(0);

        #[doc = "Input is on high level."]
        pub const _1_VALUE: Self = Self(1);
    }
}
