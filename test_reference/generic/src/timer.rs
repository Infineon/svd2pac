/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Fri, 14 Aug 2026 14:11:04 +0000

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
    pub fn bitfield_reg(&self) -> &'static self::BitfieldRegT {
        unsafe { self::BitfieldRegT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Status Register"]
    #[inline(always)]
    pub fn sr(&self) -> &'static self::SrT {
        unsafe { self::SrT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub fn int(&self) -> &'static self::IntT {
        unsafe { self::IntT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "The Counter Register reflects the actual Value of the Timer/Counter"]
    #[inline(always)]
    pub fn nobitfield_reg(&self) -> &'static self::NobitfieldRegT {
        unsafe { self::NobitfieldRegT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "The Match Register stores the compare Value for the MATCH condition"]
    #[inline(always)]
    pub fn r#match(&self) -> &'static self::MatchT {
        unsafe { self::MatchT::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }

    #[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
    #[inline(always)]
    pub fn prescale_rd(&self) -> &'static self::PrescaleRdT {
        unsafe { self::PrescaleRdT::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
    #[inline(always)]
    pub fn prescale_wr(&self) -> &'static self::PrescaleWrT {
        unsafe { self::PrescaleWrT::from_ptr(self._svd2pac_as_ptr().add(44usize)) }
    }

    #[doc = "Array of register"]
    #[inline(always)]
    pub const fn arrayreg(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<self::ArrayregT, 4, 0x4> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x50usize))
        }
    }

    #[doc = "Another defintion register using alternate group"]
    #[inline(always)]
    pub fn bitfield_reg_alt_group(&self) -> &'static self::BitfieldRegAltGroupT {
        unsafe { self::BitfieldRegAltGroupT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "64 bit register"]
    #[inline(always)]
    pub fn register64bit(&self) -> &'static self::Register64BitT {
        unsafe { self::Register64BitT::from_ptr(self._svd2pac_as_ptr().add(96usize)) }
    }

    #[doc = "Register to test when peripheral has same name as register"]
    #[inline(always)]
    pub fn timer(&self) -> &'static self::TimerT {
        unsafe { self::TimerT::from_ptr(self._svd2pac_as_ptr().add(8192usize)) }
    }

    #[doc = "Test Cluster"]
    #[inline(always)]
    pub const fn cluster1(&self) -> crate::timer::Cluster1 {
        unsafe { crate::timer::_Cluster1::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(256usize)) }
    }

    #[doc = "Test Cluster array"]
    #[inline(always)]
    pub fn clusterdim(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::timer::_ClusterDim, 4, 0x100> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1000usize))
        }
    }
}

#[doc = "Register to test basic bitfield features"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitfieldReg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for BitfieldReg {
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
pub struct BitfieldRegT;
unsafe impl crate::common::AsPtr for BitfieldRegT {}
impl crate::common::Reg<BitfieldReg> for BitfieldRegT {}

unsafe impl crate::common::Read<BitfieldReg> for BitfieldRegT {}
unsafe impl crate::common::Write<BitfieldReg> for BitfieldRegT {}
impl BitfieldReg {
    #[doc = "Boolean Bitfield Read Only"]
    #[inline(always)]
    pub fn boolr(self) -> crate::common::RegisterFieldBool<0, 1, 0, BitfieldReg, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, BitfieldReg, common::R>::from_register(self, 0)
    }

    #[doc = "Boolean Bitfield Write Only"]
    #[inline(always)]
    pub fn boolw(self) -> crate::common::RegisterFieldBool<1, 1, 0, BitfieldReg, common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, BitfieldReg, common::W>::from_register(self, 0)
    }

    #[doc = "Boolean bitfield Read Write"]
    #[inline(always)]
    pub fn boolrw(self) -> crate::common::RegisterFieldBool<2, 1, 0, BitfieldReg, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, BitfieldReg, common::RW>::from_register(self, 0)
    }

    #[doc = "Raw Bitfield Read Only"]
    #[inline(always)]
    pub fn bitfieldr(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, BitfieldReg, common::R> {
        crate::common::RegisterField::<3, 0x7, 1, 0, u8, u8, BitfieldReg, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Bitfield Raw Write Only"]
    #[inline(always)]
    pub fn bitfieldw(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, BitfieldReg, common::W> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, u8, BitfieldReg, common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BitField Raw Read Write"]
    #[inline(always)]
    pub fn bitfieldrw(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, BitfieldReg, common::RW> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, u8, BitfieldReg, common::RW>::from_register(
            self, 0,
        )
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
        BitfieldReg,
        common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            bitfield_reg::BitfieldEnumerated,
            bitfield_reg::BitfieldEnumerated,
            BitfieldReg,
            common::RW,
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
        BitfieldReg,
        common::RW,
    > {
        assert!(index < 8);
        crate::common::RegisterField::<
            16,
            0x3,
            8,
            2,
            bitfield_reg::FieldArray,
            bitfield_reg::FieldArray,
            BitfieldReg,
            common::RW,
        >::from_register(self, index)
    }
}
impl crate::common::ResetValue<BitfieldReg> for BitfieldRegT {
    #[inline(always)]
    fn reset_value(&self) -> BitfieldReg {
        BitfieldReg::new(0)
    }
}
pub mod bitfield_reg {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct BitfieldEnumerated(u8);

    impl BitfieldEnumerated {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for BitfieldEnumerated {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for BitfieldEnumerated {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<BitfieldEnumerated> for u64 {
        #[inline(always)]
        fn from(value: BitfieldEnumerated) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for BitfieldEnumerated {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl BitfieldEnumerated {
        #[doc = "Core Clock"]
        pub const C_CLK: Self = Self(0);

        #[doc = "GPIO A, PIN 0"]
        pub const GPIOA_0: Self = Self(1);

        #[doc = "GPIO A, PIN 1"]
        pub const GPIOA_1: Self = Self(2);

        #[doc = "GPIO A, PIN 2"]
        pub const GPIOA_2: Self = Self(3);

        #[doc = "GPIO A, PIN 3"]
        pub const GPIOA_3: Self = Self(4);

        #[doc = "GPIO A, PIN 4"]
        pub const GPIOA_4: Self = Self(5);

        #[doc = "GPIO A, PIN 5"]
        pub const GPIOA_5: Self = Self(6);

        #[doc = "GPIO A, PIN 6"]
        pub const GPIOA_6: Self = Self(7);

        #[doc = "GPIO A, PIN 7"]
        pub const GPIOA_7: Self = Self(8);

        #[doc = "GPIO B, PIN 0"]
        pub const GPIOB_0: Self = Self(9);

        #[doc = "GPIO B, PIN 1"]
        pub const GPIOB_1: Self = Self(10);

        #[doc = "GPIO B, PIN 2"]
        pub const GPIOB_2: Self = Self(11);

        #[doc = "GPIO B, PIN 3"]
        pub const GPIOB_3: Self = Self(12);

        #[doc = "GPIO C, PIN 0"]
        pub const GPIOC_0: Self = Self(13);

        #[doc = "GPIO C, PIN 1"]
        pub const GPIOC_5: Self = Self(14);

        #[doc = "GPIO C, PIN 2"]
        pub const GPIOC_6: Self = Self(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct FieldArray(u8);

    impl FieldArray {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for FieldArray {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for FieldArray {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<FieldArray> for u64 {
        #[inline(always)]
        fn from(value: FieldArray) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for FieldArray {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl FieldArray {
        #[doc = "Only rising edges result in a counter increment or decrement"]
        pub const RISING: Self = Self(0);

        #[doc = "Only falling edges result in a counter increment or decrement"]
        pub const FALLING: Self = Self(1);

        #[doc = "Rising and falling edges result in a counter increment or decrement"]
        pub const BOTH: Self = Self(2);
    }
}

#[doc = "Status Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr {
    pub(crate) data: u16,
    pub(crate) mask: u16,
}

impl crate::common::RegisterValue for Sr {
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
pub struct SrT;
unsafe impl crate::common::AsPtr for SrT {}
impl crate::common::Reg<Sr> for SrT {}

unsafe impl crate::common::Read<Sr> for SrT {}
impl Sr {
    #[doc = "Shows if Timer is running or not"]
    #[doc = "0 = Stopped: Timer is not running"]
    #[doc = "1 = Running: Timer is running"]
    #[inline(always)]
    pub fn run(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sr, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sr, common::R>::from_register(self, 0)
    }

    #[doc = "Shows if the MATCH was hit"]
    #[doc = "0 = No_Match: The MATCH condition was not hit"]
    #[doc = "1 = Match_Hit: The MATCH condition was hit"]
    #[inline(always)]
    pub fn r#match(self) -> crate::common::RegisterFieldBool<8, 1, 0, Sr, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sr, common::RW>::from_register(self, 0)
    }

    #[doc = "Shows if an underflow occured. This flag is sticky"]
    #[doc = "0 = No_Underflow: No underflow occured since last clear"]
    #[doc = "1 = Underflow: A minimum of one underflow occured since last clear"]
    #[inline(always)]
    pub fn un(self) -> crate::common::RegisterFieldBool<9, 1, 0, Sr, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sr, common::RW>::from_register(self, 0)
    }

    #[doc = "Shows if an overflow occured. This flag is sticky"]
    #[doc = "0 = No_Overflow: No overflow occured since last clear"]
    #[doc = "1 = Overflow_occured: A minimum of one overflow occured since last clear"]
    #[inline(always)]
    pub fn ov(self) -> crate::common::RegisterFieldBool<10, 1, 0, Sr, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Sr, common::RW>::from_register(self, 0)
    }

    #[doc = "Shows if Timer is in RESET state"]
    #[doc = "0 = Ready: Timer is not in RESET state and can operate"]
    #[doc = "1 = In_Reset: Timer is in RESET state and can not operate"]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<12, 1, 0, Sr, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sr, common::R>::from_register(self, 0)
    }

    #[doc = "Shows the currently active RELOAD Register"]
    #[inline(always)]
    pub fn reload(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, sr::Reload, sr::Reload, Sr, common::R> {
        crate::common::RegisterField::<14,0x3,1,0,sr::Reload,sr::Reload,Sr,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Sr> for SrT {
    #[inline(always)]
    fn reset_value(&self) -> Sr {
        Sr::new(0)
    }
}
pub mod sr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Reload(u8);

    impl Reload {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Reload {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Reload {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Reload> for u64 {
        #[inline(always)]
        fn from(value: Reload) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Reload {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Reload {
        #[doc = "Reload Register number 0 is active"]
        pub const RELOAD_0: Self = Self(0);

        #[doc = "Reload Register number 1 is active"]
        pub const RELOAD_1: Self = Self(1);

        #[doc = "Reload Register number 2 is active"]
        pub const RELOAD_2: Self = Self(2);

        #[doc = "Reload Register number 3 is active"]
        pub const RELOAD_3: Self = Self(3);
    }
}

#[doc = "Interrupt Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int {
    pub(crate) data: u16,
    pub(crate) mask: u16,
}

impl crate::common::RegisterValue for Int {
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
pub struct IntT;
unsafe impl crate::common::AsPtr for IntT {}
impl crate::common::Reg<Int> for IntT {}

unsafe impl crate::common::Write<Int> for IntT {}
impl Int {
    #[doc = "Interrupt Enable"]
    #[doc = "0 = Disabled: Timer does not generate Interrupts"]
    #[doc = "1 = Enable: Timer triggers the TIMERn Interrupt"]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Int, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Int, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Mode, selects on which condition the Timer should generate an Interrupt"]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, int::Mode, int::Mode, Int, common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,int::Mode,int::Mode,Int,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Int> for IntT {
    #[inline(always)]
    fn reset_value(&self) -> Int {
        Int::new(0)
    }
}
pub mod int {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Mode(u8);

    impl Mode {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Mode {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Mode {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Mode> for u64 {
        #[inline(always)]
        fn from(value: Mode) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Mode {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Mode {
        #[doc = "Timer generates an Interrupt when the MATCH condition is hit"]
        pub const MATCH: Self = Self(0);

        #[doc = "Timer generates an Interrupt when it underflows"]
        pub const UNDERFLOW: Self = Self(1);

        #[doc = "Timer generates an Interrupt when it overflows"]
        pub const OVERFLOW: Self = Self(2);
    }
}

#[doc = "The Counter Register reflects the actual Value of the Timer/Counter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NobitfieldReg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for NobitfieldReg {
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
pub struct NobitfieldRegT;
unsafe impl crate::common::AsPtr for NobitfieldRegT {}
impl crate::common::Reg<NobitfieldReg> for NobitfieldRegT {}

unsafe impl crate::common::Read<NobitfieldReg> for NobitfieldRegT {}
unsafe impl crate::common::Write<NobitfieldReg> for NobitfieldRegT {}

impl crate::common::NoBitfieldReg for NobitfieldReg {}
impl crate::common::ResetValue<NobitfieldReg> for NobitfieldRegT {
    #[inline(always)]
    fn reset_value(&self) -> NobitfieldReg {
        NobitfieldReg::new(0)
    }
}

#[doc = "The Match Register stores the compare Value for the MATCH condition"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Match {
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
pub struct MatchT;
unsafe impl crate::common::AsPtr for MatchT {}
impl crate::common::Reg<Match> for MatchT {}

unsafe impl crate::common::Read<Match> for MatchT {}
unsafe impl crate::common::Write<Match> for MatchT {}

impl crate::common::NoBitfieldReg for Match {}
impl crate::common::ResetValue<Match> for MatchT {
    #[inline(always)]
    fn reset_value(&self) -> Match {
        Match::new(0)
    }
}

#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrescaleRd {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for PrescaleRd {
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
pub struct PrescaleRdT;
unsafe impl crate::common::AsPtr for PrescaleRdT {}
impl crate::common::Reg<PrescaleRd> for PrescaleRdT {}

unsafe impl crate::common::Read<PrescaleRd> for PrescaleRdT {}

impl crate::common::NoBitfieldReg for PrescaleRd {}
impl crate::common::ResetValue<PrescaleRd> for PrescaleRdT {
    #[inline(always)]
    fn reset_value(&self) -> PrescaleRd {
        PrescaleRd::new(0)
    }
}

#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrescaleWr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for PrescaleWr {
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
pub struct PrescaleWrT;
unsafe impl crate::common::AsPtr for PrescaleWrT {}
impl crate::common::Reg<PrescaleWr> for PrescaleWrT {}

unsafe impl crate::common::Write<PrescaleWr> for PrescaleWrT {}

impl crate::common::NoBitfieldReg for PrescaleWr {}
impl crate::common::ResetValue<PrescaleWr> for PrescaleWrT {
    #[inline(always)]
    fn reset_value(&self) -> PrescaleWr {
        PrescaleWr::new(0)
    }
}

#[doc = "Array of register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arrayreg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Arrayreg {
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
pub struct ArrayregT;
unsafe impl crate::common::AsPtr for ArrayregT {}
impl crate::common::Reg<Arrayreg> for ArrayregT {}

unsafe impl crate::common::Read<Arrayreg> for ArrayregT {}
unsafe impl crate::common::Write<Arrayreg> for ArrayregT {}

impl crate::common::NoBitfieldReg for Arrayreg {}
impl crate::common::ResetValue<Arrayreg> for ArrayregT {
    #[inline(always)]
    fn reset_value(&self) -> Arrayreg {
        Arrayreg::new(0)
    }
}

#[doc = "Another defintion register using alternate group"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitfieldRegAltGroup {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for BitfieldRegAltGroup {
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
pub struct BitfieldRegAltGroupT;
unsafe impl crate::common::AsPtr for BitfieldRegAltGroupT {}
impl crate::common::Reg<BitfieldRegAltGroup> for BitfieldRegAltGroupT {}

unsafe impl crate::common::Read<BitfieldRegAltGroup> for BitfieldRegAltGroupT {}
unsafe impl crate::common::Write<BitfieldRegAltGroup> for BitfieldRegAltGroupT {}

impl crate::common::NoBitfieldReg for BitfieldRegAltGroup {}
impl crate::common::ResetValue<BitfieldRegAltGroup> for BitfieldRegAltGroupT {
    #[inline(always)]
    fn reset_value(&self) -> BitfieldRegAltGroup {
        BitfieldRegAltGroup::new(0)
    }
}

#[doc = "64 bit register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Register64Bit {
    pub(crate) data: u64,
    pub(crate) mask: u64,
}

impl crate::common::RegisterValue for Register64Bit {
    type DataType = u64;

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
pub struct Register64BitT;
unsafe impl crate::common::AsPtr for Register64BitT {}
impl crate::common::Reg<Register64Bit> for Register64BitT {}

unsafe impl crate::common::Read<Register64Bit> for Register64BitT {}
unsafe impl crate::common::Write<Register64Bit> for Register64BitT {}
impl Register64Bit {
    #[doc = "1 = True"]
    #[doc = "0 = False"]
    #[inline(always)]
    pub fn boolean(self) -> crate::common::RegisterFieldBool<0, 1, 0, Register64Bit, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Register64Bit, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Register64Bit> for Register64BitT {
    #[inline(always)]
    fn reset_value(&self) -> Register64Bit {
        Register64Bit::new(18446744073709551615)
    }
}

#[doc = "Register to test when peripheral has same name as register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Timer {
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
pub struct TimerT;
unsafe impl crate::common::AsPtr for TimerT {}
impl crate::common::Reg<Timer> for TimerT {}

unsafe impl crate::common::Read<Timer> for TimerT {}
unsafe impl crate::common::Write<Timer> for TimerT {}

impl crate::common::NoBitfieldReg for Timer {}
impl crate::common::ResetValue<Timer> for TimerT {
    #[inline(always)]
    fn reset_value(&self) -> Timer {
        Timer::new(0)
    }
}

#[doc(hidden)]
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
    pub fn cr(&self) -> &'static cluster1::CrT {
        unsafe { cluster1::CrT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "A cluster inside another cluster"]
    #[inline(always)]
    pub const fn cluster1(&self) -> crate::timer::cluster1::Cluster1 {
        unsafe {
            crate::timer::cluster1::_Cluster1::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "HSSL"]
    #[inline(always)]
    pub const fn hssl(&self) -> crate::timer::cluster1::HsslHssl {
        unsafe {
            crate::timer::cluster1::_HsslHssl::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }
}

unsafe impl crate::common::AsPtr for _Cluster1 {
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
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Cr {
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
    pub struct CrT;
    unsafe impl crate::common::AsPtr for CrT {}
    impl crate::common::Reg<Cr> for CrT {}

    unsafe impl crate::common::Read<Cr> for CrT {}
    unsafe impl crate::common::Write<Cr> for CrT {}
    impl Cr {
        #[inline(always)]
        pub fn filed1(self) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Cr, common::RW> {
            crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, Cr, common::RW>::from_register(
                self, 0,
            )
        }

        #[inline(always)]
        pub fn psc(
            self,
        ) -> crate::common::RegisterField<3, 0x3, 1, 0, cr::Psc, cr::Psc, Cr, common::RW> {
            crate::common::RegisterField::<3,0x3,1,0,cr::Psc,cr::Psc,Cr,common::RW>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Cr> for CrT {
        #[inline(always)]
        fn reset_value(&self) -> Cr {
            Cr::new(0)
        }
    }
    pub mod cr {
        #[allow(unused_imports)]
        use crate::common;
        #[allow(unused_imports)]
        use crate::common::{
            AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
            RegisterValue as _, ResetValue as _, Write as _,
        };

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        #[repr(transparent)]
        pub struct Psc(u8);

        impl Psc {
            pub fn new(value: u8) -> Self {
                Self(value)
            }
        }

        impl crate::common::EnumBitfieldStruct for Psc {
            type RegNumberT = u8;

            fn value(&self) -> Self::RegNumberT {
                self.0
            }
        }

        impl From<u8> for Psc {
            #[inline(always)]
            fn from(value: u8) -> Self {
                Self(value)
            }
        }

        impl From<Psc> for u64 {
            #[inline(always)]
            fn from(value: Psc) -> Self {
                value.value().into()
            }
        }

        impl CastFrom<u64> for Psc {
            #[inline(always)]
            fn cast_from(val: u64) -> Self {
                Self(u8::cast_from(val))
            }
        }

        impl Psc {
            pub const VAL_1: Self = Self(1);
        }
    }
    #[doc(hidden)]
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
        pub fn nestedreg(&self) -> &'static cluster1::NestedRegT {
            unsafe { cluster1::NestedRegT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
        }
    }

    unsafe impl crate::common::AsPtr for _Cluster1 {
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
        use crate::common;
        #[allow(unused_imports)]
        use crate::common::{
            AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
            ResetValue as _, Write as _,
        };

        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NestedReg {
            pub(crate) data: u32,
            pub(crate) mask: u32,
        }

        impl crate::common::RegisterValue for NestedReg {
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
        pub struct NestedRegT;
        unsafe impl crate::common::AsPtr for NestedRegT {}
        impl crate::common::Reg<NestedReg> for NestedRegT {}

        unsafe impl crate::common::Read<NestedReg> for NestedRegT {}
        unsafe impl crate::common::Write<NestedReg> for NestedRegT {}

        impl crate::common::NoBitfieldReg for NestedReg {}
        impl crate::common::ResetValue<NestedReg> for NestedRegT {
            #[inline(always)]
            fn reset_value(&self) -> NestedReg {
                NestedReg::new(74565)
            }
        }
    }
    #[doc(hidden)]
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
            &self,
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

    unsafe impl crate::common::AsPtr for _HsslHssl {
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
        use crate::common;
        #[allow(unused_imports)]
        use crate::common::{
            AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
            ResetValue as _, Write as _,
        };
        #[doc(hidden)]
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
            pub fn hsslxcoky(&self) -> &'static hssl_hssl_ch::HssLxCoKyT {
                unsafe { hssl_hssl_ch::HssLxCoKyT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
            }
        }

        unsafe impl crate::common::AsPtr for _HsslHsslCh {
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
            use crate::common;
            #[allow(unused_imports)]
            use crate::common::{
                AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
                ResetValue as _, Write as _,
            };

            #[doc = "HSSL0 Channel 0 OK Service Request\n resetvalue={Application Reset:0x0}"]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct HssLxCoKy {
                pub(crate) data: u32,
                pub(crate) mask: u32,
            }

            impl crate::common::RegisterValue for HssLxCoKy {
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
            pub struct HssLxCoKyT;
            unsafe impl crate::common::AsPtr for HssLxCoKyT {}
            impl crate::common::Reg<HssLxCoKy> for HssLxCoKyT {}

            unsafe impl crate::common::Read<HssLxCoKy> for HssLxCoKyT {}
            unsafe impl crate::common::Write<HssLxCoKy> for HssLxCoKyT {}

            impl crate::common::NoBitfieldReg for HssLxCoKy {}
            impl crate::common::ResetValue<HssLxCoKy> for HssLxCoKyT {
                #[inline(always)]
                fn reset_value(&self) -> HssLxCoKy {
                    HssLxCoKy::new(0)
                }
            }
        }
    }
}
#[doc(hidden)]
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
    pub fn cr(&self) -> &'static clusterdim::CrT {
        unsafe { clusterdim::CrT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }
}

unsafe impl crate::common::AsPtr for _ClusterDim {
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
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Cr {
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
    pub struct CrT;
    unsafe impl crate::common::AsPtr for CrT {}
    impl crate::common::Reg<Cr> for CrT {}

    unsafe impl crate::common::Read<Cr> for CrT {}
    unsafe impl crate::common::Write<Cr> for CrT {}
    impl Cr {
        #[inline(always)]
        pub fn field1(self) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Cr, common::RW> {
            crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, Cr, common::RW>::from_register(
                self, 0,
            )
        }

        #[inline(always)]
        pub fn psc(
            self,
        ) -> crate::common::RegisterField<3, 0x3, 1, 0, cr::Psc, cr::Psc, Cr, common::RW> {
            crate::common::RegisterField::<3,0x3,1,0,cr::Psc,cr::Psc,Cr,common::RW>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Cr> for CrT {
        #[inline(always)]
        fn reset_value(&self) -> Cr {
            Cr::new(4096)
        }
    }
    pub mod cr {
        #[allow(unused_imports)]
        use crate::common;
        #[allow(unused_imports)]
        use crate::common::{
            AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
            RegisterValue as _, ResetValue as _, Write as _,
        };

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        #[repr(transparent)]
        pub struct Psc(u8);

        impl Psc {
            pub fn new(value: u8) -> Self {
                Self(value)
            }
        }

        impl crate::common::EnumBitfieldStruct for Psc {
            type RegNumberT = u8;

            fn value(&self) -> Self::RegNumberT {
                self.0
            }
        }

        impl From<u8> for Psc {
            #[inline(always)]
            fn from(value: u8) -> Self {
                Self(value)
            }
        }

        impl From<Psc> for u64 {
            #[inline(always)]
            fn from(value: Psc) -> Self {
                value.value().into()
            }
        }

        impl CastFrom<u64> for Psc {
            #[inline(always)]
            fn cast_from(val: u64) -> Self {
                Self(u8::cast_from(val))
            }
        }

        impl Psc {
            pub const VAL_1: Self = Self(1);
        }
    }
}
