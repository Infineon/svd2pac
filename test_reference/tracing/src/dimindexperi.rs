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

#[doc = "Array cluster index"]
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
    pub const fn areg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<clust::AReg_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }
    #[inline(always)]
    pub const fn aregd(&self) -> &'static crate::common::Reg<clust::AReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::AReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aregf(&self) -> &'static crate::common::Reg<clust::AReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::AReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn aregg(&self) -> &'static crate::common::Reg<clust::AReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::AReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn breg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<clust::BReg_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xcusize))
        }
    }
    #[inline(always)]
    pub const fn breg2(&self) -> &'static crate::common::Reg<clust::BReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::BReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn breg3(&self) -> &'static crate::common::Reg<clust::BReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::BReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x10usize),
            )
        }
    }
    #[inline(always)]
    pub const fn breg4(&self) -> &'static crate::common::Reg<clust::BReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::BReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn creg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<clust::CReg_SPEC, crate::common::RW>,
        3,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x18usize))
        }
    }
    #[inline(always)]
    pub const fn cregc(&self) -> &'static crate::common::Reg<clust::CReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::CReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x18usize),
            )
        }
    }
    #[inline(always)]
    pub const fn cregd(&self) -> &'static crate::common::Reg<clust::CReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::CReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn crege(&self) -> &'static crate::common::Reg<clust::CReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clust::CReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x20usize),
            )
        }
    }
}

unsafe impl AsPtr for _Clust {
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
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AReg_SPEC;
    impl crate::sealed::RegSpec for AReg_SPEC {
        type DataType = u32;
    }

    pub type AReg = crate::RegValueT<AReg_SPEC>;

    impl AReg {
        #[inline(always)]
        pub fn arraybitfield_(
            self,
            index: u8,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg_SPEC, crate::common::RW>
        {
            assert!(index < 3);
            crate::common::RegisterField::<1,0x3,3,2,u8,u8,AReg_SPEC,crate::common::RW>::from_register(self,index)
        }
        #[inline(always)]
        pub fn arraybitfield_c(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x3,3,2,u8,u8,AReg_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[inline(always)]
        pub fn arraybitfield_d(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x3,3,2,u8,u8,AReg_SPEC,crate::common::RW>::from_register(self,1)
        }

        #[inline(always)]
        pub fn arraybitfield_e(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 3, 2, u8, u8, AReg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x3,3,2,u8,u8,AReg_SPEC,crate::common::RW>::from_register(self,2)
        }
    }
    impl ::core::default::Default for AReg {
        #[inline(always)]
        fn default() -> AReg {
            <crate::RegValueT<AReg_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BReg_SPEC;
    impl crate::sealed::RegSpec for BReg_SPEC {
        type DataType = u32;
    }

    pub type BReg = crate::RegValueT<BReg_SPEC>;

    impl BReg {
        #[inline(always)]
        pub fn arraybitfieldbool_(
            self,
            index: u8,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg_SPEC, crate::common::RW> {
            assert!(index < 3);

            crate::common::RegisterFieldBool::<0, 3, 1, BReg_SPEC, crate::common::RW>::from_register(
                self, index,
            )
        }
        #[inline(always)]
        pub fn arraybitfieldbool_c(
            self,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 3, 1, BReg_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }

        #[inline(always)]
        pub fn arraybitfieldbool_d(
            self,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 3, 1, BReg_SPEC, crate::common::RW>::from_register(
                self, 1,
            )
        }

        #[inline(always)]
        pub fn arraybitfieldbool_e(
            self,
        ) -> crate::common::RegisterFieldBool<0, 3, 1, BReg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 3, 1, BReg_SPEC, crate::common::RW>::from_register(
                self, 2,
            )
        }
    }
    impl ::core::default::Default for BReg {
        #[inline(always)]
        fn default() -> BReg {
            <crate::RegValueT<BReg_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CReg_SPEC;
    impl crate::sealed::RegSpec for CReg_SPEC {
        type DataType = u32;
    }

    pub type CReg = crate::RegValueT<CReg_SPEC>;

    impl NoBitfieldReg<CReg_SPEC> for CReg {}
    impl ::core::default::Default for CReg {
        #[inline(always)]
        fn default() -> CReg {
            <crate::RegValueT<CReg_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
