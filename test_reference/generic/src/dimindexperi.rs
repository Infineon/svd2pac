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
#[doc = r"Test Dim Index"]
unsafe impl ::core::marker::Send for super::DimIndexPeri {}
unsafe impl ::core::marker::Sync for super::DimIndexPeri {}
impl super::DimIndexPeri {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Array cluster index"]
    #[inline(always)]
    pub fn clust(
        self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::dimindexperi::_Clust, 3, 0x30> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub fn clu3st(self) -> crate::dimindexperi::Clust {
        unsafe {
            crate::dimindexperi::_Clust::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub fn clu4st(self) -> crate::dimindexperi::Clust {
        unsafe {
            crate::dimindexperi::_Clust::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(0x30usize))
        }
    }
    #[inline(always)]
    pub fn clu5st(self) -> crate::dimindexperi::Clust {
        unsafe {
            crate::dimindexperi::_Clust::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(0x60usize))
        }
    }
}

#[doc(hidden)]
#[non_exhaustive]
pub struct _Clust;

#[doc = "Array cluster index"]
pub type Clust = &'static _Clust;

unsafe impl ::core::marker::Sync for _Clust {}
impl _Clust {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[inline(always)]
    pub const fn areg(&self) -> &'static crate::common::ClusterRegisterArray<clust::ARegT, 3, 0x4> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub fn aregd(&self) -> &'static clust::ARegT {
        unsafe { clust::ARegT::from_ptr(self._svd2pac_as_ptr().add(0x0usize)) }
    }
    #[inline(always)]
    pub fn aregf(&self) -> &'static clust::ARegT {
        unsafe { clust::ARegT::from_ptr(self._svd2pac_as_ptr().add(0x4usize)) }
    }
    #[inline(always)]
    pub fn aregg(&self) -> &'static clust::ARegT {
        unsafe { clust::ARegT::from_ptr(self._svd2pac_as_ptr().add(0x8usize)) }
    }

    #[inline(always)]
    pub const fn breg(&self) -> &'static crate::common::ClusterRegisterArray<clust::BRegT, 3, 0x4> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xcusize))
        }
    }
    #[inline(always)]
    pub fn breg2(&self) -> &'static clust::BRegT {
        unsafe { clust::BRegT::from_ptr(self._svd2pac_as_ptr().add(0xcusize)) }
    }
    #[inline(always)]
    pub fn breg3(&self) -> &'static clust::BRegT {
        unsafe { clust::BRegT::from_ptr(self._svd2pac_as_ptr().add(0x10usize)) }
    }
    #[inline(always)]
    pub fn breg4(&self) -> &'static clust::BRegT {
        unsafe { clust::BRegT::from_ptr(self._svd2pac_as_ptr().add(0x14usize)) }
    }

    #[inline(always)]
    pub const fn creg(&self) -> &'static crate::common::ClusterRegisterArray<clust::CRegT, 3, 0x4> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18usize))
        }
    }
    #[inline(always)]
    pub fn cregc(&self) -> &'static clust::CRegT {
        unsafe { clust::CRegT::from_ptr(self._svd2pac_as_ptr().add(0x18usize)) }
    }
    #[inline(always)]
    pub fn cregd(&self) -> &'static clust::CRegT {
        unsafe { clust::CRegT::from_ptr(self._svd2pac_as_ptr().add(0x1cusize)) }
    }
    #[inline(always)]
    pub fn crege(&self) -> &'static clust::CRegT {
        unsafe { clust::CRegT::from_ptr(self._svd2pac_as_ptr().add(0x20usize)) }
    }
}

unsafe impl crate::common::AsPtr for _Clust {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod clust {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AReg {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for AReg {
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
    pub struct ARegT;
    unsafe impl crate::common::AsPtr for ARegT {}
    impl crate::common::Reg<AReg> for ARegT {}

    unsafe impl crate::common::Read<AReg> for ARegT {}
    unsafe impl crate::common::Write<AReg> for ARegT {}
    impl AReg {
        #[inline(always)]
        pub fn arraybitfield_(
            self,
            index: u8,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg, common::RW> {
            assert!(index < 3);
            crate::common::RegisterField::<1, 0x3, 3, 2, u8, u8, AReg, common::RW>::from_register(
                self, index,
            )
        }
        #[inline(always)]
        pub fn arraybitfield_c(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg, common::RW> {
            crate::common::RegisterField::<1, 0x3, 3, 2, u8, u8, AReg, common::RW>::from_register(
                self, 0,
            )
        }

        #[inline(always)]
        pub fn arraybitfield_d(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg, common::RW> {
            crate::common::RegisterField::<1, 0x3, 3, 2, u8, u8, AReg, common::RW>::from_register(
                self, 1,
            )
        }

        #[inline(always)]
        pub fn arraybitfield_e(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg, common::RW> {
            crate::common::RegisterField::<1, 0x3, 3, 2, u8, u8, AReg, common::RW>::from_register(
                self, 2,
            )
        }
    }
    impl crate::common::ResetValue<AReg> for ARegT {
        #[inline(always)]
        fn reset_value(&self) -> AReg {
            AReg::new(0)
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BReg {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for BReg {
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
    pub struct BRegT;
    unsafe impl crate::common::AsPtr for BRegT {}
    impl crate::common::Reg<BReg> for BRegT {}

    unsafe impl crate::common::Read<BReg> for BRegT {}
    unsafe impl crate::common::Write<BReg> for BRegT {}
    impl BReg {
        #[inline(always)]
        pub fn arraybitfieldbool_(
            self,
            index: u8,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg, common::RW> {
            assert!(index < 3);

            crate::common::RegisterFieldBool::<0, 3, 1, BReg, common::RW>::from_register(
                self, index,
            )
        }
        #[inline(always)]
        pub fn arraybitfieldbool_c(
            self,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg, common::RW> {
            crate::common::RegisterFieldBool::<0, 3, 1, BReg, common::RW>::from_register(self, 0)
        }

        #[inline(always)]
        pub fn arraybitfieldbool_d(
            self,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg, common::RW> {
            crate::common::RegisterFieldBool::<0, 3, 1, BReg, common::RW>::from_register(self, 1)
        }

        #[inline(always)]
        pub fn arraybitfieldbool_e(
            self,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg, common::RW> {
            crate::common::RegisterFieldBool::<0, 3, 1, BReg, common::RW>::from_register(self, 2)
        }
    }
    impl crate::common::ResetValue<BReg> for BRegT {
        #[inline(always)]
        fn reset_value(&self) -> BReg {
            BReg::new(0)
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CReg {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for CReg {
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
    pub struct CRegT;
    unsafe impl crate::common::AsPtr for CRegT {}
    impl crate::common::Reg<CReg> for CRegT {}

    unsafe impl crate::common::Read<CReg> for CRegT {}
    unsafe impl crate::common::Write<CReg> for CRegT {}

    impl crate::common::NoBitfieldReg for CReg {}
    impl crate::common::ResetValue<CReg> for CRegT {
        #[inline(always)]
        fn reset_value(&self) -> CReg {
            CReg::new(0)
        }
    }
}
