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
pub const fn r#in(&self) -> &'static crate::common::Reg<self::In_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::In_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
}

}
#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct In_SPEC;
impl crate::sealed::RegSpec for In_SPEC {
    type DataType = u8;
}
 
#[doc = "FOO Input Register"]
pub type  In = crate::RegValueT<In_SPEC>;

impl In {
     
#[doc = "SELF element of FOO"]
    #[inline(always)]
    pub fn _self(self) -> crate::common::RegisterField<1,0x1,1,0,r#in::_Self,r#in::_Self,In_SPEC,crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,r#in::_Self,r#in::_Self,In_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for In {
    #[inline(always)]
    fn default() -> In {
        <crate::RegValueT::<In_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod r#in {
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct _Self_SPEC;
    pub type  _Self = crate::EnumBitfieldStruct<u8,_Self_SPEC>;
    impl _Self {
         
#[doc = "Input is on low level."]
        pub const _0_VALUE:Self =Self::new(0);
         
#[doc = "Input is on high level."]
        pub const _1_VALUE:Self =Self::new(1);
    }
}








