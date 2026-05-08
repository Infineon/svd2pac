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
pub const fn reg1_(&self) -> &'static crate::common::ClusterRegisterArray<crate::common::Reg<self::Reg1_SPEC, crate::common::RW>, 2, 0x4> {
    unsafe { crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize)) }
}

 
#[doc = "Register with bitfields without enumeration"]
#[inline(always)]
pub const fn regbitfieldraw(&self) -> &'static crate::common::Reg<self::RegBitfieldRaw_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::RegBitfieldRaw_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(256usize)) }
}

 
#[doc = "read write reg enum"]
#[inline(always)]
pub const fn reg16bitenum(&self) -> &'static crate::common::Reg<self::Reg16BitEnum_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::Reg16BitEnum_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(260usize)) }
}

 
#[doc = "Read write whithout enum"]
#[inline(always)]
pub const fn reg8bitraw(&self) -> &'static crate::common::Reg<self::Reg8BitRaw_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::Reg8BitRaw_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(262usize)) }
}

 
#[doc = "Read write without enum"]
#[inline(always)]
pub const fn reg16bitraw(&self) -> &'static crate::common::Reg<self::Reg16BitRaw_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::Reg16BitRaw_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(264usize)) }
}

 
#[doc = "Read write without enum"]
#[inline(always)]
pub const fn reg32bitraw(&self) -> &'static crate::common::Reg<self::Reg32BitRaw_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::Reg32BitRaw_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(272usize)) }
}

 
#[doc = "Enumvalues with different usage value"]
#[inline(always)]
pub const fn regenumvalue(&self) -> &'static crate::common::Reg<self::RegEnumValue_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<self::RegEnumValue_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(512usize)) }
}
 
#[doc = "Cluster to test when peripheral has same name as register"]
#[inline(always)]
pub const fn uart(self) -> crate::uart::Uart{
    unsafe {   crate::uart::_Uart::_svd2pac_from_ptr(self._svd2pac_as_ptr().add(4096usize)) }
}

}
#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Reg1_SPEC;
impl crate::sealed::RegSpec for Reg1_SPEC {
    type DataType = u32;
}
 
#[doc = "read-write reg"]
pub type  Reg1 = crate::RegValueT<Reg1_SPEC>;


impl NoBitfieldReg<Reg1_SPEC> for Reg1 {}
impl ::core::default::Default for Reg1 {
    #[inline(always)]
    fn default() -> Reg1 {
        <crate::RegValueT::<Reg1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct RegBitfieldRaw_SPEC;
impl crate::sealed::RegSpec for RegBitfieldRaw_SPEC {
    type DataType = u32;
}
 
#[doc = "Register with bitfields without enumeration"]
pub type  RegBitfieldRaw = crate::RegValueT<RegBitfieldRaw_SPEC>;

impl RegBitfieldRaw {
    
    #[inline(always)]
    pub fn bitfield9bits(self) -> crate::common::RegisterField<0,0x1ff,1,0,u16,u16,RegBitfieldRaw_SPEC,crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,RegBitfieldRaw_SPEC,crate::common::RW>::from_register(self,0)
    }
    
    #[inline(always)]
    pub fn bitfield17bits(self) -> crate::common::RegisterField<9,0x3ffff,1,0,u32,u32,RegBitfieldRaw_SPEC,crate::common::RW> {
        crate::common::RegisterField::<9,0x3ffff,1,0,u32,u32,RegBitfieldRaw_SPEC,crate::common::RW>::from_register(self,0)
    }
    
    #[inline(always)]
    pub fn bool(self) -> 
    crate::common::RegisterFieldBool<27,1,0,RegBitfieldRaw_SPEC,crate::common::RW> {
        
    crate::common::RegisterFieldBool::<27,1,0,RegBitfieldRaw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RegBitfieldRaw {
    #[inline(always)]
    fn default() -> RegBitfieldRaw {
        <crate::RegValueT::<RegBitfieldRaw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Reg16BitEnum_SPEC;
impl crate::sealed::RegSpec for Reg16BitEnum_SPEC {
    type DataType = u16;
}
 
#[doc = "read write reg enum"]
pub type  Reg16BitEnum = crate::RegValueT<Reg16BitEnum_SPEC>;

impl Reg16BitEnum {
     
#[doc = "Check when bitfield size is not standard"]
    #[inline(always)]
    pub fn bitfield9bitsenum(self) -> crate::common::RegisterField<0,0x1ff,1,0,reg16bitenum::Bitfield9BitsEnum,reg16bitenum::Bitfield9BitsEnum,Reg16BitEnum_SPEC,crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,reg16bitenum::Bitfield9BitsEnum,reg16bitenum::Bitfield9BitsEnum,Reg16BitEnum_SPEC,crate::common::RW>::from_register(self,0)
    }
     
#[doc = "Boolean with enum"]
    #[inline(always)]
    pub fn boolenum(self) -> crate::common::RegisterField<9,0x7f,1,0,reg16bitenum::Boolenum,reg16bitenum::Boolenum,Reg16BitEnum_SPEC,crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,reg16bitenum::Boolenum,reg16bitenum::Boolenum,Reg16BitEnum_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Reg16BitEnum {
    #[inline(always)]
    fn default() -> Reg16BitEnum {
        <crate::RegValueT::<Reg16BitEnum_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod reg16bitenum {
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bitfield9BitsEnum_SPEC;
    pub type  Bitfield9BitsEnum = crate::EnumBitfieldStruct<u16,Bitfield9BitsEnum_SPEC>;
    impl Bitfield9BitsEnum {
        
        pub const VAL_0:Self =Self::new(0);
        
        pub const VAL_256:Self =Self::new(256);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boolenum_SPEC;
    pub type  Boolenum = crate::EnumBitfieldStruct<u8,Boolenum_SPEC>;
    impl Boolenum {
        
        pub const BOOL_0:Self =Self::new(0);
        
        pub const BOOL_1:Self =Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Reg8BitRaw_SPEC;
impl crate::sealed::RegSpec for Reg8BitRaw_SPEC {
    type DataType = u8;
}
 
#[doc = "Read write whithout enum"]
pub type  Reg8BitRaw = crate::RegValueT<Reg8BitRaw_SPEC>;


impl NoBitfieldReg<Reg8BitRaw_SPEC> for Reg8BitRaw {}
impl ::core::default::Default for Reg8BitRaw {
    #[inline(always)]
    fn default() -> Reg8BitRaw {
        <crate::RegValueT::<Reg8BitRaw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Reg16BitRaw_SPEC;
impl crate::sealed::RegSpec for Reg16BitRaw_SPEC {
    type DataType = u16;
}
 
#[doc = "Read write without enum"]
pub type  Reg16BitRaw = crate::RegValueT<Reg16BitRaw_SPEC>;


impl NoBitfieldReg<Reg16BitRaw_SPEC> for Reg16BitRaw {}
impl ::core::default::Default for Reg16BitRaw {
    #[inline(always)]
    fn default() -> Reg16BitRaw {
        <crate::RegValueT::<Reg16BitRaw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Reg32BitRaw_SPEC;
impl crate::sealed::RegSpec for Reg32BitRaw_SPEC {
    type DataType = u32;
}
 
#[doc = "Read write without enum"]
pub type  Reg32BitRaw = crate::RegValueT<Reg32BitRaw_SPEC>;


impl NoBitfieldReg<Reg32BitRaw_SPEC> for Reg32BitRaw {}
impl ::core::default::Default for Reg32BitRaw {
    #[inline(always)]
    fn default() -> Reg32BitRaw {
        <crate::RegValueT::<Reg32BitRaw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct RegEnumValue_SPEC;
impl crate::sealed::RegSpec for RegEnumValue_SPEC {
    type DataType = u32;
}
 
#[doc = "Enumvalues with different usage value"]
pub type  RegEnumValue = crate::RegValueT<RegEnumValue_SPEC>;

impl RegEnumValue {
    
    #[inline(always)]
    pub fn only_read_enum(self) -> crate::common::RegisterField<0,0x7,1,0,regenumvalue::OnlyReadEnumRead,u8,RegEnumValue_SPEC,crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,regenumvalue::OnlyReadEnumRead,u8,RegEnumValue_SPEC,crate::common::RW>::from_register(self,0)
    }
    
    #[inline(always)]
    pub fn only_write_enum(self) -> crate::common::RegisterField<3,0x7,1,0,u8,regenumvalue::OnlyWriteEnumWrite,RegEnumValue_SPEC,crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8,regenumvalue::OnlyWriteEnumWrite,RegEnumValue_SPEC,crate::common::RW>::from_register(self,0)
    }
    
    #[inline(always)]
    pub fn read_write_enum(self) -> crate::common::RegisterField<6,0x3,1,0,regenumvalue::ReadWriteEnum,regenumvalue::ReadWriteEnum,RegEnumValue_SPEC,crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,regenumvalue::ReadWriteEnum,regenumvalue::ReadWriteEnum,RegEnumValue_SPEC,crate::common::RW>::from_register(self,0)
    }
    
    #[inline(always)]
    pub fn read_write_enum_split(self) -> crate::common::RegisterField<8,0x3,1,0,regenumvalue::ReadWriteEnumSplitRead,regenumvalue::ReadWriteEnumSplitWrite,RegEnumValue_SPEC,crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,regenumvalue::ReadWriteEnumSplitRead,regenumvalue::ReadWriteEnumSplitWrite,RegEnumValue_SPEC,crate::common::RW>::from_register(self,0)
    }
    
    #[inline(always)]
    pub fn read_write_enum_split_binary(self) -> crate::common::RegisterField<10,0x1,1,0,regenumvalue::ReadWriteEnumSplitBinaryRead,regenumvalue::ReadWriteEnumSplitBinaryWrite,RegEnumValue_SPEC,crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,regenumvalue::ReadWriteEnumSplitBinaryRead,regenumvalue::ReadWriteEnumSplitBinaryWrite,RegEnumValue_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RegEnumValue {
    #[inline(always)]
    fn default() -> RegEnumValue {
        <crate::RegValueT::<RegEnumValue_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod regenumvalue {
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OnlyReadEnumRead_SPEC;
    pub type  OnlyReadEnumRead = crate::EnumBitfieldStruct<u8,OnlyReadEnumRead_SPEC>;
    impl OnlyReadEnumRead {
        
        pub const VALUE_2:Self =Self::new(2);
        
        pub const VALUE_1:Self =Self::new(1);
        
        pub const VALUE_0:Self =Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OnlyWriteEnumWrite_SPEC;
    pub type  OnlyWriteEnumWrite = crate::EnumBitfieldStruct<u8,OnlyWriteEnumWrite_SPEC>;
    impl OnlyWriteEnumWrite {
        
        pub const VALUE_2:Self =Self::new(2);
        
        pub const VALUE_1:Self =Self::new(1);
        
        pub const VALUE_0:Self =Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ReadWriteEnum_SPEC;
    pub type  ReadWriteEnum = crate::EnumBitfieldStruct<u8,ReadWriteEnum_SPEC>;
    impl ReadWriteEnum {
        
        pub const VALUE_2:Self =Self::new(2);
        
        pub const VALUE_1:Self =Self::new(1);
        
        pub const VALUE_0:Self =Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ReadWriteEnumSplitRead_SPEC;
    pub type  ReadWriteEnumSplitRead = crate::EnumBitfieldStruct<u8,ReadWriteEnumSplitRead_SPEC>;
    impl ReadWriteEnumSplitRead {
        
        pub const VALUE_2:Self =Self::new(2);
        
        pub const VALUE_1:Self =Self::new(1);
        
        pub const VALUE_0:Self =Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ReadWriteEnumSplitWrite_SPEC;
    pub type  ReadWriteEnumSplitWrite = crate::EnumBitfieldStruct<u8,ReadWriteEnumSplitWrite_SPEC>;
    impl ReadWriteEnumSplitWrite {
        
        pub const VALUE_2:Self =Self::new(0);
        
        pub const VALUE_1:Self =Self::new(2);
        
        pub const VALUE_0:Self =Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ReadWriteEnumSplitBinaryRead_SPEC;
    pub type  ReadWriteEnumSplitBinaryRead = crate::EnumBitfieldStruct<u8,ReadWriteEnumSplitBinaryRead_SPEC>;
    impl ReadWriteEnumSplitBinaryRead {
        
        pub const VALUE_1:Self =Self::new(1);
        
        pub const VALUE_0:Self =Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ReadWriteEnumSplitBinaryWrite_SPEC;
    pub type  ReadWriteEnumSplitBinaryWrite = crate::EnumBitfieldStruct<u8,ReadWriteEnumSplitBinaryWrite_SPEC>;
    impl ReadWriteEnumSplitBinaryWrite {
        
        pub const VALUE_2:Self =Self::new(0);
        
        pub const VALUE_0:Self =Self::new(1);
    }
}

 
#[doc = "Cluster to test when peripheral has same name as register"]
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
pub const fn uart(&self) -> &'static crate::common::Reg<uart::Uart_SPEC, crate::common::RW> {
    unsafe {   crate::common::Reg::<uart::Uart_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
}
    }

unsafe impl AsPtr for _Uart {
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
    use crate::common::{*};
    #[doc(hidden)]
#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Uart_SPEC;
impl crate::sealed::RegSpec for Uart_SPEC {
    type DataType = u32;
}

pub type  Uart = crate::RegValueT<Uart_SPEC>;

impl Uart {
    
    #[inline(always)]
    pub fn uart(self) -> crate::common::RegisterField<0,0x7,1,0,u8,u8,Uart_SPEC,crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Uart_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart {
    #[inline(always)]
    fn default() -> Uart {
        <crate::RegValueT::<Uart_SPEC> as RegisterValue<_>>::new(0)
    }
}

    }







