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
pub const fn register(&self) -> &'static crate::common::Reg<self::Register_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::Register_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
}

}
#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Register_SPEC;
impl crate::sealed::RegSpec for Register_SPEC {
    type DataType = u8;
}
 
#[doc = "\\[\\]\\\"😀\"\\n\\a\\r\'𒀀𒀽"]
pub type  Register = crate::RegValueT<Register_SPEC>;

impl Register {
     
#[doc = "\\[\\]\\\"😀\"\\n\\a\\r\'𒀀𒀽"]
    #[inline(always)]
    pub fn field(self) -> crate::common::RegisterField<1,0x1,1,0,register::Field,register::Field,Register_SPEC,crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,register::Field,register::Field,Register_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Register {
    #[inline(always)]
    fn default() -> Register {
        <crate::RegValueT::<Register_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod register {
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Field_SPEC;
    pub type  Field = crate::EnumBitfieldStruct<u8,Field_SPEC>;
    impl Field {
         
#[doc = "\\[\\]\\\"😀\"\\n\\a\\r\'𒀀𒀽"]
        pub const ENUM_VALUE:Self =Self::new(0);
    }
}








