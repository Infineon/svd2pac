/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.7.0 on Fri, 8 May 2026 12:39:15 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
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
    pub const fn baseregister(
        &self,
    ) -> &'static crate::common::Reg<self::BaseRegister_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BaseRegister_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

    #[inline(always)]
    pub const fn derivedregister(
        &self,
    ) -> &'static crate::common::Reg<self::DerivedRegister_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DerivedRegister_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4098usize),
            )
        }
    }

    #[inline(always)]
    pub const fn derivedfromfaraway(
        &self,
    ) -> &'static crate::common::Reg<self::DerivedFromFarAway_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DerivedFromFarAway_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4100usize),
            )
        }
    }

    #[doc = "Cluster that defines the base type"]
    #[inline(always)]
    pub const fn basecluster(self) -> crate::derivedtest::BaseClusterType {
        unsafe {
            crate::derivedtest::_BaseClusterType::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "ArrayCluster"]
    #[inline(always)]
    pub fn arraycluster(
        self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::derivedtest::_BaseClusterType, 4, 0x8>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8usize))
        }
    }

    #[inline(always)]
    pub const fn derivedderivedcluster(self) -> crate::derivedtest::BaseClusterType {
        unsafe {
            crate::derivedtest::_BaseClusterType::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn derivedfromfarawaycluster(self) -> crate::timer::cluster1::Cluster1 {
        unsafe {
            crate::timer::cluster1::_Cluster1::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BaseRegister_SPEC;
impl crate::sealed::RegSpec for BaseRegister_SPEC {
    type DataType = u16;
}

#[doc = "BaseRegister"]
pub type BaseRegister = crate::RegValueT<BaseRegister_SPEC>;

impl BaseRegister {
    #[doc = "Shows if Timer is running or not"]
    #[inline(always)]
    pub fn run(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        baseregister::Run,
        baseregister::Run,
        BaseRegister_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            baseregister::Run,
            baseregister::Run,
            BaseRegister_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BaseRegister {
    #[inline(always)]
    fn default() -> BaseRegister {
        <crate::RegValueT<BaseRegister_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod baseregister {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Run_SPEC;
    pub type Run = crate::EnumBitfieldStruct<u8, Run_SPEC>;
    impl Run {
        #[doc = "Timer is not running"]
        pub const STOPPED: Self = Self::new(0);

        #[doc = "Timer is running"]
        pub const RUNNING: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DerivedRegister_SPEC;
impl crate::sealed::RegSpec for DerivedRegister_SPEC {
    type DataType = u16;
}

pub type DerivedRegister = crate::RegValueT<DerivedRegister_SPEC>;

impl NoBitfieldReg<DerivedRegister_SPEC> for DerivedRegister {}
impl ::core::default::Default for DerivedRegister {
    #[inline(always)]
    fn default() -> DerivedRegister {
        <crate::RegValueT<DerivedRegister_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DerivedFromFarAway_SPEC;
impl crate::sealed::RegSpec for DerivedFromFarAway_SPEC {
    type DataType = u32;
}

pub type DerivedFromFarAway = crate::RegValueT<DerivedFromFarAway_SPEC>;

impl NoBitfieldReg<DerivedFromFarAway_SPEC> for DerivedFromFarAway {}
impl ::core::default::Default for DerivedFromFarAway {
    #[inline(always)]
    fn default() -> DerivedFromFarAway {
        <crate::RegValueT<DerivedFromFarAway_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "Cluster that defines the base type"]
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
    pub const fn reg1(
        &self,
    ) -> &'static crate::common::Reg<baseclustertype::Reg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<baseclustertype::Reg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reg2(
        &self,
    ) -> &'static crate::common::Reg<baseclustertype::Reg2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<baseclustertype::Reg2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}

unsafe impl AsPtr for _BaseClusterType {
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
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reg1_SPEC;
    impl crate::sealed::RegSpec for Reg1_SPEC {
        type DataType = u32;
    }

    pub type Reg1 = crate::RegValueT<Reg1_SPEC>;

    impl NoBitfieldReg<Reg1_SPEC> for Reg1 {}
    impl ::core::default::Default for Reg1 {
        #[inline(always)]
        fn default() -> Reg1 {
            <crate::RegValueT<Reg1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reg2_SPEC;
    impl crate::sealed::RegSpec for Reg2_SPEC {
        type DataType = u32;
    }

    pub type Reg2 = crate::RegValueT<Reg2_SPEC>;

    impl NoBitfieldReg<Reg2_SPEC> for Reg2 {}
    impl ::core::default::Default for Reg2 {
        #[inline(always)]
        fn default() -> Reg2 {
            <crate::RegValueT<Reg2_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
