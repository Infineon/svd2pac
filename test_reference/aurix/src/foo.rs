/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Fri, 14 Aug 2026 13:58:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common;
#[allow(unused_imports)]
use crate::common::{
    AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
    ResetValue as _, Write as _,
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
    unsafe {   self::InT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
}

}
 
#[doc = "FOO Input Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In {
    pub(crate) data: u8,
    pub(crate) mask: u8
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
        Self {
            data,
            mask: 0x0,
        }
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
#[doc = "0 = 0_VALUE: Input is on low level."]
#[doc = "1 = 1_VALUE: Input is on high level."]
    #[inline(always)]
    pub fn _self(self) -> 
    crate::common::RegisterFieldBool<1,1,0,In,common::RW> {
        
    crate::common::RegisterFieldBool::<1,1,0,In,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<In> for InT {
    #[inline(always)]
    fn reset_value(&self) -> In {
        In::new(0)
    }
}









