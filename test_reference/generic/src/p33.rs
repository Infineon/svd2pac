/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 14:25:24 +0000

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
#[doc = r"Port naming peripheral struct and peripheral const are the same"]
unsafe impl ::core::marker::Send for super::P33 {}
unsafe impl ::core::marker::Sync for super::P33 {}
impl super::P33 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Cluster that defines the base type"]
    #[inline(always)]
    pub const fn i2c2(self) -> crate::p33::I2C2 {
        unsafe { crate::p33::_I2C2::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }
}

#[doc(hidden)]
#[non_exhaustive]
pub struct _I2C2;

#[doc = "Cluster that defines the base type"]
pub type I2C2 = &'static _I2C2;

unsafe impl ::core::marker::Sync for _I2C2 {}
impl _I2C2 {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[inline(always)]
    pub fn reg1(&self) -> &'static i2c2::Reg1T {
        unsafe { i2c2::Reg1T::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[inline(always)]
    pub fn reg2(&self) -> &'static i2c2::Reg2T {
        unsafe { i2c2::Reg2T::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }
}

unsafe impl crate::common::AsPtr for _I2C2 {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod i2c2 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reg1 {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Reg1 {
        type DataType = u32;

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
    pub struct Reg1T;
    unsafe impl crate::common::AsPtr for Reg1T {}
    impl crate::common::Reg<Reg1> for Reg1T {}

    unsafe impl crate::common::Read<Reg1> for Reg1T {}
    unsafe impl crate::common::Write<Reg1> for Reg1T {}

    impl crate::common::NoBitfieldReg for Reg1 {}
    impl crate::common::ResetValue<Reg1> for Reg1T {
        #[inline(always)]
        fn reset_value(&self) -> Reg1 {
            Reg1::new(0)
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reg2 {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Reg2 {
        type DataType = u32;

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
    pub struct Reg2T;
    unsafe impl crate::common::AsPtr for Reg2T {}
    impl crate::common::Reg<Reg2> for Reg2T {}

    unsafe impl crate::common::Read<Reg2> for Reg2T {}
    unsafe impl crate::common::Write<Reg2> for Reg2T {}

    impl crate::common::NoBitfieldReg for Reg2 {}
    impl crate::common::ResetValue<Reg2> for Reg2T {
        #[inline(always)]
        fn reset_value(&self) -> Reg2 {
            Reg2::new(0)
        }
    }
}
