/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.7.0 on Tue, 12 May 2026 17:17:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Description of peripheral"]
unsafe impl ::core::marker::Send for super::Timer {}
unsafe impl ::core::marker::Sync for super::Timer {}
impl super::Timer {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Register to test basic bitfield features"]
    #[inline(always)]
    pub const fn bitfield_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BitfieldReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BitfieldReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &'static crate::common::Reg<self::Sr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub const fn int(&self) -> &'static crate::common::Reg<self::Int_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Int_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "The Counter Register reflects the actual Value of the Timer/Counter"]
    #[inline(always)]
    pub const fn nobitfield_reg(
        &self,
    ) -> &'static crate::common::Reg<self::NobitfieldReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::NobitfieldReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "The Match Register stores the compare Value for the MATCH condition"]
    #[inline(always)]
    pub const fn r#match(
        &self,
    ) -> &'static crate::common::Reg<self::Match_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Match_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
    #[inline(always)]
    pub const fn prescale_rd(
        &self,
    ) -> &'static crate::common::Reg<self::PrescaleRd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::PrescaleRd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
    #[inline(always)]
    pub const fn prescale_wr(
        &self,
    ) -> &'static crate::common::Reg<self::PrescaleWr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::PrescaleWr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Array of register"]
    #[inline(always)]
    pub const fn arrayreg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Arrayreg_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x50usize))
        }
    }

    #[doc = "Another defintion register using alternate group"]
    #[inline(always)]
    pub const fn bitfield_reg_alt_group(
        &self,
    ) -> &'static crate::common::Reg<self::BitfieldRegAltGroup_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BitfieldRegAltGroup_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "64 bit register"]
    #[inline(always)]
    pub const fn register64bit(
        &self,
    ) -> &'static crate::common::Reg<self::Register64Bit_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Register64Bit_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Register with double underscore in name"]
    #[inline(always)]
    pub const fn register_with_double__underscore(
        &self,
    ) -> &'static crate::common::Reg<self::RegisterWithDoubleUnderscore_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::RegisterWithDoubleUnderscore_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(112usize))
        }
    }

    #[doc = "Register to test when peripheral has same name as register"]
    #[inline(always)]
    pub const fn timer(&self) -> &'static crate::common::Reg<self::Timer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8192usize),
            )
        }
    }

    #[doc = "Test Cluster"]
    #[inline(always)]
    pub const fn cluster1(self) -> crate::timer::Cluster1 {
        unsafe { crate::timer::_Cluster1::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(256usize)) }
    }

    #[doc = "Test Cluster array"]
    #[inline(always)]
    pub fn clusterdim(
        self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::timer::_ClusterDim, 4, 0x100> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitfieldReg_SPEC;
impl crate::sealed::RegSpec for BitfieldReg_SPEC {
    type DataType = u32;
}

#[doc = "Register to test basic bitfield features"]
pub type BitfieldReg = crate::RegValueT<BitfieldReg_SPEC>;

impl BitfieldReg {
    #[doc = "Boolean Bitfield Read Only"]
    #[inline(always)]
    pub fn boolr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BitfieldReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BitfieldReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Boolean Bitfield Write Only"]
    #[inline(always)]
    pub fn boolw(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BitfieldReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,BitfieldReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Boolean bitfield Read Write"]
    #[inline(always)]
    pub fn boolrw(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BitfieldReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BitfieldReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Raw Bitfield Read Only"]
    #[inline(always)]
    pub fn bitfieldr(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, BitfieldReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,BitfieldReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Bitfield Raw Write Only"]
    #[inline(always)]
    pub fn bitfieldw(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, BitfieldReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,BitfieldReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "BitField Raw Read Write"]
    #[inline(always)]
    pub fn bitfieldrw(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, BitfieldReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,BitfieldReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bitfield with enumerated field"]
    #[inline(always)]
    pub fn bitfieldenumerated(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        bitfield_reg::BitfieldEnumerated,
        bitfield_reg::BitfieldEnumerated,
        BitfieldReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            bitfield_reg::BitfieldEnumerated,
            bitfield_reg::BitfieldEnumerated,
            BitfieldReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Array of bitfields"]
    #[inline(always)]
    pub fn fieldarray(
        self,
        index: u8,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        8,
        2,
        bitfield_reg::FieldArray,
        bitfield_reg::FieldArray,
        BitfieldReg_SPEC,
        crate::common::RW,
    > {
        assert!(index < 8);
        crate::common::RegisterField::<
            16,
            0x3,
            8,
            2,
            bitfield_reg::FieldArray,
            bitfield_reg::FieldArray,
            BitfieldReg_SPEC,
            crate::common::RW,
        >::from_register(self, index)
    }
}
impl ::core::default::Default for BitfieldReg {
    #[inline(always)]
    fn default() -> BitfieldReg {
        <crate::RegValueT<BitfieldReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bitfield_reg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct BitfieldEnumerated_SPEC;
    pub type BitfieldEnumerated = crate::EnumBitfieldStruct<u8, BitfieldEnumerated_SPEC>;
    impl BitfieldEnumerated {
        #[doc = "Core Clock"]
        pub const C_CLK: Self = Self::new(0);

        #[doc = "GPIO A, PIN 0"]
        pub const GPIOA_0: Self = Self::new(1);

        #[doc = "GPIO A, PIN 1"]
        pub const GPIOA_1: Self = Self::new(2);

        #[doc = "GPIO A, PIN 2"]
        pub const GPIOA_2: Self = Self::new(3);

        #[doc = "GPIO A, PIN 3"]
        pub const GPIOA_3: Self = Self::new(4);

        #[doc = "GPIO A, PIN 4"]
        pub const GPIOA_4: Self = Self::new(5);

        #[doc = "GPIO A, PIN 5"]
        pub const GPIOA_5: Self = Self::new(6);

        #[doc = "GPIO A, PIN 6"]
        pub const GPIOA_6: Self = Self::new(7);

        #[doc = "GPIO A, PIN 7"]
        pub const GPIOA_7: Self = Self::new(8);

        #[doc = "GPIO B, PIN 0"]
        pub const GPIOB_0: Self = Self::new(9);

        #[doc = "GPIO B, PIN 1"]
        pub const GPIOB_1: Self = Self::new(10);

        #[doc = "GPIO B, PIN 2"]
        pub const GPIOB_2: Self = Self::new(11);

        #[doc = "GPIO B, PIN 3"]
        pub const GPIOB_3: Self = Self::new(12);

        #[doc = "GPIO C, PIN 0"]
        pub const GPIOC_0: Self = Self::new(13);

        #[doc = "GPIO C, PIN 1"]
        pub const GPIOC_5: Self = Self::new(14);

        #[doc = "GPIO C, PIN 2"]
        pub const GPIOC_6: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct FieldArray_SPEC;
    pub type FieldArray = crate::EnumBitfieldStruct<u8, FieldArray_SPEC>;
    impl FieldArray {
        #[doc = "Only rising edges result in a counter increment or decrement"]
        pub const RISING: Self = Self::new(0);

        #[doc = "Only falling edges result in a counter increment or decrement"]
        pub const FALLING: Self = Self::new(1);

        #[doc = "Rising and falling edges result in a counter increment or decrement"]
        pub const BOTH: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr_SPEC;
impl crate::sealed::RegSpec for Sr_SPEC {
    type DataType = u16;
}

#[doc = "Status Register"]
pub type Sr = crate::RegValueT<Sr_SPEC>;

impl Sr {
    #[doc = "Shows if Timer is running or not"]
    #[inline(always)]
    pub fn run(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sr::Run, sr::Run, Sr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,sr::Run,sr::Run,Sr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Shows if the MATCH was hit"]
    #[inline(always)]
    pub fn r#match(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sr::Match, sr::Match, Sr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,sr::Match,sr::Match,Sr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Shows if an underflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn un(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, sr::Un, sr::Un, Sr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,sr::Un,sr::Un,Sr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Shows if an overflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn ov(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, sr::Ov, sr::Ov, Sr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,sr::Ov,sr::Ov,Sr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Shows if Timer is in RESET state"]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, sr::Rst, sr::Rst, Sr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,sr::Rst,sr::Rst,Sr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Shows the currently active RELOAD Register"]
    #[inline(always)]
    pub fn reload(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        sr::Reload,
        sr::Reload,
        Sr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            sr::Reload,
            sr::Reload,
            Sr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        <crate::RegValueT<Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Run_SPEC;
    pub type Run = crate::EnumBitfieldStruct<u8, Run_SPEC>;
    impl Run {
        #[doc = "Timer is not running"]
        pub const STOPPED: Self = Self::new(0);

        #[doc = "Timer is running"]
        pub const RUNNING: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Match_SPEC;
    pub type Match = crate::EnumBitfieldStruct<u8, Match_SPEC>;
    impl Match {
        #[doc = "The MATCH condition was not hit"]
        pub const NO_MATCH: Self = Self::new(0);

        #[doc = "The MATCH condition was hit"]
        pub const MATCH_HIT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Un_SPEC;
    pub type Un = crate::EnumBitfieldStruct<u8, Un_SPEC>;
    impl Un {
        #[doc = "No underflow occured since last clear"]
        pub const NO_UNDERFLOW: Self = Self::new(0);

        #[doc = "A minimum of one underflow occured since last clear"]
        pub const UNDERFLOW: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ov_SPEC;
    pub type Ov = crate::EnumBitfieldStruct<u8, Ov_SPEC>;
    impl Ov {
        #[doc = "No overflow occured since last clear"]
        pub const NO_OVERFLOW: Self = Self::new(0);

        #[doc = "A minimum of one overflow occured since last clear"]
        pub const OVERFLOW_OCCURED: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "Timer is not in RESET state and can operate"]
        pub const READY: Self = Self::new(0);

        #[doc = "Timer is in RESET state and can not operate"]
        pub const IN_RESET: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reload_SPEC;
    pub type Reload = crate::EnumBitfieldStruct<u8, Reload_SPEC>;
    impl Reload {
        #[doc = "Reload Register number 0 is active"]
        pub const RELOAD_0: Self = Self::new(0);

        #[doc = "Reload Register number 1 is active"]
        pub const RELOAD_1: Self = Self::new(1);

        #[doc = "Reload Register number 2 is active"]
        pub const RELOAD_2: Self = Self::new(2);

        #[doc = "Reload Register number 3 is active"]
        pub const RELOAD_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int_SPEC;
impl crate::sealed::RegSpec for Int_SPEC {
    type DataType = u16;
}

#[doc = "Interrupt Register"]
pub type Int = crate::RegValueT<Int_SPEC>;

impl Int {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, int::En, int::En, Int_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,int::En,int::En,Int_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt Mode, selects on which condition the Timer should generate an Interrupt"]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, int::Mode, int::Mode, Int_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            int::Mode,
            int::Mode,
            Int_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Int {
    #[inline(always)]
    fn default() -> Int {
        <crate::RegValueT<Int_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod int {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Timer does not generate Interrupts"]
        pub const DISABLED: Self = Self::new(0);

        #[doc = "Timer triggers the TIMERn Interrupt"]
        pub const ENABLE: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "Timer generates an Interrupt when the MATCH condition is hit"]
        pub const MATCH: Self = Self::new(0);

        #[doc = "Timer generates an Interrupt when it underflows"]
        pub const UNDERFLOW: Self = Self::new(1);

        #[doc = "Timer generates an Interrupt when it overflows"]
        pub const OVERFLOW: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NobitfieldReg_SPEC;
impl crate::sealed::RegSpec for NobitfieldReg_SPEC {
    type DataType = u32;
}

#[doc = "The Counter Register reflects the actual Value of the Timer/Counter"]
pub type NobitfieldReg = crate::RegValueT<NobitfieldReg_SPEC>;

impl NoBitfieldReg<NobitfieldReg_SPEC> for NobitfieldReg {}
impl ::core::default::Default for NobitfieldReg {
    #[inline(always)]
    fn default() -> NobitfieldReg {
        <crate::RegValueT<NobitfieldReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match_SPEC;
impl crate::sealed::RegSpec for Match_SPEC {
    type DataType = u32;
}

#[doc = "The Match Register stores the compare Value for the MATCH condition"]
pub type Match = crate::RegValueT<Match_SPEC>;

impl NoBitfieldReg<Match_SPEC> for Match {}
impl ::core::default::Default for Match {
    #[inline(always)]
    fn default() -> Match {
        <crate::RegValueT<Match_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrescaleRd_SPEC;
impl crate::sealed::RegSpec for PrescaleRd_SPEC {
    type DataType = u32;
}

#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
pub type PrescaleRd = crate::RegValueT<PrescaleRd_SPEC>;

impl NoBitfieldReg<PrescaleRd_SPEC> for PrescaleRd {}
impl ::core::default::Default for PrescaleRd {
    #[inline(always)]
    fn default() -> PrescaleRd {
        <crate::RegValueT<PrescaleRd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrescaleWr_SPEC;
impl crate::sealed::RegSpec for PrescaleWr_SPEC {
    type DataType = u32;
}

#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
pub type PrescaleWr = crate::RegValueT<PrescaleWr_SPEC>;

impl NoBitfieldReg<PrescaleWr_SPEC> for PrescaleWr {}
impl ::core::default::Default for PrescaleWr {
    #[inline(always)]
    fn default() -> PrescaleWr {
        <crate::RegValueT<PrescaleWr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arrayreg_SPEC;
impl crate::sealed::RegSpec for Arrayreg_SPEC {
    type DataType = u32;
}

#[doc = "Array of register"]
pub type Arrayreg = crate::RegValueT<Arrayreg_SPEC>;

impl NoBitfieldReg<Arrayreg_SPEC> for Arrayreg {}
impl ::core::default::Default for Arrayreg {
    #[inline(always)]
    fn default() -> Arrayreg {
        <crate::RegValueT<Arrayreg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitfieldRegAltGroup_SPEC;
impl crate::sealed::RegSpec for BitfieldRegAltGroup_SPEC {
    type DataType = u32;
}

#[doc = "Another defintion register using alternate group"]
pub type BitfieldRegAltGroup = crate::RegValueT<BitfieldRegAltGroup_SPEC>;

impl NoBitfieldReg<BitfieldRegAltGroup_SPEC> for BitfieldRegAltGroup {}
impl ::core::default::Default for BitfieldRegAltGroup {
    #[inline(always)]
    fn default() -> BitfieldRegAltGroup {
        <crate::RegValueT<BitfieldRegAltGroup_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Register64Bit_SPEC;
impl crate::sealed::RegSpec for Register64Bit_SPEC {
    type DataType = u64;
}

#[doc = "64 bit register"]
pub type Register64Bit = crate::RegValueT<Register64Bit_SPEC>;

impl Register64Bit {
    #[inline(always)]
    pub fn boolean(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        register64bit::Boolean,
        register64bit::Boolean,
        Register64Bit_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            register64bit::Boolean,
            register64bit::Boolean,
            Register64Bit_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Register64Bit {
    #[inline(always)]
    fn default() -> Register64Bit {
        <crate::RegValueT<Register64Bit_SPEC> as RegisterValue<_>>::new(18446744073709551615)
    }
}
pub mod register64bit {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boolean_SPEC;
    pub type Boolean = crate::EnumBitfieldStruct<u8, Boolean_SPEC>;
    impl Boolean {
        pub const TRUE: Self = Self::new(1);

        pub const FALSE: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegisterWithDoubleUnderscore_SPEC;
impl crate::sealed::RegSpec for RegisterWithDoubleUnderscore_SPEC {
    type DataType = u32;
}

#[doc = "Register with double underscore in name"]
pub type RegisterWithDoubleUnderscore = crate::RegValueT<RegisterWithDoubleUnderscore_SPEC>;

impl RegisterWithDoubleUnderscore {
    #[doc = "Field with double underscore in name"]
    #[inline(always)]
    pub fn field_with_double__underscore(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RegisterWithDoubleUnderscore_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RegisterWithDoubleUnderscore_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RegisterWithDoubleUnderscore {
    #[inline(always)]
    fn default() -> RegisterWithDoubleUnderscore {
        <crate::RegValueT<RegisterWithDoubleUnderscore_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer_SPEC;
impl crate::sealed::RegSpec for Timer_SPEC {
    type DataType = u32;
}

#[doc = "Register to test when peripheral has same name as register"]
pub type Timer = crate::RegValueT<Timer_SPEC>;

impl NoBitfieldReg<Timer_SPEC> for Timer {}
impl ::core::default::Default for Timer {
    #[inline(always)]
    fn default() -> Timer {
        <crate::RegValueT<Timer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "Test Cluster"]
#[non_exhaustive]
pub struct _Cluster1;

#[doc = "Test Cluster"]
pub type Cluster1 = &'static _Cluster1;

unsafe impl ::core::marker::Sync for _Cluster1 {}
impl _Cluster1 {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[inline(always)]
    pub const fn cr(&self) -> &'static crate::common::Reg<cluster1::Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<cluster1::Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "A cluster inside another cluster"]
    #[inline(always)]
    pub const fn cluster1(self) -> crate::timer::cluster1::Cluster1 {
        unsafe {
            crate::timer::cluster1::_Cluster1::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "HSSL"]
    #[inline(always)]
    pub const fn hssl(self) -> crate::timer::cluster1::HsslHssl {
        unsafe {
            crate::timer::cluster1::_HsslHssl::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }
}

unsafe impl AsPtr for _Cluster1 {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod cluster1 {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr_SPEC;
    impl crate::sealed::RegSpec for Cr_SPEC {
        type DataType = u32;
    }

    pub type Cr = crate::RegValueT<Cr_SPEC>;

    impl Cr {
        #[inline(always)]
        pub fn filed1(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Cr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8,u8,Cr_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[inline(always)]
        pub fn psc(
            self,
        ) -> crate::common::RegisterField<3, 0x3, 1, 0, cr::Psc, cr::Psc, Cr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x3,1,0,cr::Psc,cr::Psc,Cr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            <crate::RegValueT<Cr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod cr {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Psc_SPEC;
        pub type Psc = crate::EnumBitfieldStruct<u8, Psc_SPEC>;
        impl Psc {
            pub const VAL_1: Self = Self::new(1);
        }
    }

    #[doc = "A cluster inside another cluster"]
    #[non_exhaustive]
    pub struct _Cluster1;

    #[doc = "A cluster inside another cluster"]
    pub type Cluster1 = &'static _Cluster1;

    unsafe impl ::core::marker::Sync for _Cluster1 {}
    impl _Cluster1 {
        #[inline(always)]
        pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
            &*(ptr as *const _)
        }

        #[inline(always)]
        pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
            self as *const Self as *mut u8
        }

        #[inline(always)]
        pub const fn nestedreg(
            &self,
        ) -> &'static crate::common::Reg<cluster1::NestedReg_SPEC, crate::common::RW> {
            unsafe {
                crate::common::Reg::<cluster1::NestedReg_SPEC, crate::common::RW>::from_ptr(
                    self._svd2pac_as_ptr().add(0usize),
                )
            }
        }
    }

    unsafe impl AsPtr for _Cluster1 {
        fn as_ptr(&self) -> *mut u8 {
            self._svd2pac_as_ptr()
        }

        #[inline(always)]
        unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
            Self::_svd2pac_from_ptr(ptr)
        }
    }

    pub mod cluster1 {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NestedReg_SPEC;
        impl crate::sealed::RegSpec for NestedReg_SPEC {
            type DataType = u32;
        }

        pub type NestedReg = crate::RegValueT<NestedReg_SPEC>;

        impl NoBitfieldReg<NestedReg_SPEC> for NestedReg {}
        impl ::core::default::Default for NestedReg {
            #[inline(always)]
            fn default() -> NestedReg {
                <crate::RegValueT<NestedReg_SPEC> as RegisterValue<_>>::new(74565)
            }
        }
    }

    #[doc = "HSSL"]
    #[non_exhaustive]
    pub struct _HsslHssl;

    #[doc = "HSSL"]
    pub type HsslHssl = &'static _HsslHssl;

    unsafe impl ::core::marker::Sync for _HsslHssl {}
    impl _HsslHssl {
        #[inline(always)]
        pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
            &*(ptr as *const _)
        }

        #[inline(always)]
        pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
            self as *const Self as *mut u8
        }

        #[doc = "CH"]
        #[inline(always)]
        pub fn ch(
            self,
        ) -> &'static crate::common::ClusterRegisterArray<
            crate::timer::cluster1::hssl_hssl::_HsslHsslCh,
            2,
            0x4,
        > {
            unsafe {
                crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
            }
        }
    }

    unsafe impl AsPtr for _HsslHssl {
        fn as_ptr(&self) -> *mut u8 {
            self._svd2pac_as_ptr()
        }

        #[inline(always)]
        unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
            Self::_svd2pac_from_ptr(ptr)
        }
    }

    pub mod hssl_hssl {
        #[allow(unused_imports)]
        use crate::common::*;

        #[doc = "CH"]
        #[non_exhaustive]
        pub struct _HsslHsslCh;

        #[doc = "CH"]
        pub type HsslHsslCh = &'static _HsslHsslCh;

        unsafe impl ::core::marker::Sync for _HsslHsslCh {}
        impl _HsslHsslCh {
            #[inline(always)]
            pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
                &*(ptr as *const _)
            }

            #[inline(always)]
            pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
                self as *const Self as *mut u8
            }

            #[doc = "HSSL0 Channel 0 OK Service Request\n resetvalue={Application Reset:0x0}"]
            #[inline(always)]
            pub const fn hsslxcoky(
                &self,
            ) -> &'static crate::common::Reg<hssl_hssl_ch::HssLxCoKy_SPEC, crate::common::RW>
            {
                unsafe {
                    crate::common::Reg::<hssl_hssl_ch::HssLxCoKy_SPEC, crate::common::RW>::from_ptr(
                        self._svd2pac_as_ptr().add(0usize),
                    )
                }
            }
        }

        unsafe impl AsPtr for _HsslHsslCh {
            fn as_ptr(&self) -> *mut u8 {
                self._svd2pac_as_ptr()
            }

            #[inline(always)]
            unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
                Self::_svd2pac_from_ptr(ptr)
            }
        }

        pub mod hssl_hssl_ch {
            #[allow(unused_imports)]
            use crate::common::*;
            #[doc(hidden)]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct HssLxCoKy_SPEC;
            impl crate::sealed::RegSpec for HssLxCoKy_SPEC {
                type DataType = u32;
            }

            #[doc = "HSSL0 Channel 0 OK Service Request\n resetvalue={Application Reset:0x0}"]
            pub type HssLxCoKy = crate::RegValueT<HssLxCoKy_SPEC>;

            impl NoBitfieldReg<HssLxCoKy_SPEC> for HssLxCoKy {}
            impl ::core::default::Default for HssLxCoKy {
                #[inline(always)]
                fn default() -> HssLxCoKy {
                    <crate::RegValueT<HssLxCoKy_SPEC> as RegisterValue<_>>::new(0)
                }
            }
        }
    }
}

#[doc = "Test Cluster array"]
#[non_exhaustive]
pub struct _ClusterDim;

#[doc = "Test Cluster array"]
pub type ClusterDim = &'static _ClusterDim;

unsafe impl ::core::marker::Sync for _ClusterDim {}
impl _ClusterDim {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[inline(always)]
    pub const fn cr(&self) -> &'static crate::common::Reg<clusterdim::Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<clusterdim::Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}

unsafe impl AsPtr for _ClusterDim {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod clusterdim {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr_SPEC;
    impl crate::sealed::RegSpec for Cr_SPEC {
        type DataType = u32;
    }

    pub type Cr = crate::RegValueT<Cr_SPEC>;

    impl Cr {
        #[inline(always)]
        pub fn field1(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Cr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8,u8,Cr_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[inline(always)]
        pub fn psc(
            self,
        ) -> crate::common::RegisterField<3, 0x3, 1, 0, cr::Psc, cr::Psc, Cr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x3,1,0,cr::Psc,cr::Psc,Cr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            <crate::RegValueT<Cr_SPEC> as RegisterValue<_>>::new(4096)
        }
    }
    pub mod cr {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Psc_SPEC;
        pub type Psc = crate::EnumBitfieldStruct<u8, Psc_SPEC>;
        impl Psc {
            pub const VAL_1: Self = Self::new(1);
        }
    }
}
