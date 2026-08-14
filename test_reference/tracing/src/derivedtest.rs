/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Fri, 14 Aug 2026 14:16:57 +0000

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
#[doc = r"Fake Peripheral to test cluster and register with derivedFrom attribute"]
unsafe impl ::core::marker::Send for super::DerivedTest {}
unsafe impl ::core::marker::Sync for super::DerivedTest {}
impl super::DerivedTest {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "BaseRegister"]
    #[inline(always)]
    pub fn baseregister(&self) -> &'static self::BaseRegisterT {
        unsafe { self::BaseRegisterT::from_ptr(self._svd2pac_as_ptr().add(4096usize)) }
    }

    #[inline(always)]
    pub fn derivedregister(&self) -> &'static self::DerivedRegisterT {
        unsafe { self::DerivedRegisterT::from_ptr(self._svd2pac_as_ptr().add(4098usize)) }
    }

    #[inline(always)]
    pub fn derivedfromfaraway(&self) -> &'static self::DerivedFromFarAwayT {
        unsafe { self::DerivedFromFarAwayT::from_ptr(self._svd2pac_as_ptr().add(4100usize)) }
    }

    #[doc = "Cluster that defines the base type"]
    #[inline(always)]
    pub const fn basecluster(&self) -> crate::derivedtest::BaseClusterType {
        unsafe {
            crate::derivedtest::_BaseClusterType::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "ArrayCluster"]
    #[inline(always)]
    pub fn arraycluster(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::derivedtest::_BaseClusterType, 4, 0x8>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8usize))
        }
    }

    #[inline(always)]
    pub const fn derivedderivedcluster(&self) -> crate::derivedtest::BaseClusterType {
        unsafe {
            crate::derivedtest::_BaseClusterType::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn derivedfromfarawaycluster(&self) -> crate::timer::cluster1::Cluster1 {
        unsafe {
            crate::timer::cluster1::_Cluster1::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }
}

#[doc = "BaseRegister"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BaseRegister {
    pub(crate) data: u16,
    pub(crate) mask: u16,
}

impl crate::common::RegisterValue for BaseRegister {
    type DataType = u16;

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
pub struct BaseRegisterT;
unsafe impl crate::common::AsPtr for BaseRegisterT {}
impl crate::common::Reg<BaseRegister> for BaseRegisterT {}

unsafe impl crate::common::Read<BaseRegister> for BaseRegisterT {}
unsafe impl crate::common::Write<BaseRegister> for BaseRegisterT {}
impl BaseRegister {
    #[doc = "Shows if Timer is running or not"]
    #[doc = "0 = Stopped: Timer is not running"]
    #[doc = "1 = Running: Timer is running"]
    #[inline(always)]
    pub fn run(self) -> crate::common::RegisterFieldBool<0, 1, 0, BaseRegister, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, BaseRegister, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<BaseRegister> for BaseRegisterT {
    #[inline(always)]
    fn reset_value(&self) -> BaseRegister {
        BaseRegister::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DerivedRegister {
    pub(crate) data: u16,
    pub(crate) mask: u16,
}

impl crate::common::RegisterValue for DerivedRegister {
    type DataType = u16;

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
pub struct DerivedRegisterT;
unsafe impl crate::common::AsPtr for DerivedRegisterT {}
impl crate::common::Reg<DerivedRegister> for DerivedRegisterT {}

unsafe impl crate::common::Read<DerivedRegister> for DerivedRegisterT {}
unsafe impl crate::common::Write<DerivedRegister> for DerivedRegisterT {}

impl crate::common::NoBitfieldReg for DerivedRegister {}
impl crate::common::ResetValue<DerivedRegister> for DerivedRegisterT {
    #[inline(always)]
    fn reset_value(&self) -> DerivedRegister {
        DerivedRegister::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DerivedFromFarAway {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DerivedFromFarAway {
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
pub struct DerivedFromFarAwayT;
unsafe impl crate::common::AsPtr for DerivedFromFarAwayT {}
impl crate::common::Reg<DerivedFromFarAway> for DerivedFromFarAwayT {}

unsafe impl crate::common::Read<DerivedFromFarAway> for DerivedFromFarAwayT {}
unsafe impl crate::common::Write<DerivedFromFarAway> for DerivedFromFarAwayT {}

impl crate::common::NoBitfieldReg for DerivedFromFarAway {}
impl crate::common::ResetValue<DerivedFromFarAway> for DerivedFromFarAwayT {
    #[inline(always)]
    fn reset_value(&self) -> DerivedFromFarAway {
        DerivedFromFarAway::new(0)
    }
}

#[doc(hidden)]
#[non_exhaustive]
pub struct _BaseClusterType;

#[doc = "Cluster that defines the base type"]
pub type BaseClusterType = &'static _BaseClusterType;

unsafe impl ::core::marker::Sync for _BaseClusterType {}
impl _BaseClusterType {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[inline(always)]
    pub fn reg1(&self) -> &'static baseclustertype::Reg1T {
        unsafe { baseclustertype::Reg1T::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[inline(always)]
    pub fn reg2(&self) -> &'static baseclustertype::Reg2T {
        unsafe { baseclustertype::Reg2T::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }
}

unsafe impl crate::common::AsPtr for _BaseClusterType {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod baseclustertype {
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
