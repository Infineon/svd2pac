/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 15:58:05 +0000

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
#[doc = r"Test cluster"]
unsafe impl ::core::marker::Send for super::Uart {}
unsafe impl ::core::marker::Sync for super::Uart {}
impl super::Uart {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "read-write reg"]
    #[inline(always)]
    pub const fn reg1_(&self) -> &'static crate::common::ClusterRegisterArray<self::Reg1T, 2, 0x4> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[doc = "Register with bitfields without enumeration"]
    #[inline(always)]
    pub fn regbitfieldraw(&self) -> &'static self::RegBitfieldRawT {
        unsafe { self::RegBitfieldRawT::from_ptr(self._svd2pac_as_ptr().add(256usize)) }
    }

    #[doc = "read write reg enum"]
    #[inline(always)]
    pub fn reg16bitenum(&self) -> &'static self::Reg16BitEnumT {
        unsafe { self::Reg16BitEnumT::from_ptr(self._svd2pac_as_ptr().add(260usize)) }
    }

    #[doc = "Read write whithout enum"]
    #[inline(always)]
    pub fn reg8bitraw(&self) -> &'static self::Reg8BitRawT {
        unsafe { self::Reg8BitRawT::from_ptr(self._svd2pac_as_ptr().add(262usize)) }
    }

    #[doc = "Read write without enum"]
    #[inline(always)]
    pub fn reg16bitraw(&self) -> &'static self::Reg16BitRawT {
        unsafe { self::Reg16BitRawT::from_ptr(self._svd2pac_as_ptr().add(264usize)) }
    }

    #[doc = "Read write without enum"]
    #[inline(always)]
    pub fn reg32bitraw(&self) -> &'static self::Reg32BitRawT {
        unsafe { self::Reg32BitRawT::from_ptr(self._svd2pac_as_ptr().add(272usize)) }
    }

    #[doc = "Enumvalues with different usage value"]
    #[inline(always)]
    pub fn regenumvalue(&self) -> &'static self::RegEnumValueT {
        unsafe { self::RegEnumValueT::from_ptr(self._svd2pac_as_ptr().add(512usize)) }
    }

    #[doc = "Cluster to test when peripheral has same name as register"]
    #[inline(always)]
    pub const fn uart(&self) -> crate::uart::Uart {
        unsafe { crate::uart::_Uart::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(4096usize)) }
    }
}

#[doc = "read-write reg"]
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

#[doc = "Register with bitfields without enumeration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegBitfieldRaw {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for RegBitfieldRaw {
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
pub struct RegBitfieldRawT;
unsafe impl crate::common::AsPtr for RegBitfieldRawT {}
impl crate::common::Reg<RegBitfieldRaw> for RegBitfieldRawT {}

unsafe impl crate::common::Read<RegBitfieldRaw> for RegBitfieldRawT {}
unsafe impl crate::common::Write<RegBitfieldRaw> for RegBitfieldRawT {}
impl RegBitfieldRaw {
    #[inline(always)]
    pub fn bitfield9bits(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, RegBitfieldRaw, common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,RegBitfieldRaw,common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bitfield17bits(
        self,
    ) -> crate::common::RegisterField<9, 0x3ffff, 1, 0, u32, u32, RegBitfieldRaw, common::RW> {
        crate::common::RegisterField::<9,0x3ffff,1,0,u32,u32,RegBitfieldRaw,common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bool(self) -> crate::common::RegisterFieldBool<27, 1, 0, RegBitfieldRaw, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, RegBitfieldRaw, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<RegBitfieldRaw> for RegBitfieldRawT {
    #[inline(always)]
    fn reset_value(&self) -> RegBitfieldRaw {
        RegBitfieldRaw::new(0)
    }
}

#[doc = "read write reg enum"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg16BitEnum {
    pub(crate) data: u16,
    pub(crate) mask: u16,
}

impl crate::common::RegisterValue for Reg16BitEnum {
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
pub struct Reg16BitEnumT;
unsafe impl crate::common::AsPtr for Reg16BitEnumT {}
impl crate::common::Reg<Reg16BitEnum> for Reg16BitEnumT {}

unsafe impl crate::common::Read<Reg16BitEnum> for Reg16BitEnumT {}
unsafe impl crate::common::Write<Reg16BitEnum> for Reg16BitEnumT {}
impl Reg16BitEnum {
    #[doc = "Check when bitfield size is not standard"]
    #[inline(always)]
    pub fn bitfield9bitsenum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        reg16bitenum::Bitfield9BitsEnum,
        reg16bitenum::Bitfield9BitsEnum,
        Reg16BitEnum,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            reg16bitenum::Bitfield9BitsEnum,
            reg16bitenum::Bitfield9BitsEnum,
            Reg16BitEnum,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Boolean with enum"]
    #[inline(always)]
    pub fn boolenum(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7f,
        1,
        0,
        reg16bitenum::Boolenum,
        reg16bitenum::Boolenum,
        Reg16BitEnum,
        common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7f,
            1,
            0,
            reg16bitenum::Boolenum,
            reg16bitenum::Boolenum,
            Reg16BitEnum,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Reg16BitEnum> for Reg16BitEnumT {
    #[inline(always)]
    fn reset_value(&self) -> Reg16BitEnum {
        Reg16BitEnum::new(0)
    }
}
pub mod reg16bitenum {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bitfield9BitsEnum(u16);

    impl Bitfield9BitsEnum {
        pub fn new(value: u16) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bitfield9BitsEnum {
        type RegNumberT = u16;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u16> for Bitfield9BitsEnum {
        #[inline(always)]
        fn from(value: u16) -> Self {
            Self(value)
        }
    }

    impl From<Bitfield9BitsEnum> for u64 {
        #[inline(always)]
        fn from(value: Bitfield9BitsEnum) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bitfield9BitsEnum {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u16::cast_from(val))
        }
    }

    impl Bitfield9BitsEnum {
        pub const VAL_0: Self = Self(0);

        pub const VAL_256: Self = Self(256);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Boolenum(u8);

    impl Boolenum {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Boolenum {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Boolenum {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Boolenum> for u64 {
        #[inline(always)]
        fn from(value: Boolenum) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Boolenum {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Boolenum {
        pub const BOOL_0: Self = Self(0);

        pub const BOOL_1: Self = Self(1);
    }
}

#[doc = "Read write whithout enum"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg8BitRaw {
    pub(crate) data: u8,
    pub(crate) mask: u8,
}

impl crate::common::RegisterValue for Reg8BitRaw {
    type DataType = u8;

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
pub struct Reg8BitRawT;
unsafe impl crate::common::AsPtr for Reg8BitRawT {}
impl crate::common::Reg<Reg8BitRaw> for Reg8BitRawT {}

unsafe impl crate::common::Read<Reg8BitRaw> for Reg8BitRawT {}
unsafe impl crate::common::Write<Reg8BitRaw> for Reg8BitRawT {}

impl crate::common::NoBitfieldReg for Reg8BitRaw {}
impl crate::common::ResetValue<Reg8BitRaw> for Reg8BitRawT {
    #[inline(always)]
    fn reset_value(&self) -> Reg8BitRaw {
        Reg8BitRaw::new(0)
    }
}

#[doc = "Read write without enum"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg16BitRaw {
    pub(crate) data: u16,
    pub(crate) mask: u16,
}

impl crate::common::RegisterValue for Reg16BitRaw {
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
pub struct Reg16BitRawT;
unsafe impl crate::common::AsPtr for Reg16BitRawT {}
impl crate::common::Reg<Reg16BitRaw> for Reg16BitRawT {}

unsafe impl crate::common::Read<Reg16BitRaw> for Reg16BitRawT {}
unsafe impl crate::common::Write<Reg16BitRaw> for Reg16BitRawT {}

impl crate::common::NoBitfieldReg for Reg16BitRaw {}
impl crate::common::ResetValue<Reg16BitRaw> for Reg16BitRawT {
    #[inline(always)]
    fn reset_value(&self) -> Reg16BitRaw {
        Reg16BitRaw::new(0)
    }
}

#[doc = "Read write without enum"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg32BitRaw {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Reg32BitRaw {
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
pub struct Reg32BitRawT;
unsafe impl crate::common::AsPtr for Reg32BitRawT {}
impl crate::common::Reg<Reg32BitRaw> for Reg32BitRawT {}

unsafe impl crate::common::Read<Reg32BitRaw> for Reg32BitRawT {}
unsafe impl crate::common::Write<Reg32BitRaw> for Reg32BitRawT {}

impl crate::common::NoBitfieldReg for Reg32BitRaw {}
impl crate::common::ResetValue<Reg32BitRaw> for Reg32BitRawT {
    #[inline(always)]
    fn reset_value(&self) -> Reg32BitRaw {
        Reg32BitRaw::new(0)
    }
}

#[doc = "Enumvalues with different usage value"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RegEnumValue {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for RegEnumValue {
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
pub struct RegEnumValueT;
unsafe impl crate::common::AsPtr for RegEnumValueT {}
impl crate::common::Reg<RegEnumValue> for RegEnumValueT {}

unsafe impl crate::common::Read<RegEnumValue> for RegEnumValueT {}
unsafe impl crate::common::Write<RegEnumValue> for RegEnumValueT {}
impl RegEnumValue {
    #[inline(always)]
    pub fn only_read_enum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        regenumvalue::OnlyReadEnumRead,
        u8,
        RegEnumValue,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            regenumvalue::OnlyReadEnumRead,
            u8,
            RegEnumValue,
            common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn only_write_enum(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        u8,
        regenumvalue::OnlyWriteEnumWrite,
        RegEnumValue,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            u8,
            regenumvalue::OnlyWriteEnumWrite,
            RegEnumValue,
            common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn read_write_enum(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        regenumvalue::ReadWriteEnum,
        regenumvalue::ReadWriteEnum,
        RegEnumValue,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            regenumvalue::ReadWriteEnum,
            regenumvalue::ReadWriteEnum,
            RegEnumValue,
            common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn read_write_enum_split(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        regenumvalue::ReadWriteEnumSplitRead,
        regenumvalue::ReadWriteEnumSplitWrite,
        RegEnumValue,
        common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            regenumvalue::ReadWriteEnumSplitRead,
            regenumvalue::ReadWriteEnumSplitWrite,
            RegEnumValue,
            common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn read_write_enum_split_binary(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        regenumvalue::ReadWriteEnumSplitBinaryRead,
        regenumvalue::ReadWriteEnumSplitBinaryWrite,
        RegEnumValue,
        common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            regenumvalue::ReadWriteEnumSplitBinaryRead,
            regenumvalue::ReadWriteEnumSplitBinaryWrite,
            RegEnumValue,
            common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn inherited_access_enum(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        regenumvalue::InheritedAccessEnumRead,
        regenumvalue::InheritedAccessEnumWrite,
        RegEnumValue,
        common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            regenumvalue::InheritedAccessEnumRead,
            regenumvalue::InheritedAccessEnumWrite,
            RegEnumValue,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<RegEnumValue> for RegEnumValueT {
    #[inline(always)]
    fn reset_value(&self) -> RegEnumValue {
        RegEnumValue::new(0)
    }
}
pub mod regenumvalue {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct OnlyReadEnumRead(u8);

    impl OnlyReadEnumRead {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for OnlyReadEnumRead {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for OnlyReadEnumRead {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<OnlyReadEnumRead> for u64 {
        #[inline(always)]
        fn from(value: OnlyReadEnumRead) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for OnlyReadEnumRead {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl OnlyReadEnumRead {
        pub const VALUE_2: Self = Self(2);

        pub const VALUE_1: Self = Self(1);

        pub const VALUE_0: Self = Self(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct OnlyWriteEnumWrite(u8);

    impl OnlyWriteEnumWrite {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for OnlyWriteEnumWrite {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for OnlyWriteEnumWrite {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<OnlyWriteEnumWrite> for u64 {
        #[inline(always)]
        fn from(value: OnlyWriteEnumWrite) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for OnlyWriteEnumWrite {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl OnlyWriteEnumWrite {
        pub const VALUE_2: Self = Self(2);

        pub const VALUE_1: Self = Self(1);

        pub const VALUE_0: Self = Self(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReadWriteEnum(u8);

    impl ReadWriteEnum {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReadWriteEnum {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReadWriteEnum {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReadWriteEnum> for u64 {
        #[inline(always)]
        fn from(value: ReadWriteEnum) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReadWriteEnum {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl ReadWriteEnum {
        pub const VALUE_2: Self = Self(2);

        pub const VALUE_1: Self = Self(1);

        pub const VALUE_0: Self = Self(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReadWriteEnumSplitRead(u8);

    impl ReadWriteEnumSplitRead {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReadWriteEnumSplitRead {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReadWriteEnumSplitRead {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReadWriteEnumSplitRead> for u64 {
        #[inline(always)]
        fn from(value: ReadWriteEnumSplitRead) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReadWriteEnumSplitRead {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl ReadWriteEnumSplitRead {
        pub const VALUE_2: Self = Self(2);

        pub const VALUE_1: Self = Self(1);

        pub const VALUE_0: Self = Self(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReadWriteEnumSplitWrite(u8);

    impl ReadWriteEnumSplitWrite {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReadWriteEnumSplitWrite {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReadWriteEnumSplitWrite {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReadWriteEnumSplitWrite> for u64 {
        #[inline(always)]
        fn from(value: ReadWriteEnumSplitWrite) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReadWriteEnumSplitWrite {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl ReadWriteEnumSplitWrite {
        pub const VALUE_2: Self = Self(0);

        pub const VALUE_1: Self = Self(2);

        pub const VALUE_0: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReadWriteEnumSplitBinaryRead(u8);

    impl ReadWriteEnumSplitBinaryRead {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReadWriteEnumSplitBinaryRead {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReadWriteEnumSplitBinaryRead {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReadWriteEnumSplitBinaryRead> for u64 {
        #[inline(always)]
        fn from(value: ReadWriteEnumSplitBinaryRead) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReadWriteEnumSplitBinaryRead {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl ReadWriteEnumSplitBinaryRead {
        pub const VALUE_1: Self = Self(1);

        pub const VALUE_0: Self = Self(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReadWriteEnumSplitBinaryWrite(u8);

    impl ReadWriteEnumSplitBinaryWrite {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReadWriteEnumSplitBinaryWrite {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReadWriteEnumSplitBinaryWrite {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReadWriteEnumSplitBinaryWrite> for u64 {
        #[inline(always)]
        fn from(value: ReadWriteEnumSplitBinaryWrite) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReadWriteEnumSplitBinaryWrite {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl ReadWriteEnumSplitBinaryWrite {
        pub const VALUE_2: Self = Self(0);

        pub const VALUE_0: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct InheritedAccessEnumRead(u8);

    impl InheritedAccessEnumRead {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for InheritedAccessEnumRead {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for InheritedAccessEnumRead {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<InheritedAccessEnumRead> for u64 {
        #[inline(always)]
        fn from(value: InheritedAccessEnumRead) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for InheritedAccessEnumRead {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl InheritedAccessEnumRead {
        pub const VALUE_1: Self = Self(1);

        pub const VALUE_0: Self = Self(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct InheritedAccessEnumWrite(u8);

    impl InheritedAccessEnumWrite {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for InheritedAccessEnumWrite {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for InheritedAccessEnumWrite {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<InheritedAccessEnumWrite> for u64 {
        #[inline(always)]
        fn from(value: InheritedAccessEnumWrite) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for InheritedAccessEnumWrite {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl InheritedAccessEnumWrite {
        pub const VALUE_2: Self = Self(0);

        pub const VALUE_0: Self = Self(1);
    }
}

#[doc(hidden)]
#[non_exhaustive]
pub struct _Uart;

#[doc = "Cluster to test when peripheral has same name as register"]
pub type Uart = &'static _Uart;

unsafe impl ::core::marker::Sync for _Uart {}
impl _Uart {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[inline(always)]
    pub fn uart(&self) -> &'static uart::UartT {
        unsafe { uart::UartT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }
}

unsafe impl crate::common::AsPtr for _Uart {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod uart {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uart {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Uart {
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
    pub struct UartT;
    unsafe impl crate::common::AsPtr for UartT {}
    impl crate::common::Reg<Uart> for UartT {}

    unsafe impl crate::common::Read<Uart> for UartT {}
    unsafe impl crate::common::Write<Uart> for UartT {}
    impl Uart {
        #[inline(always)]
        pub fn uart(self) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Uart, common::RW> {
            crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, Uart, common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl crate::common::ResetValue<Uart> for UartT {
        #[inline(always)]
        fn reset_value(&self) -> Uart {
            Uart::new(0)
        }
    }
}
