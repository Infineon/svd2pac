/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.7.0 on Fri, 8 May 2026 12:51:25 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::{*};
#[allow(unused_imports)]
use crate::common::sealed;
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
pub const fn i2c2(self) -> crate::p33::I2C2{
    unsafe {   crate::p33::_I2C2::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(0usize)) }
}

}

 
#[doc = "Cluster that defines the base type"]
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
pub const fn reg1(&self) -> &'static crate::common::Reg<i2c2::Reg1_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<i2c2::Reg1_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
}
    
#[inline(always)]
pub const fn reg2(&self) -> &'static crate::common::Reg<i2c2::Reg2_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<i2c2::Reg2_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
}
    }

unsafe impl AsPtr for _I2C2 {
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
    use crate::common::{*};
    #[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Reg1_SPEC;
impl crate::sealed::RegSpec for Reg1_SPEC {
    type DataType = u32;
}

pub type  Reg1 = crate::RegValueT<Reg1_SPEC>;


impl NoBitfieldReg<Reg1_SPEC> for Reg1 {}
impl ::core::default::Default for Reg1 {
    #[inline(always)]
    fn default() -> Reg1 {
        <crate::RegValueT::<Reg1_SPEC> as RegisterValue<_>>::new(0)
    }
}

    #[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Reg2_SPEC;
impl crate::sealed::RegSpec for Reg2_SPEC {
    type DataType = u32;
}

pub type  Reg2 = crate::RegValueT<Reg2_SPEC>;


impl NoBitfieldReg<Reg2_SPEC> for Reg2 {}
impl ::core::default::Default for Reg2 {
    #[inline(always)]
    fn default() -> Reg2 {
        <crate::RegValueT::<Reg2_SPEC> as RegisterValue<_>>::new(0)
    }
}

    }







