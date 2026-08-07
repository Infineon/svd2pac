/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 15:56:11 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common;
#[allow(unused_imports)]
use crate::common::{
    AsPtr as _, NoBitfieldReg as _, ReadCore as _, Reg as _, RegisterValue as _,
    ResetValue as _, WriteCore as _,
};
#[doc = r"CPU"]
unsafe impl ::core::marker::Send for super::CsfrCpu1  {}
unsafe impl ::core::marker::Sync for super::CsfrCpu1  {}
impl super::CsfrCpu1 {
#[doc = r"CPUx SIST Mode Access Control Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn smacon(&self) -> crate::common::RegCore<self::Smacon, 0x900c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn diear(&self) -> crate::common::RegCore<self::Diear, 0x9020,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dietr(&self) -> crate::common::RegCore<self::Dietr, 0x9024,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn piear(&self) -> crate::common::RegCore<self::Piear, 0x9210,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn pietr(&self) -> crate::common::RegCore<self::Pietr, 0x9214,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Task Address Space Identifier Register\n resetvalue={Application Reset:0x1F}"]
#[inline(always)]
pub const fn task_asi(&self) -> crate::common::RegCore<self::TaskAsi, 0x8004,31> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
#[inline(always)]
pub const fn pma0(&self) -> crate::common::RegCore<self::Pma0, 0x8100,768> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Code Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
#[inline(always)]
pub const fn pma1(&self) -> crate::common::RegCore<self::Pma1, 0x8104,768> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx  Peripheral Space Identifier register\n resetvalue={Application Reset:0x0C000}"]
#[inline(always)]
pub const fn pma2(&self) -> crate::common::RegCore<self::Pma2, 0x8108,49152> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Compatibility Control Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
#[inline(always)]
pub const fn compat(&self) -> crate::common::RegCore<self::Compat, 0x9400,4294967295> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Previous Context Information Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn pcxi(&self) -> crate::common::RegCore<self::Pcxi, 0xfe00,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Status Word\n resetvalue={Application Reset:0x0B80}"]
#[inline(always)]
pub const fn psw(&self) -> crate::common::RegCore<self::Psw, 0xfe04,2944> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Counter\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn pc(&self) -> crate::common::RegCore<self::Pc, 0xfe08,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx System Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x0}"]
#[inline(always)]
pub const fn syscon(&self) -> crate::common::RegCore<self::Syscon, 0xfe14,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Identification Register TC1.6.2P\n resetvalue={Application Reset:0x0C0C021}"]
#[inline(always)]
pub const fn cpu_id(&self) -> crate::common::RegCore<self::CpuId, 0xfe18,12632097> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Core Identification Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn core_id(&self) -> crate::common::RegCore<self::CoreId, 0xfe1c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Base Interrupt Vector Table Pointer\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn biv(&self) -> crate::common::RegCore<self::Biv, 0xfe20,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Base Trap Vector Table Pointer\n resetvalue={Application Reset:0x0A0000100}"]
#[inline(always)]
pub const fn btv(&self) -> crate::common::RegCore<self::Btv, 0xfe24,2684354816> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Interrupt Stack Pointer\n resetvalue={Application Reset:0x100}"]
#[inline(always)]
pub const fn isp(&self) -> crate::common::RegCore<self::Isp, 0xfe28,256> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn icr(&self) -> crate::common::RegCore<self::Icr, 0xfe2c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Free CSA List Head Pointer\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn fcx(&self) -> crate::common::RegCore<self::Fcx, 0xfe38,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Free CSA List Limit Pointer\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn lcx(&self) -> crate::common::RegCore<self::Lcx, 0xfe3c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Customer ID register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cus_id(&self) -> crate::common::RegCore<self::CusId, 0xfe50,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dy0(&self)-> crate::common::RegCore<self::Dy, 0xff00,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy1(&self)-> crate::common::RegCore<self::Dy, 0xff04,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy2(&self)-> crate::common::RegCore<self::Dy, 0xff08,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy3(&self)-> crate::common::RegCore<self::Dy, 0xff0c,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy4(&self)-> crate::common::RegCore<self::Dy, 0xff10,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy5(&self)-> crate::common::RegCore<self::Dy, 0xff14,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy6(&self)-> crate::common::RegCore<self::Dy, 0xff18,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy7(&self)-> crate::common::RegCore<self::Dy, 0xff1c,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy8(&self)-> crate::common::RegCore<self::Dy, 0xff20,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy9(&self)-> crate::common::RegCore<self::Dy, 0xff24,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy10(&self)-> crate::common::RegCore<self::Dy, 0xff28,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy11(&self)-> crate::common::RegCore<self::Dy, 0xff2c,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy12(&self)-> crate::common::RegCore<self::Dy, 0xff30,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy13(&self)-> crate::common::RegCore<self::Dy, 0xff34,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy14(&self)-> crate::common::RegCore<self::Dy, 0xff38,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn dy15(&self)-> crate::common::RegCore<self::Dy, 0xff3c,0> {
    unsafe { crate::common::RegCore::new() }
}


#[doc = r"CPUx Address General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn ay0(&self)-> crate::common::RegCore<self::Ay, 0xff80,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay1(&self)-> crate::common::RegCore<self::Ay, 0xff84,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay2(&self)-> crate::common::RegCore<self::Ay, 0xff88,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay3(&self)-> crate::common::RegCore<self::Ay, 0xff8c,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay4(&self)-> crate::common::RegCore<self::Ay, 0xff90,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay5(&self)-> crate::common::RegCore<self::Ay, 0xff94,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay6(&self)-> crate::common::RegCore<self::Ay, 0xff98,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay7(&self)-> crate::common::RegCore<self::Ay, 0xff9c,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay8(&self)-> crate::common::RegCore<self::Ay, 0xffa0,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay9(&self)-> crate::common::RegCore<self::Ay, 0xffa4,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay10(&self)-> crate::common::RegCore<self::Ay, 0xffa8,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay11(&self)-> crate::common::RegCore<self::Ay, 0xffac,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay12(&self)-> crate::common::RegCore<self::Ay, 0xffb0,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay13(&self)-> crate::common::RegCore<self::Ay, 0xffb4,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay14(&self)-> crate::common::RegCore<self::Ay, 0xffb8,0> {
    unsafe { crate::common::RegCore::new() }
}
pub const fn ay15(&self)-> crate::common::RegCore<self::Ay, 0xffbc,0> {
    unsafe { crate::common::RegCore::new() }
}


#[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpxe_0(&self) -> crate::common::RegCore<self::Cpxe0, 0xe000,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpxe_1(&self) -> crate::common::RegCore<self::Cpxe1, 0xe004,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpxe_2(&self) -> crate::common::RegCore<self::Cpxe2, 0xe008,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpxe_3(&self) -> crate::common::RegCore<self::Cpxe3, 0xe00c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpre_0(&self) -> crate::common::RegCore<self::Dpre0, 0xe010,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpre_1(&self) -> crate::common::RegCore<self::Dpre1, 0xe014,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpre_2(&self) -> crate::common::RegCore<self::Dpre2, 0xe018,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpre_3(&self) -> crate::common::RegCore<self::Dpre3, 0xe01c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpwe_0(&self) -> crate::common::RegCore<self::Dpwe0, 0xe020,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpwe_1(&self) -> crate::common::RegCore<self::Dpwe1, 0xe024,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpwe_2(&self) -> crate::common::RegCore<self::Dpwe2, 0xe028,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpwe_3(&self) -> crate::common::RegCore<self::Dpwe3, 0xe02c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpxe_4(&self) -> crate::common::RegCore<self::Cpxe4, 0xe040,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpxe_5(&self) -> crate::common::RegCore<self::Cpxe5, 0xe044,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpre_4(&self) -> crate::common::RegCore<self::Dpre4, 0xe050,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpre_5(&self) -> crate::common::RegCore<self::Dpre5, 0xe054,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpwe_4(&self) -> crate::common::RegCore<self::Dpwe4, 0xe060,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpwe_5(&self) -> crate::common::RegCore<self::Dpwe5, 0xe064,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Counter Control\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn cctrl(&self) -> crate::common::RegCore<self::Cctrl, 0xfc00,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx CPU Clock Cycle Count\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn ccnt(&self) -> crate::common::RegCore<self::Ccnt, 0xfc04,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Instruction Count\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn icnt(&self) -> crate::common::RegCore<self::Icnt, 0xfc08,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Multi Count Register 1\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn m1cnt(&self) -> crate::common::RegCore<self::M1Cnt, 0xfc0c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Multi Count Register 2\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn m2cnt(&self) -> crate::common::RegCore<self::M2Cnt, 0xfc10,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Multi Count Register 3\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn m3cnt(&self) -> crate::common::RegCore<self::M3Cnt, 0xfc14,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Debug Status Register\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn dbgsr(&self) -> crate::common::RegCore<self::Dbgsr, 0xfd00,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx External Event Register\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn exevt(&self) -> crate::common::RegCore<self::Exevt, 0xfd08,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Core Register Access Event\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn crevt(&self) -> crate::common::RegCore<self::Crevt, 0xfd0c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Software Debug Event\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn swevt(&self) -> crate::common::RegCore<self::Swevt, 0xfd10,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx TriggerAddressx\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn trig_acc(&self) -> crate::common::RegCore<self::TrigAcc, 0xfd30,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Debug Monitor Start Address\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dms(&self) -> crate::common::RegCore<self::Dms, 0xfd40,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Debug Context Save Area Pointer\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dcx(&self) -> crate::common::RegCore<self::Dcx, 0xfd44,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Debug Trap Control Register\n resetvalue={Application Reset:0x1}"]
#[inline(always)]
pub const fn dbgtcr(&self) -> crate::common::RegCore<self::Dbgtcr, 0xfd48,1> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx SRI Error Generation Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn segen(&self) -> crate::common::RegCore<self::Segen, 0x1030,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Control Register 2\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dcon2(&self) -> crate::common::RegCore<self::Dcon2, 0x9000,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dstr(&self) -> crate::common::RegCore<self::Dstr, 0x9010,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Asynchronous Trap Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn datr(&self) -> crate::common::RegCore<self::Datr, 0x9018,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Error Address Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn deadd(&self) -> crate::common::RegCore<self::Deadd, 0x901c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Data Memory Control Register\n resetvalue={Application Reset:0x2}"]
#[inline(always)]
pub const fn dcon0(&self) -> crate::common::RegCore<self::Dcon0, 0x9040,2> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn pstr(&self) -> crate::common::RegCore<self::Pstr, 0x9200,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Control 1\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn pcon1(&self) -> crate::common::RegCore<self::Pcon1, 0x9204,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Control 2\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn pcon2(&self) -> crate::common::RegCore<self::Pcon2, 0x9208,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = r"CPUx Program Control 0\n resetvalue={Application Reset:0x2}"]
#[inline(always)]
pub const fn pcon0(&self) -> crate::common::RegCore<self::Pcon0, 0x920c,2> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = "DPR"]#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l0(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc000,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u0(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc004,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l1(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc008,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u1(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc00c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l2(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc010,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u2(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc014,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l3(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc018,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u3(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc01c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l4(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc020,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u4(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc024,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l5(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc028,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u5(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc02c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l6(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc030,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u6(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc034,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l7(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc038,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u7(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc03c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l8(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc040,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u8(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc044,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l9(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc048,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u9(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc04c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l10(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc050,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u10(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc054,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l11(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc058,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u11(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc05c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l12(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc060,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u12(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc064,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l13(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc068,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u13(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc06c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l14(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc070,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u14(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc074,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l15(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc078,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u15(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc07c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l16(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc080,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u16(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc084,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_l17(&self) -> crate::common::RegCore<self::DprDpRyL, 0xc088,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn dpr_dpry_u17(&self) -> crate::common::RegCore<self::DprDpRyU, 0xc08c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = "CPR"]#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l0(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd000,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u0(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd004,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l1(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd008,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u1(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd00c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l2(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd010,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u2(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd014,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l3(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd018,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u3(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd01c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l4(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd020,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u4(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd024,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l5(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd028,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u5(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd02c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l6(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd030,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u6(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd034,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l7(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd038,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u7(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd03c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l8(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd040,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u8(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd044,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_l9(&self) -> crate::common::RegCore<self::CprCpRyL, 0xd048,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn cpr_cpry_u9(&self) -> crate::common::RegCore<self::CprCpRyU, 0xd04c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = "TPS"]
#[doc = r"CPUx Temporal Protection System Control Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_con(&self) -> crate::common::RegCore<self::TpsCon, 0xe400,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Temporal Protection System Timer Register 0\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_timer0(&self)-> crate::common::RegCore<self::TpsTimer, 0xe404,0> {
    unsafe { crate::common::RegCore::new() }
}pub const fn tps_timer1(&self)-> crate::common::RegCore<self::TpsTimer, 0xe408,0> {
    unsafe { crate::common::RegCore::new() }
}pub const fn tps_timer2(&self)-> crate::common::RegCore<self::TpsTimer, 0xe40c,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = "TPS EXTIM"]
#[doc = r"CPUx Exception Entry Timer Load Value\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_extim_entry_lval(&self) -> crate::common::RegCore<self::TpsExtimEntryLval, 0xe440,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Exception Entry Timer Current Value\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_extim_entry_cval(&self) -> crate::common::RegCore<self::TpsExtimEntryCval, 0xe444,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Exception Exit  Timer Load Value\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_extim_exit_lval(&self) -> crate::common::RegCore<self::TpsExtimExitLval, 0xe448,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Exception Exit Timer Current Value\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_extim_exit_cval(&self) -> crate::common::RegCore<self::TpsExtimExitCval, 0xe44c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Exception Timer Class Enable Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_extim_class_en(&self) -> crate::common::RegCore<self::TpsExtimClassEn, 0xe450,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Exception Timer Status Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_extim_stat(&self) -> crate::common::RegCore<self::TpsExtimStat, 0xe454,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Exception Timer FCX Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn tps_extim_fcx(&self) -> crate::common::RegCore<self::TpsExtimFcx, 0xe458,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = "FPU TRAP"]
#[doc = r"CPUx Trap Control Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn fpu_trap_con(&self) -> crate::common::RegCore<self::FpuTrapCon, 0xa000,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trapping Instruction Program Counter Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn fpu_trap_pc(&self) -> crate::common::RegCore<self::FpuTrapPc, 0xa004,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trapping Instruction Opcode Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn fpu_trap_opc(&self) -> crate::common::RegCore<self::FpuTrapOpc, 0xa008,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn fpu_trap_src1(&self) -> crate::common::RegCore<self::FpuTrapSrc1, 0xa010,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn fpu_trap_src2(&self) -> crate::common::RegCore<self::FpuTrapSrc2, 0xa014,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
#[inline(always)]
pub const fn fpu_trap_src3(&self) -> crate::common::RegCore<self::FpuTrapSrc3, 0xa018,0> {
    unsafe { crate::common::RegCore::new() }
}

#[doc = "Trigger"]#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt0(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf000,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr0(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf004,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt1(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf008,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr1(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf00c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt2(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf010,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr2(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf014,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt3(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf018,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr3(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf01c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt4(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf020,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr4(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf024,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt5(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf028,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr5(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf02c,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt6(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf030,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr6(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf034,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_trievt7(&self) -> crate::common::RegCore<self::TrTRiEvt, 0xf038,0> {
    unsafe { crate::common::RegCore::new() }
}
#[doc = r"CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[inline(always)]
pub const fn tr_triadr7(&self) -> crate::common::RegCore<self::TrTRiAdr, 0xf03c,0> {
    unsafe { crate::common::RegCore::new() }
}



}
 
#[doc = "CPUx SIST Mode Access Control Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smacon {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Smacon {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Smacon {}
unsafe impl crate::common::WriteCore for Smacon {}
impl Smacon {
     
#[doc = "In Order Data Transactions   IODT"]
    #[inline(always)]
    pub fn iodt(self) -> crate::common::RegisterField<24,0x1,1,0,smacon::Iodt,smacon::Iodt,Smacon,common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,smacon::Iodt,smacon::Iodt,Smacon,common::RW>::from_register(self,0)
    }
}
pub mod smacon {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Iodt(u8);
    
    impl Iodt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Iodt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Iodt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Iodt> for u64 {
        #[inline(always)]
        fn from(value: Iodt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Iodt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Iodt {
         
#[doc = "0 Normal operation  Non dependent loads bypass stores."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 In order operation  Loads always flush preceding stores  processor store buffer disabled."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diear {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Diear {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Diear {}
unsafe impl crate::common::WriteCore for Diear {}
impl Diear {
     
#[doc = "Transaction Address   TA. Physical address being accessed by operation that encountered data integrity error."]
    #[inline(always)]
    pub fn ta(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,Diear,common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Diear,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Data Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dietr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dietr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dietr {}
unsafe impl crate::common::WriteCore for Dietr {}
impl Dietr {
     
#[doc = "Integrity Error Detected   IED"]
    #[inline(always)]
    pub fn ied(self) -> crate::common::RegisterField<0,0x1,1,0,dietr::Ied,dietr::Ied,Dietr,common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,dietr::Ied,dietr::Ied,Dietr,common::RW>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Tag Memory   IE T"]
    #[inline(always)]
    pub fn ie_t(self) -> 
    crate::common::RegisterFieldBool<1,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<1,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Cache Memory   IE C"]
    #[inline(always)]
    pub fn ie_c(self) -> 
    crate::common::RegisterFieldBool<2,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<2,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Scratchpad Memory   IE S"]
    #[inline(always)]
    pub fn ie_s(self) -> 
    crate::common::RegisterFieldBool<3,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<3,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Bus Interface   IE BI"]
    #[inline(always)]
    pub fn ie_bi(self) -> 
    crate::common::RegisterFieldBool<4,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<4,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Error Information   E INFO. If IE BS   1  Bus Master Tag ID of requesting masterIf IE C   1  Cache way."]
    #[inline(always)]
    pub fn e_info(self) -> crate::common::RegisterField<5,0x3f,1,0,u8,u8,Dietr,common::R> {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Dual Bit Error Detected   IE UNC"]
    #[inline(always)]
    pub fn ie_unc(self) -> 
    crate::common::RegisterFieldBool<11,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<11,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Safety Protection Error Detected   IE SP"]
    #[inline(always)]
    pub fn ie_sp(self) -> 
    crate::common::RegisterFieldBool<12,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<12,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Bus Slave Access Indicator   IE BS"]
    #[inline(always)]
    pub fn ie_bs(self) -> 
    crate::common::RegisterFieldBool<13,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<13,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   DLMU   IE DLMU"]
    #[inline(always)]
    pub fn ie_dlmu(self) -> 
    crate::common::RegisterFieldBool<14,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<14,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Local Pflash Bank   IE LPB"]
    #[inline(always)]
    pub fn ie_lpb(self) -> 
    crate::common::RegisterFieldBool<15,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<15,1,0,Dietr,common::R>::from_register(self,0)
    }
     
#[doc = "Memory Test Mode Violation detected   IE MTMV"]
    #[inline(always)]
    pub fn ie_mtmv(self) -> 
    crate::common::RegisterFieldBool<16,1,0,Dietr,common::R> {
        
    crate::common::RegisterFieldBool::<16,1,0,Dietr,common::R>::from_register(self,0)
    }
}
pub mod dietr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Ied(u8);
    
    impl Ied {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Ied {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Ied {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Ied> for u64 {
        #[inline(always)]
        fn from(value: Ied) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Ied {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Ied {
         
#[doc = "0 Write  Clear IED bit  re enable DIETR and DIEAR update. Read   No data integrity error condition occurred"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Write   No Effect. Read  Data integrity error condition detected. DIETR and DIEAR contents valid  further DIETR and DIEAR updates disabled.."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Program Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Piear {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Piear {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Piear {}
unsafe impl crate::common::WriteCore for Piear {}
impl Piear {
     
#[doc = "Transaction Address   TA. Physical address being accessed by operation that encountered program integrity error."]
    #[inline(always)]
    pub fn ta(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,Piear,common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Piear,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Program Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pietr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pietr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pietr {}
unsafe impl crate::common::WriteCore for Pietr {}
impl Pietr {
     
#[doc = "Integrity Error Detected   IED"]
    #[inline(always)]
    pub fn ied(self) -> crate::common::RegisterField<0,0x1,1,0,pietr::Ied,pietr::Ied,Pietr,common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,pietr::Ied,pietr::Ied,Pietr,common::RW>::from_register(self,0)
    }
     
#[doc = "Integrity Error   TAG Memory   IE T"]
    #[inline(always)]
    pub fn ie_t(self) -> 
    crate::common::RegisterFieldBool<1,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<1,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Cache Memory   IE C"]
    #[inline(always)]
    pub fn ie_c(self) -> 
    crate::common::RegisterFieldBool<2,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<2,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Scratchpad Memory   IE S"]
    #[inline(always)]
    pub fn ie_s(self) -> 
    crate::common::RegisterFieldBool<3,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<3,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Bus Interface   IE BI"]
    #[inline(always)]
    pub fn ie_bi(self) -> 
    crate::common::RegisterFieldBool<4,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<4,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Error Information   E INFO. If IE BS  1  Bus Master Tag ID of requesting masterIf IE C   1  Cache way."]
    #[inline(always)]
    pub fn e_info(self) -> crate::common::RegisterField<5,0x3f,1,0,u8,u8,Pietr,common::R> {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Uncorrectable Error Detected   IE UNC"]
    #[inline(always)]
    pub fn ie_unc(self) -> 
    crate::common::RegisterFieldBool<11,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<11,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Safety Protection Error Detected   IE SP"]
    #[inline(always)]
    pub fn ie_sp(self) -> 
    crate::common::RegisterFieldBool<12,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<12,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Bus Slave Access Indicator   IE BS"]
    #[inline(always)]
    pub fn ie_bs(self) -> 
    crate::common::RegisterFieldBool<13,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<13,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Address Phase error detected at SRI slave interface   IE ADDR"]
    #[inline(always)]
    pub fn ie_addr(self) -> 
    crate::common::RegisterFieldBool<14,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<14,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Integrity Error   Local Pflash bank   IE LPB"]
    #[inline(always)]
    pub fn ie_lpb(self) -> 
    crate::common::RegisterFieldBool<15,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<15,1,0,Pietr,common::R>::from_register(self,0)
    }
     
#[doc = "Memory Test Mode Violation detected   IE MTMV"]
    #[inline(always)]
    pub fn ie_mtmv(self) -> 
    crate::common::RegisterFieldBool<16,1,0,Pietr,common::R> {
        
    crate::common::RegisterFieldBool::<16,1,0,Pietr,common::R>::from_register(self,0)
    }
}
pub mod pietr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Ied(u8);
    
    impl Ied {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Ied {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Ied {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Ied> for u64 {
        #[inline(always)]
        fn from(value: Ied) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Ied {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Ied {
         
#[doc = "0 Write  Clear IED bit  re enable PIETR and PIEAR update. Read   No data integrity error condition occurred"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Write   No Effect. Read  Data integrity error condition detected. PIETR and PIEAR contents valid  further PIETR and PIEAR updates disabled.."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Task Address Space Identifier Register\n resetvalue={Application Reset:0x1F}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TaskAsi {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TaskAsi {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for TaskAsi {}
unsafe impl crate::common::WriteCore for TaskAsi {}
impl TaskAsi {
     
#[doc = "Address Space Identifier   ASI. The ASI register contains the Address Space Identifier of the current process."]
    #[inline(always)]
    pub fn asi(self) -> crate::common::RegisterField<0,0x1f,1,0,u8,u8,TaskAsi,common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,TaskAsi,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Data Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma0 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pma0 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pma0 {}
unsafe impl crate::common::WriteCore for Pma0 {}
impl Pma0 {
     
#[doc = "Data Access Cacheability Segments FHto 0H   DAC.  Note   segments F H  E H  D H and A H are constrained to be        non cacheable"]
    #[inline(always)]
    pub fn dac(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Pma0,common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pma0,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Code Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma1 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pma1 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pma1 {}
unsafe impl crate::common::WriteCore for Pma1 {}
impl Pma1 {
     
#[doc = "Code Access Cacheability Segments FH 0H   CAC.  Note  Segments F H  E H  C H  A H are constrained to be non cacheable"]
    #[inline(always)]
    pub fn cac(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Pma1,common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pma1,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx  Peripheral Space Identifier register\n resetvalue={Application Reset:0x0C000}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma2 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pma2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pma2 {}
unsafe impl crate::common::WriteCore for Pma2 {}
impl Pma2 {
     
#[doc = "Peripheral Space Identifier Segments FH 0H   PSI"]
    #[inline(always)]
    pub fn psi(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Pma2,common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pma2,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Compatibility Control Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compat {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Compat {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Compat {}
unsafe impl crate::common::WriteCore for Compat {}
impl Compat {
     
#[doc = "Rounding Mode Compatibility   RM"]
    #[inline(always)]
    pub fn rm(self) -> crate::common::RegisterField<3,0x1,1,0,compat::Rm,compat::Rm,Compat,common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,compat::Rm,compat::Rm,Compat,common::RW>::from_register(self,0)
    }
     
#[doc = "SYSCON Safety Protection Mode Compatibility   SP"]
    #[inline(always)]
    pub fn sp(self) -> crate::common::RegisterField<4,0x1,1,0,compat::Sp,compat::Sp,Compat,common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,compat::Sp,compat::Sp,Compat,common::RW>::from_register(self,0)
    }
}
pub mod compat {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Rm(u8);
    
    impl Rm {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Rm {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Rm {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Rm> for u64 {
        #[inline(always)]
        fn from(value: Rm) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Rm {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Rm {
         
#[doc = "0 PSW.RM not restored by RET."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 PSW.RM restored by RET  TC1.3 behavior ."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Sp(u8);
    
    impl Sp {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Sp {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Sp {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Sp> for u64 {
        #[inline(always)]
        fn from(value: Sp) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Sp {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Sp {
         
#[doc = "0 SYSCON 31 1  safety endinit protected."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 SYSCON 31 1  not safety endinit protected  TC1.3 behavior ."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Previous Context Information Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcxi {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pcxi {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pcxi {}
unsafe impl crate::common::WriteCore for Pcxi {}
impl Pcxi {
     
#[doc = "Previous Context Pointer Offset Field   PCXO. The PCXO and PCXS fields form the pointer PCX  which points to the CSA of the previous context."]
    #[inline(always)]
    pub fn pcxo(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Pcxi,common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pcxi,common::RW>::from_register(self,0)
    }
     
#[doc = "Previous Context Pointer Segment Address   PCXS. Contains the segment address portion of the PCX. This field is used in conjunction with the PCXO field."]
    #[inline(always)]
    pub fn pcxs(self) -> crate::common::RegisterField<16,0xf,1,0,u8,u8,Pcxi,common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Pcxi,common::RW>::from_register(self,0)
    }
     
#[doc = "Upper or Lower Context Tag   UL. Identifies the type of context saved. If the type does not match the type expected when a context restore operation is performed  a trap is generated."]
    #[inline(always)]
    pub fn ul(self) -> crate::common::RegisterField<20,0x1,1,0,pcxi::Ul,pcxi::Ul,Pcxi,common::RW> {
        crate::common::RegisterField::<20,0x1,1,0,pcxi::Ul,pcxi::Ul,Pcxi,common::RW>::from_register(self,0)
    }
     
#[doc = "Previous Interrupt Enable   PIE. Indicates the state of the interrupt enable bit  ICR.IE  for the interrupted task."]
    #[inline(always)]
    pub fn pie(self) -> 
    crate::common::RegisterFieldBool<21,1,0,Pcxi,common::RW> {
        
    crate::common::RegisterFieldBool::<21,1,0,Pcxi,common::RW>::from_register(self,0)
    }
     
#[doc = "Previous CPU Priority Number   PCPN. Contains the priority level number of the interrupted task."]
    #[inline(always)]
    pub fn pcpn(self) -> crate::common::RegisterField<22,0xff,1,0,u8,u8,Pcxi,common::RW> {
        crate::common::RegisterField::<22,0xff,1,0,u8,u8,Pcxi,common::RW>::from_register(self,0)
    }
}
pub mod pcxi {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Ul(u8);
    
    impl Ul {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Ul {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Ul {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Ul> for u64 {
        #[inline(always)]
        fn from(value: Ul) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Ul {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Ul {
         
#[doc = "0 Lower Context"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Upper Context"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Program Status Word\n resetvalue={Application Reset:0x0B80}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psw {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Psw {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Psw {}
unsafe impl crate::common::WriteCore for Psw {}
impl Psw {
     
#[doc = "Call Depth Counter   CDC. Consists of two variable width subfields. The first subfield consists of a string of zero or more initial 1 bits  terminated by the first 0 bit. The remaining bits form the second subfield  CDC.COUNT  which constitutes the Call Depth Count value. The count value is incremented on each Call and is decremented on a Return. 0cccccc B   6 bit counter  trap on overflow. 10ccccc B   5 bit counter  trap on overflow. 110cccc B   4 bit counter  trap on overflow. 1110ccc B   3 bit counter  trap on overflow. 11110cc B   2 bit counter  trap on overflow. 111110c B   1 bit counter  trap on overflow. 1111110 B   Trap every call  Call Trace mode . 1111111 B   Disable Call Depth Counting. When the call depth count  CDC.COUNT  overflows a trap  CDO  is generated. Setting the CDC to 1111110 B allows no bits for the counter and causes every call to be trapped. This is used for Call Depth Tracing. Setting the CDC to 1111111 B disables Call Depth Counting."]
    #[inline(always)]
    pub fn cdc(self) -> crate::common::RegisterField<0,0x7f,1,0,u8,u8,Psw,common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Psw,common::RW>::from_register(self,0)
    }
     
#[doc = "Call Depth Count Enable   CDE. Enables call depth counting  provided that the PSW.CDC mask field is not all set to 1. If PSW.CDC   1111111 B   call depth counting is disabled regardless of the setting on the PSW.CDE bit."]
    #[inline(always)]
    pub fn cde(self) -> crate::common::RegisterField<7,0x1,1,0,psw::Cde,psw::Cde,Psw,common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,psw::Cde,psw::Cde,Psw,common::RW>::from_register(self,0)
    }
     
#[doc = "Interrupt Stack Control   IS. Determines if the current execution thread is using the shared global  interrupt  stack or a user stack."]
    #[inline(always)]
    pub fn is(self) -> crate::common::RegisterField<9,0x1,1,0,psw::Is,psw::Is,Psw,common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,psw::Is,psw::Is,Psw,common::RW>::from_register(self,0)
    }
     
#[doc = "Access Privilege Level Control  I O Privilege    IO. Determines the access level to special function registers and peripheral devices."]
    #[inline(always)]
    pub fn io(self) -> crate::common::RegisterField<10,0x3,1,0,psw::Io,psw::Io,Psw,common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,psw::Io,psw::Io,Psw,common::RW>::from_register(self,0)
    }
     
#[doc = "Safe Task Identifier   S"]
    #[inline(always)]
    pub fn s(self) -> 
    crate::common::RegisterFieldBool<14,1,0,Psw,common::RW> {
        
    crate::common::RegisterFieldBool::<14,1,0,Psw,common::RW>::from_register(self,0)
    }
     
#[doc = "User Status Bits   USB. The eight most significant bits of the PSW are designated as User Status Bits. These bits may be set or cleared as side effects of instruction execution. Refer to the TriCore Architecture manual for details."]
    #[inline(always)]
    pub fn usb(self) -> crate::common::RegisterField<24,0xff,1,0,u8,u8,Psw,common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Psw,common::RW>::from_register(self,0)
    }
}
pub mod psw {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Cde(u8);
    
    impl Cde {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Cde {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Cde {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Cde> for u64 {
        #[inline(always)]
        fn from(value: Cde) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Cde {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Cde {
         
#[doc = "0 Call depth counting is temporarily disabled. It is automatically re enabled after execution of the next Call instruction."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Call depth counting is enabled."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Is(u8);
    
    impl Is {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Is {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Is {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Is> for u64 {
        #[inline(always)]
        fn from(value: Is) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Is {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Is {
         
#[doc = "0 User Stack.  If an interrupt is taken when the IS bit is 0  then the stack pointer register is loaded from the ISP register before execution starts at the first instruction of the Interrupt Service Routine  ISR ."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Shared Global Stack.  If an interrupt is taken when the PSW.IS bit is 1  then the current value of the stack pointer is used by the Interrupt Service Routine  ISR ."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Io(u8);
    
    impl Io {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Io {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Io {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Io> for u64 {
        #[inline(always)]
        fn from(value: Io) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Io {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Io {
         
#[doc = "00 User 0 Mode No peripheral access. Access to memory regions with the peripheral space attribute are prohibited and results in a PSE or MPP trap. This access level is given to tasks that need not directly access peripheral devices. Tasks at this level do not have permission to enable or disable interrupts."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 User 1 Mode Regular peripheral access. Enables access to common peripheral devices that are not specially protected  including read write access to serial I O ports  read access to timers  and access to most I O status registers. Tasks at this level may disable interrupts."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "10 Supervisor Mode Enables access to all peripheral devices. It enables read write access to core registers and protected peripheral devices. Tasks at this level may disable interrupts."]
        pub const CONST_22:Self =Self(2);
    }
}
 
#[doc = "CPUx Program Counter\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pc {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pc {}
unsafe impl crate::common::WriteCore for Pc {}
impl Pc {
     
#[doc = "Program Counter   PC"]
    #[inline(always)]
    pub fn pc(self) -> crate::common::RegisterField<1,0x7fffffff,1,0,u32,u32,Pc,common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,Pc,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx System Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Syscon {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Syscon {}
unsafe impl crate::common::WriteCore for Syscon {}
impl Syscon {
     
#[doc = "Free Context List Depleted Sticky Flag   FCDSF. This sticky bit indicates that a FCD  Free Context List Depleted  trap occurred since the bit was last cleared by software."]
    #[inline(always)]
    pub fn fcdsf(self) -> crate::common::RegisterField<0,0x1,1,0,syscon::Fcdsf,syscon::Fcdsf,Syscon,common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,syscon::Fcdsf,syscon::Fcdsf,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "Memory Protection Enable   PROTEN. Enables the memory protection system. Memory protection is controlled through the memory protection register        sets. Note  Initialize the protection register sets prior to setting        PROTEN to one."]
    #[inline(always)]
    pub fn proten(self) -> crate::common::RegisterField<1,0x1,1,0,syscon::Proten,syscon::Proten,Syscon,common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,syscon::Proten,syscon::Proten,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "Temporal Protection Enable   TPROTEN. Enable the Temporal Protection system."]
    #[inline(always)]
    pub fn tproten(self) -> crate::common::RegisterField<2,0x1,1,0,syscon::Tproten,syscon::Tproten,Syscon,common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,syscon::Tproten,syscon::Tproten,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "Initial State Interrupt   IS. of PSW.S bit in interrupt handle"]
    #[inline(always)]
    pub fn is(self) -> 
    crate::common::RegisterFieldBool<3,1,0,Syscon,common::RW> {
        
    crate::common::RegisterFieldBool::<3,1,0,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "Initial State Trap   TS. of PSW.S bit in trap handle"]
    #[inline(always)]
    pub fn ts(self) -> 
    crate::common::RegisterFieldBool<4,1,0,Syscon,common::RW> {
        
    crate::common::RegisterFieldBool::<4,1,0,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "Emulator Space Disable. Disable the Emulator Space system"]
    #[inline(always)]
    pub fn esdis(self) -> 
    crate::common::RegisterFieldBool<8,1,0,Syscon,common::RW> {
        
    crate::common::RegisterFieldBool::<8,1,0,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "User 1 Instruction execution disable   U1 IED. Disable the execution of User 1 mode instructions in User 1 IO mode. Disables User 1 ability to enable and  disable interrupts."]
    #[inline(always)]
    pub fn u1_ied(self) -> 
    crate::common::RegisterFieldBool<16,1,0,Syscon,common::RW> {
        
    crate::common::RegisterFieldBool::<16,1,0,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "User 1 Peripheral access as supervisor   U1 IOS. Allow User 1 mode tasks to access peripherals as if in Supervisor mode. Enables User 1 access to all  peripheral registers."]
    #[inline(always)]
    pub fn u1_ios(self) -> 
    crate::common::RegisterFieldBool<17,1,0,Syscon,common::RW> {
        
    crate::common::RegisterFieldBool::<17,1,0,Syscon,common::RW>::from_register(self,0)
    }
     
#[doc = "Boot Halt   BHALT"]
    #[inline(always)]
    pub fn bhalt(self) -> crate::common::RegisterField<24,0x1,1,0,syscon::Bhalt,syscon::Bhalt,Syscon,common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,syscon::Bhalt,syscon::Bhalt,Syscon,common::RW>::from_register(self,0)
    }
}
pub mod syscon {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fcdsf(u8);
    
    impl Fcdsf {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fcdsf {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fcdsf {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fcdsf> for u64 {
        #[inline(always)]
        fn from(value: Fcdsf) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fcdsf {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Fcdsf {
         
#[doc = "0 No FCD trap occurred since the last clear."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 An FCD trap occurred since the last clear."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Proten(u8);
    
    impl Proten {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Proten {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Proten {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Proten> for u64 {
        #[inline(always)]
        fn from(value: Proten) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Proten {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Proten {
         
#[doc = "0 Memory Protection is disabled."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Memory Protection is enabled."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Tproten(u8);
    
    impl Tproten {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Tproten {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Tproten {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Tproten> for u64 {
        #[inline(always)]
        fn from(value: Tproten) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Tproten {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Tproten {
         
#[doc = "0 Temporal Protection is disabled."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Temporal Protection is enabled."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bhalt(u8);
    
    impl Bhalt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bhalt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bhalt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bhalt> for u64 {
        #[inline(always)]
        fn from(value: Bhalt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bhalt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Bhalt {
         
#[doc = "0 Core is not in boot halt."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Core is in boot halt  write to 0 will exit"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Identification Register TC1.6.2P\n resetvalue={Application Reset:0x0C0C021}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuId {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for CpuId {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for CpuId {}
unsafe impl crate::common::WriteCore for CpuId {}
impl CpuId {
     
#[doc = "Revision Number   MOD REV"]
    #[inline(always)]
    pub fn mod_rev(self) -> crate::common::RegisterField<0,0xff,1,0,cpu_id::ModRev,cpu_id::ModRev,CpuId,common::R> {
        crate::common::RegisterField::<0,0xff,1,0,cpu_id::ModRev,cpu_id::ModRev,CpuId,common::R>::from_register(self,0)
    }
     
#[doc = "32 Bit Module Enable   MOD 32B"]
    #[inline(always)]
    pub fn mod_32b(self) -> crate::common::RegisterField<8,0xff,1,0,cpu_id::Mod32B,cpu_id::Mod32B,CpuId,common::R> {
        crate::common::RegisterField::<8,0xff,1,0,cpu_id::Mod32B,cpu_id::Mod32B,CpuId,common::R>::from_register(self,0)
    }
     
#[doc = "Module Identification Number   MOD"]
    #[inline(always)]
    pub fn r#mod(self) -> crate::common::RegisterField<16,0xffff,1,0,cpu_id::Mod,cpu_id::Mod,CpuId,common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,cpu_id::Mod,cpu_id::Mod,CpuId,common::R>::from_register(self,0)
    }
}
pub mod cpu_id {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ModRev(u8);
    
    impl ModRev {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ModRev {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ModRev {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ModRev> for u64 {
        #[inline(always)]
        fn from(value: ModRev) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ModRev {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl ModRev {
         
#[doc = "20 Reset value"]
        pub const CONST_3232:Self =Self(32);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Mod32B(u8);
    
    impl Mod32B {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Mod32B {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Mod32B {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Mod32B> for u64 {
        #[inline(always)]
        fn from(value: Mod32B) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Mod32B {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Mod32B {
         
#[doc = "C0 A value of C0 H in this field indicates a 32 bit module with a 32 bit module ID register."]
        pub const CONST_192192:Self =Self(192);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Mod(u8);
    
    impl Mod {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Mod {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Mod {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Mod> for u64 {
        #[inline(always)]
        fn from(value: Mod) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Mod {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Mod {
         
#[doc = "00C0 For module identification."]
        pub const CONST_192192:Self =Self(192);
    }
}
 
#[doc = "CPUx Core Identification Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoreId {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for CoreId {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for CoreId {}
unsafe impl crate::common::WriteCore for CoreId {}
impl CoreId {
     
#[doc = "Core Identification Number   CORE ID. The identification number of the core."]
    #[inline(always)]
    pub fn core_id(self) -> crate::common::RegisterField<0,0x7,1,0,u8,u8,CoreId,common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoreId,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Base Interrupt Vector Table Pointer\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biv {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Biv {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Biv {}
unsafe impl crate::common::WriteCore for Biv {}
impl Biv {
     
#[doc = "Vector Spacing Select   VSS. 0  32 byte vector spacing. 1  8 Byte vector spacing."]
    #[inline(always)]
    pub fn vss(self) -> 
    crate::common::RegisterFieldBool<0,1,0,Biv,common::RW> {
        
    crate::common::RegisterFieldBool::<0,1,0,Biv,common::RW>::from_register(self,0)
    }
     
#[doc = "Base Address of Interrupt Vector Table   BIV. The address in the BIV register must be aligned to an even byte address  halfword address . Because of the simple ORing of the left shifted priority number and the contents of the BIV register  the alignment of the base address of the vector table must be to a power of two boundary  dependent on the number of interrupt entries used. For the full range of 256 interrupt entries an alignment to an 8 KByte boundary is required. If fewer sources are used  the alignment requirements are correspondingly relaxed."]
    #[inline(always)]
    pub fn biv(self) -> crate::common::RegisterField<1,0x7fffffff,1,0,u32,u32,Biv,common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,Biv,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Base Trap Vector Table Pointer\n resetvalue={Application Reset:0x0A0000100}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btv {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Btv {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Btv {}
unsafe impl crate::common::WriteCore for Btv {}
impl Btv {
     
#[doc = "Base Address of Trap Vector Table   BTV. The address in the BTV register must be aligned to an even byte address  halfword address . Also  due to the simple ORing of the left shifted trap identification number and the contents of the BTV register  the alignment of the base address of the vector table must be to a power of two boundary. There are eight different trap classes  resulting in Trap Classes from 0 to 7. The contents of BTV should therefore be set to at least a 256 byte boundary  8 Trap Classes   8 word spacing ."]
    #[inline(always)]
    pub fn btv(self) -> crate::common::RegisterField<1,0x7fffffff,1,0,u32,u32,Btv,common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,Btv,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Interrupt Stack Pointer\n resetvalue={Application Reset:0x100}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isp {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Isp {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Isp {}
unsafe impl crate::common::WriteCore for Isp {}
impl Isp {
     
#[doc = "Interrupt Stack Pointer   ISP"]
    #[inline(always)]
    pub fn isp(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,Isp,common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Isp,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Icr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Icr {}
unsafe impl crate::common::WriteCore for Icr {}
impl Icr {
     
#[doc = "Current CPU Priority Number   CCPN. The Current CPU Priority Number  CCPN  bit field indicates the current priority level of the CPU. It is automatically updated by hardware on entry or exit of Interrupt Service Routines  ISRs  and through the execution of a BISR instruction. CCPN can also be updated through an MTCR instruction."]
    #[inline(always)]
    pub fn ccpn(self) -> crate::common::RegisterField<0,0xff,1,0,u8,u8,Icr,common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Icr,common::RW>::from_register(self,0)
    }
     
#[doc = "Global Interrupt Enable Bit   IE. The interrupt enable bit globally enables the CPU service request system. Whether a service request is delivered to the CPU depends on the individual Service Request Enable Bits  SRE  in the SRNs  and the current state of the CPU. ICR.IE is automatically updated by hardware on entry and exit of an Interrupt Service Routine  ISR . ICR.IE is cleared to 0 when an interrupt is taken  and is restored to the previous value when the ISR executes an RFE instruction to terminate itself. ICR.IE can also be updated through the execution of the ENABLE  DISABLE  MTCR  and BISR instructions."]
    #[inline(always)]
    pub fn ie(self) -> crate::common::RegisterField<15,0x1,1,0,icr::Ie,icr::Ie,Icr,common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,icr::Ie,icr::Ie,Icr,common::RW>::from_register(self,0)
    }
     
#[doc = "Pending Interrupt Priority Number   PIPN. A read only bit field that is updated by the ICU at the end of each interrupt arbitration process. It indicates the priority number of the pending service request. ICR.PIPN is set to 0 when no request is pending  and at the beginning of each new arbitration process. ..."]
    #[inline(always)]
    pub fn pipn(self) -> crate::common::RegisterField<16,0xff,1,0,icr::Pipn,icr::Pipn,Icr,common::R> {
        crate::common::RegisterField::<16,0xff,1,0,icr::Pipn,icr::Pipn,Icr,common::R>::from_register(self,0)
    }
}
pub mod icr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Ie(u8);
    
    impl Ie {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Ie {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Ie {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Ie> for u64 {
        #[inline(always)]
        fn from(value: Ie) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Ie {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Ie {
         
#[doc = "0 Interrupt system is globally disabled"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Interrupt system is globally enabled"]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Pipn(u8);
    
    impl Pipn {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Pipn {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Pipn {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Pipn> for u64 {
        #[inline(always)]
        fn from(value: Pipn) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Pipn {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Pipn {
         
#[doc = "00 No valid pending request."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 Request pending  lowest priority."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "FF Request pending  highest priority."]
        pub const CONST_255255:Self =Self(255);
    }
}
 
#[doc = "CPUx Free CSA List Head Pointer\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcx {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Fcx {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Fcx {}
unsafe impl crate::common::WriteCore for Fcx {}
impl Fcx {
     
#[doc = "FCX Offset Address Field   FCXO. The FCXO and FCXS fields together form the FCX pointer  which points to the next available CSA."]
    #[inline(always)]
    pub fn fcxo(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Fcx,common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Fcx,common::RW>::from_register(self,0)
    }
     
#[doc = "FCX Segment Address Field   FCXS. Used in conjunction with the FCXO field."]
    #[inline(always)]
    pub fn fcxs(self) -> crate::common::RegisterField<16,0xf,1,0,u8,u8,Fcx,common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Fcx,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Free CSA List Limit Pointer\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcx {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Lcx {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Lcx {}
unsafe impl crate::common::WriteCore for Lcx {}
impl Lcx {
     
#[doc = "LCX Offset Field   LCXO. The LCXO and LCXS fields form the pointer LCX  which points to the last available CSA."]
    #[inline(always)]
    pub fn lcxo(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Lcx,common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Lcx,common::RW>::from_register(self,0)
    }
     
#[doc = "LCX Segment Address   LCXS. This field is used in conjunction with the LCXO field."]
    #[inline(always)]
    pub fn lcxs(self) -> crate::common::RegisterField<16,0xf,1,0,u8,u8,Lcx,common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Lcx,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Customer ID register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CusId {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for CusId {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for CusId {}
unsafe impl crate::common::WriteCore for CusId {}
impl CusId {
     
#[doc = "Customer ID   CID. See CROSSREFERENCE for the relation between CUS ID and CORE ID for each derivative"]
    #[inline(always)]
    pub fn cid(self) -> crate::common::RegisterField<0,0x7,1,0,u8,u8,CusId,common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CusId,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Data General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dy {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dy {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dy {}
unsafe impl crate::common::WriteCore for Dy {}
impl Dy {
     
#[doc = "Data Register   DATA. General purpose registers"]
    #[inline(always)]
    pub fn data(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,Dy,common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dy,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Address General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ay {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Ay {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Ay {}
unsafe impl crate::common::WriteCore for Ay {}
impl Ay {
     
#[doc = "Address Register   ADDR. General purpose registers"]
    #[inline(always)]
    pub fn addr(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,Ay,common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ay,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe0 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Cpxe0 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Cpxe0 {}
unsafe impl crate::common::WriteCore for Cpxe0 {}
impl Cpxe0 {
     
#[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(self) -> crate::common::RegisterField<0,0x3ff,1,0,cpxe_0::XeN,cpxe_0::XeN,Cpxe0,common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_0::XeN,cpxe_0::XeN,Cpxe0,common::RW>::from_register(self,0)
    }
}
pub mod cpxe_0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct XeN(u8);
    
    impl XeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for XeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for XeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<XeN> for u64 {
        #[inline(always)]
        fn from(value: XeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for XeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl XeN {
         
#[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe1 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Cpxe1 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Cpxe1 {}
unsafe impl crate::common::WriteCore for Cpxe1 {}
impl Cpxe1 {
     
#[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(self) -> crate::common::RegisterField<0,0x3ff,1,0,cpxe_1::XeN,cpxe_1::XeN,Cpxe1,common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_1::XeN,cpxe_1::XeN,Cpxe1,common::RW>::from_register(self,0)
    }
}
pub mod cpxe_1 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct XeN(u8);
    
    impl XeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for XeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for XeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<XeN> for u64 {
        #[inline(always)]
        fn from(value: XeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for XeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl XeN {
         
#[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe2 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Cpxe2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Cpxe2 {}
unsafe impl crate::common::WriteCore for Cpxe2 {}
impl Cpxe2 {
     
#[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(self) -> crate::common::RegisterField<0,0x3ff,1,0,cpxe_2::XeN,cpxe_2::XeN,Cpxe2,common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_2::XeN,cpxe_2::XeN,Cpxe2,common::RW>::from_register(self,0)
    }
}
pub mod cpxe_2 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct XeN(u8);
    
    impl XeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for XeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for XeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<XeN> for u64 {
        #[inline(always)]
        fn from(value: XeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for XeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl XeN {
         
#[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe3 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Cpxe3 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Cpxe3 {}
unsafe impl crate::common::WriteCore for Cpxe3 {}
impl Cpxe3 {
     
#[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(self) -> crate::common::RegisterField<0,0x3ff,1,0,cpxe_3::XeN,cpxe_3::XeN,Cpxe3,common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_3::XeN,cpxe_3::XeN,Cpxe3,common::RW>::from_register(self,0)
    }
}
pub mod cpxe_3 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct XeN(u8);
    
    impl XeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for XeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for XeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<XeN> for u64 {
        #[inline(always)]
        fn from(value: XeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for XeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl XeN {
         
#[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre0 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpre0 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpre0 {}
unsafe impl crate::common::WriteCore for Dpre0 {}
impl Dpre0 {
     
#[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpre_0::ReN,dpre_0::ReN,Dpre0,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_0::ReN,dpre_0::ReN,Dpre0,common::RW>::from_register(self,0)
    }
}
pub mod dpre_0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReN(u8);
    
    impl ReN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReN> for u64 {
        #[inline(always)]
        fn from(value: ReN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl ReN {
         
#[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre1 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpre1 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpre1 {}
unsafe impl crate::common::WriteCore for Dpre1 {}
impl Dpre1 {
     
#[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpre_1::ReN,dpre_1::ReN,Dpre1,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_1::ReN,dpre_1::ReN,Dpre1,common::RW>::from_register(self,0)
    }
}
pub mod dpre_1 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReN(u8);
    
    impl ReN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReN> for u64 {
        #[inline(always)]
        fn from(value: ReN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl ReN {
         
#[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre2 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpre2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpre2 {}
unsafe impl crate::common::WriteCore for Dpre2 {}
impl Dpre2 {
     
#[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpre_2::ReN,dpre_2::ReN,Dpre2,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_2::ReN,dpre_2::ReN,Dpre2,common::RW>::from_register(self,0)
    }
}
pub mod dpre_2 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReN(u8);
    
    impl ReN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReN> for u64 {
        #[inline(always)]
        fn from(value: ReN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl ReN {
         
#[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre3 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpre3 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpre3 {}
unsafe impl crate::common::WriteCore for Dpre3 {}
impl Dpre3 {
     
#[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpre_3::ReN,dpre_3::ReN,Dpre3,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_3::ReN,dpre_3::ReN,Dpre3,common::RW>::from_register(self,0)
    }
}
pub mod dpre_3 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReN(u8);
    
    impl ReN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReN> for u64 {
        #[inline(always)]
        fn from(value: ReN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl ReN {
         
#[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe0 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpwe0 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpwe0 {}
unsafe impl crate::common::WriteCore for Dpwe0 {}
impl Dpwe0 {
     
#[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpwe_0::WeN,dpwe_0::WeN,Dpwe0,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_0::WeN,dpwe_0::WeN,Dpwe0,common::RW>::from_register(self,0)
    }
}
pub mod dpwe_0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct WeN(u8);
    
    impl WeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for WeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for WeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<WeN> for u64 {
        #[inline(always)]
        fn from(value: WeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for WeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl WeN {
         
#[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe1 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpwe1 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpwe1 {}
unsafe impl crate::common::WriteCore for Dpwe1 {}
impl Dpwe1 {
     
#[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpwe_1::WeN,dpwe_1::WeN,Dpwe1,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_1::WeN,dpwe_1::WeN,Dpwe1,common::RW>::from_register(self,0)
    }
}
pub mod dpwe_1 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct WeN(u8);
    
    impl WeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for WeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for WeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<WeN> for u64 {
        #[inline(always)]
        fn from(value: WeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for WeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl WeN {
         
#[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe2 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpwe2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpwe2 {}
unsafe impl crate::common::WriteCore for Dpwe2 {}
impl Dpwe2 {
     
#[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpwe_2::WeN,dpwe_2::WeN,Dpwe2,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_2::WeN,dpwe_2::WeN,Dpwe2,common::RW>::from_register(self,0)
    }
}
pub mod dpwe_2 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct WeN(u8);
    
    impl WeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for WeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for WeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<WeN> for u64 {
        #[inline(always)]
        fn from(value: WeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for WeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl WeN {
         
#[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe3 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpwe3 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpwe3 {}
unsafe impl crate::common::WriteCore for Dpwe3 {}
impl Dpwe3 {
     
#[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpwe_3::WeN,dpwe_3::WeN,Dpwe3,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_3::WeN,dpwe_3::WeN,Dpwe3,common::RW>::from_register(self,0)
    }
}
pub mod dpwe_3 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct WeN(u8);
    
    impl WeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for WeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for WeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<WeN> for u64 {
        #[inline(always)]
        fn from(value: WeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for WeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl WeN {
         
#[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe4 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Cpxe4 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Cpxe4 {}
unsafe impl crate::common::WriteCore for Cpxe4 {}
impl Cpxe4 {
     
#[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(self) -> crate::common::RegisterField<0,0x3ff,1,0,cpxe_4::XeN,cpxe_4::XeN,Cpxe4,common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_4::XeN,cpxe_4::XeN,Cpxe4,common::RW>::from_register(self,0)
    }
}
pub mod cpxe_4 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct XeN(u8);
    
    impl XeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for XeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for XeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<XeN> for u64 {
        #[inline(always)]
        fn from(value: XeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for XeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl XeN {
         
#[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe5 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Cpxe5 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Cpxe5 {}
unsafe impl crate::common::WriteCore for Cpxe5 {}
impl Cpxe5 {
     
#[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(self) -> crate::common::RegisterField<0,0x3ff,1,0,cpxe_5::XeN,cpxe_5::XeN,Cpxe5,common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_5::XeN,cpxe_5::XeN,Cpxe5,common::RW>::from_register(self,0)
    }
}
pub mod cpxe_5 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct XeN(u8);
    
    impl XeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for XeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for XeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<XeN> for u64 {
        #[inline(always)]
        fn from(value: XeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for XeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl XeN {
         
#[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre4 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpre4 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpre4 {}
unsafe impl crate::common::WriteCore for Dpre4 {}
impl Dpre4 {
     
#[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpre_4::ReN,dpre_4::ReN,Dpre4,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_4::ReN,dpre_4::ReN,Dpre4,common::RW>::from_register(self,0)
    }
}
pub mod dpre_4 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReN(u8);
    
    impl ReN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReN> for u64 {
        #[inline(always)]
        fn from(value: ReN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl ReN {
         
#[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre5 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpre5 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpre5 {}
unsafe impl crate::common::WriteCore for Dpre5 {}
impl Dpre5 {
     
#[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpre_5::ReN,dpre_5::ReN,Dpre5,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_5::ReN,dpre_5::ReN,Dpre5,common::RW>::from_register(self,0)
    }
}
pub mod dpre_5 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ReN(u8);
    
    impl ReN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ReN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ReN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ReN> for u64 {
        #[inline(always)]
        fn from(value: ReN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ReN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl ReN {
         
#[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe4 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpwe4 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpwe4 {}
unsafe impl crate::common::WriteCore for Dpwe4 {}
impl Dpwe4 {
     
#[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpwe_4::WeN,dpwe_4::WeN,Dpwe4,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_4::WeN,dpwe_4::WeN,Dpwe4,common::RW>::from_register(self,0)
    }
}
pub mod dpwe_4 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct WeN(u8);
    
    impl WeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for WeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for WeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<WeN> for u64 {
        #[inline(always)]
        fn from(value: WeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for WeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl WeN {
         
#[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe5 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dpwe5 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dpwe5 {}
unsafe impl crate::common::WriteCore for Dpwe5 {}
impl Dpwe5 {
     
#[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(self) -> crate::common::RegisterField<0,0x3ffff,1,0,dpwe_5::WeN,dpwe_5::WeN,Dpwe5,common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_5::WeN,dpwe_5::WeN,Dpwe5,common::RW>::from_register(self,0)
    }
}
pub mod dpwe_5 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct WeN(u8);
    
    impl WeN {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for WeN {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for WeN {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<WeN> for u64 {
        #[inline(always)]
        fn from(value: WeN) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for WeN {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl WeN {
         
#[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Counter Control\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cctrl {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Cctrl {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Cctrl {}
unsafe impl crate::common::WriteCore for Cctrl {}
impl Cctrl {
     
#[doc = "Counter Mode   CM"]
    #[inline(always)]
    pub fn cm(self) -> crate::common::RegisterField<0,0x1,1,0,cctrl::Cm,cctrl::Cm,Cctrl,common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,cctrl::Cm,cctrl::Cm,Cctrl,common::RW>::from_register(self,0)
    }
     
#[doc = "Count Enable   CE"]
    #[inline(always)]
    pub fn ce(self) -> crate::common::RegisterField<1,0x1,1,0,cctrl::Ce,cctrl::Ce,Cctrl,common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,cctrl::Ce,cctrl::Ce,Cctrl,common::RW>::from_register(self,0)
    }
     
#[doc = "M1CNT Configuration   M1"]
    #[inline(always)]
    pub fn m1(self) -> crate::common::RegisterField<2,0x7,1,0,u8,u8,Cctrl,common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,Cctrl,common::RW>::from_register(self,0)
    }
     
#[doc = "M2CNT Configuration   M2"]
    #[inline(always)]
    pub fn m2(self) -> crate::common::RegisterField<5,0x7,1,0,u8,u8,Cctrl,common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Cctrl,common::RW>::from_register(self,0)
    }
     
#[doc = "M3CNT Configuration   M3"]
    #[inline(always)]
    pub fn m3(self) -> crate::common::RegisterField<8,0x7,1,0,u8,u8,Cctrl,common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,Cctrl,common::RW>::from_register(self,0)
    }
}
pub mod cctrl {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Cm(u8);
    
    impl Cm {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Cm {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Cm {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Cm> for u64 {
        #[inline(always)]
        fn from(value: Cm) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Cm {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Cm {
         
#[doc = "0 Normal Mode."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Task Mode."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Ce(u8);
    
    impl Ce {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Ce {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Ce {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Ce> for u64 {
        #[inline(always)]
        fn from(value: Ce) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Ce {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Ce {
         
#[doc = "0 Disable the counters  CCNT  ICNT  M1CNT  M2CNT  M3CNT."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Enable the counters  CCNT  ICNT  M1CNT  M2CNT  M3CNT."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx CPU Clock Cycle Count\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccnt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Ccnt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Ccnt {}
unsafe impl crate::common::WriteCore for Ccnt {}
impl Ccnt {
     
#[doc = "Count Value   CountValue. Current Count of the CPU Clock Cycles."]
    #[inline(always)]
    pub fn countvalue(self) -> crate::common::RegisterField<0,0x7fffffff,1,0,u32,u32,Ccnt,common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32,u32,Ccnt,common::RW>::from_register(self,0)
    }
     
#[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> 
    crate::common::RegisterFieldBool<31,1,0,Ccnt,common::RW> {
        
    crate::common::RegisterFieldBool::<31,1,0,Ccnt,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Instruction Count\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icnt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Icnt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Icnt {}
unsafe impl crate::common::WriteCore for Icnt {}
impl Icnt {
     
#[doc = "Count Value   CountValue. Count of the Instructions Executed."]
    #[inline(always)]
    pub fn countvalue(self) -> crate::common::RegisterField<0,0x7fffffff,1,0,u32,u32,Icnt,common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32,u32,Icnt,common::RW>::from_register(self,0)
    }
     
#[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> 
    crate::common::RegisterFieldBool<31,1,0,Icnt,common::RW> {
        
    crate::common::RegisterFieldBool::<31,1,0,Icnt,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Multi Count Register 1\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M1Cnt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for M1Cnt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for M1Cnt {}
unsafe impl crate::common::WriteCore for M1Cnt {}
impl M1Cnt {
     
#[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(self) -> crate::common::RegisterField<0,0x7fffffff,1,0,u32,u32,M1Cnt,common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32,u32,M1Cnt,common::RW>::from_register(self,0)
    }
     
#[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> 
    crate::common::RegisterFieldBool<31,1,0,M1Cnt,common::RW> {
        
    crate::common::RegisterFieldBool::<31,1,0,M1Cnt,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Multi Count Register 2\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M2Cnt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for M2Cnt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for M2Cnt {}
unsafe impl crate::common::WriteCore for M2Cnt {}
impl M2Cnt {
     
#[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(self) -> crate::common::RegisterField<0,0x7fffffff,1,0,u32,u32,M2Cnt,common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32,u32,M2Cnt,common::RW>::from_register(self,0)
    }
     
#[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> 
    crate::common::RegisterFieldBool<31,1,0,M2Cnt,common::RW> {
        
    crate::common::RegisterFieldBool::<31,1,0,M2Cnt,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Multi Count Register 3\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M3Cnt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for M3Cnt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for M3Cnt {}
unsafe impl crate::common::WriteCore for M3Cnt {}
impl M3Cnt {
     
#[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(self) -> crate::common::RegisterField<0,0x7fffffff,1,0,u32,u32,M3Cnt,common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32,u32,M3Cnt,common::RW>::from_register(self,0)
    }
     
#[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> 
    crate::common::RegisterFieldBool<31,1,0,M3Cnt,common::RW> {
        
    crate::common::RegisterFieldBool::<31,1,0,M3Cnt,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Debug Status Register\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgsr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dbgsr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dbgsr {}
unsafe impl crate::common::WriteCore for Dbgsr {}
impl Dbgsr {
     
#[doc = "Debug Enable   DE. Determines whether the CDC is enabled or not."]
    #[inline(always)]
    pub fn de(self) -> crate::common::RegisterField<0,0x1,1,0,dbgsr::De,dbgsr::De,Dbgsr,common::R> {
        crate::common::RegisterField::<0,0x1,1,0,dbgsr::De,dbgsr::De,Dbgsr,common::R>::from_register(self,0)
    }
     
#[doc = "CPU Halt Request   Status Field   HALT. HALT can be set or cleared by software. HALT 0  is the actual Halt bit. HALT 1  is a mask bit to specify whether or not HALT 0  is to be updated on a software write. HALT 1  is always read as 0. HALT 1  must be set to 1 in order to update HALT 0  by software  R  read  W  write ."]
    #[inline(always)]
    pub fn halt(self) -> crate::common::RegisterField<1,0x3,1,0,dbgsr::Halt,dbgsr::Halt,Dbgsr,common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,dbgsr::Halt,dbgsr::Halt,Dbgsr,common::RW>::from_register(self,0)
    }
     
#[doc = "Suspend in Halt   SIH. State of the Suspend In signal."]
    #[inline(always)]
    pub fn sih(self) -> crate::common::RegisterField<3,0x1,1,0,dbgsr::Sih,dbgsr::Sih,Dbgsr,common::R> {
        crate::common::RegisterField::<3,0x1,1,0,dbgsr::Sih,dbgsr::Sih,Dbgsr,common::R>::from_register(self,0)
    }
     
#[doc = "Current State of the Core Suspend Out Signal   SUSP"]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterField<4,0x1,1,0,dbgsr::Susp,dbgsr::Susp,Dbgsr,common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,dbgsr::Susp,dbgsr::Susp,Dbgsr,common::RW>::from_register(self,0)
    }
     
#[doc = "Previous State of Core Suspend Out Signal   PREVSUSP. Updated when a Debug Event causes a hardware update of DBGSR.SUSP. This field is not updated for writes to DBGSR.SUSP."]
    #[inline(always)]
    pub fn prevsusp(self) -> crate::common::RegisterField<6,0x1,1,0,dbgsr::Prevsusp,dbgsr::Prevsusp,Dbgsr,common::R> {
        crate::common::RegisterField::<6,0x1,1,0,dbgsr::Prevsusp,dbgsr::Prevsusp,Dbgsr,common::R>::from_register(self,0)
    }
     
#[doc = "Posted Event   PEVT"]
    #[inline(always)]
    pub fn pevt(self) -> crate::common::RegisterField<7,0x1,1,0,dbgsr::Pevt,dbgsr::Pevt,Dbgsr,common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,dbgsr::Pevt,dbgsr::Pevt,Dbgsr,common::RW>::from_register(self,0)
    }
}
pub mod dbgsr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct De(u8);
    
    impl De {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for De {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for De {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<De> for u64 {
        #[inline(always)]
        fn from(value: De) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for De {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl De {
         
#[doc = "0 The CDC is disabled."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 The CDC is enabled."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Halt(u8);
    
    impl Halt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Halt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Halt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Halt> for u64 {
        #[inline(always)]
        fn from(value: Halt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Halt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Halt {
         
#[doc = "00 R  CPU running.  W  HALT 0  unchanged."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 R  CPU halted.  W  HALT 0  unchanged."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "10 R  Not Applicable. W  reset HALT 0 ."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "11 R  Not Applicable. W  If DBGSR.DE    1  The CDC is enabled   set HALT 0 . If DBGSR.DE    0  The CDC is not enabled   HALT 0  is left unchanged."]
        pub const CONST_33:Self =Self(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Sih(u8);
    
    impl Sih {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Sih {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Sih {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Sih> for u64 {
        #[inline(always)]
        fn from(value: Sih) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Sih {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Sih {
         
#[doc = "0 The Suspend In signal is negated. The CPU is not in Halt Mode   except when the Halt mechanism is set following a Debug Event or a write to DBGSR.HALT ."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 The Suspend In signal is asserted. The CPU is in Halt Mode."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Susp(u8);
    
    impl Susp {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Susp {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Susp {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Susp> for u64 {
        #[inline(always)]
        fn from(value: Susp) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Susp {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Susp {
         
#[doc = "0 Core suspend out inactive."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Core suspend out active."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Prevsusp(u8);
    
    impl Prevsusp {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Prevsusp {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Prevsusp {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Prevsusp> for u64 {
        #[inline(always)]
        fn from(value: Prevsusp) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Prevsusp {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Prevsusp {
         
#[doc = "0 Previous core suspend out inactive."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Previous core suspend out active."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Pevt(u8);
    
    impl Pevt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Pevt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Pevt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Pevt> for u64 {
        #[inline(always)]
        fn from(value: Pevt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Pevt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Pevt {
         
#[doc = "0 No posted event."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Posted event."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx External Event Register\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exevt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Exevt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Exevt {}
unsafe impl crate::common::WriteCore for Exevt {}
impl Exevt {
     
#[doc = "Event Associated   EVTA. Specifies the Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(self) -> crate::common::RegisterField<0,0x7,1,0,exevt::Evta,exevt::Evta,Exevt,common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,exevt::Evta,exevt::Evta,Exevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(self) -> crate::common::RegisterField<3,0x1,1,0,exevt::Bbm,exevt::Bbm,Exevt,common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,exevt::Bbm,exevt::Bbm,Exevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(self) -> crate::common::RegisterField<4,0x1,1,0,exevt::Bod,exevt::Bod,Exevt,common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,exevt::Bod,exevt::Bod,Exevt,common::RW>::from_register(self,0)
    }
     
#[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
    #[inline(always)]
    pub fn susp(self) -> 
    crate::common::RegisterFieldBool<5,1,0,Exevt,common::RW> {
        
    crate::common::RegisterFieldBool::<5,1,0,Exevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(self) -> crate::common::RegisterField<6,0x3,1,0,exevt::Cnt,exevt::Cnt,Exevt,common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,exevt::Cnt,exevt::Cnt,Exevt,common::RW>::from_register(self,0)
    }
}
pub mod exevt {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Evta(u8);
    
    impl Evta {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Evta {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Evta {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Evta> for u64 {
        #[inline(always)]
        fn from(value: Evta) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Evta {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Evta {
         
#[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "011 BOD 0  Breakpoint trap and pulse. BRKOUT Signal. BOD 1  Breakpoint trap."]
        pub const CONST_33:Self =Self(3);
         
#[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
        pub const CONST_44:Self =Self(4);
         
#[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
        pub const CONST_55:Self =Self(5);
         
#[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
        pub const CONST_66:Self =Self(6);
         
#[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
        pub const CONST_77:Self =Self(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bbm(u8);
    
    impl Bbm {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bbm {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bbm {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bbm> for u64 {
        #[inline(always)]
        fn from(value: Bbm) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bbm {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Bbm {
         
#[doc = "0 Break after make  BAM ."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Break before make  BBM ."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bod(u8);
    
    impl Bod {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bod {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bod {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bod> for u64 {
        #[inline(always)]
        fn from(value: Bod) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bod {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Bod {
         
#[doc = "0 BRKOUT signal asserted according to the Debug Action specified in the EVTA field."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Cnt(u8);
    
    impl Cnt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Cnt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Cnt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Cnt> for u64 {
        #[inline(always)]
        fn from(value: Cnt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Cnt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Cnt {
         
#[doc = "00 No change."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 Start the performance counters."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "10 Stop the performance counters."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
        pub const CONST_33:Self =Self(3);
    }
}
 
#[doc = "CPUx Core Register Access Event\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crevt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Crevt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Crevt {}
unsafe impl crate::common::WriteCore for Crevt {}
impl Crevt {
     
#[doc = "Event Associated   EVTA. Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(self) -> crate::common::RegisterField<0,0x7,1,0,crevt::Evta,crevt::Evta,Crevt,common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,crevt::Evta,crevt::Evta,Crevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(self) -> crate::common::RegisterField<3,0x1,1,0,crevt::Bbm,crevt::Bbm,Crevt,common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,crevt::Bbm,crevt::Bbm,Crevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(self) -> crate::common::RegisterField<4,0x1,1,0,crevt::Bod,crevt::Bod,Crevt,common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,crevt::Bod,crevt::Bod,Crevt,common::RW>::from_register(self,0)
    }
     
#[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
    #[inline(always)]
    pub fn susp(self) -> 
    crate::common::RegisterFieldBool<5,1,0,Crevt,common::RW> {
        
    crate::common::RegisterFieldBool::<5,1,0,Crevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(self) -> crate::common::RegisterField<6,0x3,1,0,crevt::Cnt,crevt::Cnt,Crevt,common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,crevt::Cnt,crevt::Cnt,Crevt,common::RW>::from_register(self,0)
    }
}
pub mod crevt {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Evta(u8);
    
    impl Evta {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Evta {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Evta {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Evta> for u64 {
        #[inline(always)]
        fn from(value: Evta) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Evta {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Evta {
         
#[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "011 BOD 0  Breakpoint trap and pulse BRKOUT Signal. BOD 1  Breakpoint trap."]
        pub const CONST_33:Self =Self(3);
         
#[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
        pub const CONST_44:Self =Self(4);
         
#[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
        pub const CONST_55:Self =Self(5);
         
#[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
        pub const CONST_66:Self =Self(6);
         
#[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
        pub const CONST_77:Self =Self(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bbm(u8);
    
    impl Bbm {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bbm {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bbm {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bbm> for u64 {
        #[inline(always)]
        fn from(value: Bbm) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bbm {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Bbm {
         
#[doc = "0 Break after make  BAM ."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Break before make  BBM ."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bod(u8);
    
    impl Bod {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bod {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bod {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bod> for u64 {
        #[inline(always)]
        fn from(value: Bod) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bod {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Bod {
         
#[doc = "0 BRKOUT signal asserted according to the action specified in the EVTA field."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Cnt(u8);
    
    impl Cnt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Cnt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Cnt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Cnt> for u64 {
        #[inline(always)]
        fn from(value: Cnt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Cnt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Cnt {
         
#[doc = "00 No change."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 Start the performance counters."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "10 Stop the performance counters."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
        pub const CONST_33:Self =Self(3);
    }
}
 
#[doc = "CPUx Software Debug Event\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swevt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Swevt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Swevt {}
unsafe impl crate::common::WriteCore for Swevt {}
impl Swevt {
     
#[doc = "Event Associated   EVTA. Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(self) -> crate::common::RegisterField<0,0x7,1,0,swevt::Evta,swevt::Evta,Swevt,common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,swevt::Evta,swevt::Evta,Swevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(self) -> crate::common::RegisterField<3,0x1,1,0,swevt::Bbm,swevt::Bbm,Swevt,common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,swevt::Bbm,swevt::Bbm,Swevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(self) -> crate::common::RegisterField<4,0x1,1,0,swevt::Bod,swevt::Bod,Swevt,common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,swevt::Bod,swevt::Bod,Swevt,common::RW>::from_register(self,0)
    }
     
#[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the event is raised."]
    #[inline(always)]
    pub fn susp(self) -> 
    crate::common::RegisterFieldBool<5,1,0,Swevt,common::RW> {
        
    crate::common::RegisterFieldBool::<5,1,0,Swevt,common::RW>::from_register(self,0)
    }
     
#[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(self) -> crate::common::RegisterField<6,0x3,1,0,swevt::Cnt,swevt::Cnt,Swevt,common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,swevt::Cnt,swevt::Cnt,Swevt,common::RW>::from_register(self,0)
    }
}
pub mod swevt {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Evta(u8);
    
    impl Evta {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Evta {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Evta {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Evta> for u64 {
        #[inline(always)]
        fn from(value: Evta) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Evta {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Evta {
         
#[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "011 BOD 0  Breakpoint trap and pulse BRKOUT Signal. BOD 1  Breakpoint trap."]
        pub const CONST_33:Self =Self(3);
         
#[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
        pub const CONST_44:Self =Self(4);
         
#[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
        pub const CONST_55:Self =Self(5);
         
#[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
        pub const CONST_66:Self =Self(6);
         
#[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
        pub const CONST_77:Self =Self(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bbm(u8);
    
    impl Bbm {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bbm {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bbm {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bbm> for u64 {
        #[inline(always)]
        fn from(value: Bbm) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bbm {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Bbm {
         
#[doc = "0 Break after make  BAM ."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Break before make  BBM ."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bod(u8);
    
    impl Bod {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bod {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bod {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bod> for u64 {
        #[inline(always)]
        fn from(value: Bod) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bod {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Bod {
         
#[doc = "0 BRKOUT signal asserted according to the action specified in the EVTA field."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Cnt(u8);
    
    impl Cnt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Cnt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Cnt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Cnt> for u64 {
        #[inline(always)]
        fn from(value: Cnt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Cnt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Cnt {
         
#[doc = "00 No change."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 Start the performance counters."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "10 Stop the performance counters."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
        pub const CONST_33:Self =Self(3);
    }
}
 
#[doc = "CPUx TriggerAddressx\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigAcc {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TrigAcc {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for TrigAcc {}
unsafe impl crate::common::WriteCore for TrigAcc {}
impl TrigAcc {
     
#[doc = "Trigger 0   T0. active since last cleared"]
    #[inline(always)]
    pub fn t0(self) -> 
    crate::common::RegisterFieldBool<0,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<0,1,0,TrigAcc,common::R>::from_register(self,0)
    }
     
#[doc = "Trigger 1   T1. active since last cleared"]
    #[inline(always)]
    pub fn t1(self) -> 
    crate::common::RegisterFieldBool<1,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<1,1,0,TrigAcc,common::R>::from_register(self,0)
    }
     
#[doc = "Trigger 2   T2. active since last cleared"]
    #[inline(always)]
    pub fn t2(self) -> 
    crate::common::RegisterFieldBool<2,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<2,1,0,TrigAcc,common::R>::from_register(self,0)
    }
     
#[doc = "Trigger 3   T3. active since last cleared"]
    #[inline(always)]
    pub fn t3(self) -> 
    crate::common::RegisterFieldBool<3,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<3,1,0,TrigAcc,common::R>::from_register(self,0)
    }
     
#[doc = "Trigger 4   T4. active since last cleared"]
    #[inline(always)]
    pub fn t4(self) -> 
    crate::common::RegisterFieldBool<4,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<4,1,0,TrigAcc,common::R>::from_register(self,0)
    }
     
#[doc = "Trigger 5   T5. active since last cleared"]
    #[inline(always)]
    pub fn t5(self) -> 
    crate::common::RegisterFieldBool<5,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<5,1,0,TrigAcc,common::R>::from_register(self,0)
    }
     
#[doc = "Trigger 6   T6. active since last cleared"]
    #[inline(always)]
    pub fn t6(self) -> 
    crate::common::RegisterFieldBool<6,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<6,1,0,TrigAcc,common::R>::from_register(self,0)
    }
     
#[doc = "Trigger 7   T7. active since last cleared"]
    #[inline(always)]
    pub fn t7(self) -> 
    crate::common::RegisterFieldBool<7,1,0,TrigAcc,common::R> {
        
    crate::common::RegisterFieldBool::<7,1,0,TrigAcc,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Debug Monitor Start Address\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dms {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dms {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dms {}
unsafe impl crate::common::WriteCore for Dms {}
impl Dms {
     
#[doc = "Debug Monitor Start Address   DMSValue. The address at which monitor code execution begins when a breakpoint trap is taken."]
    #[inline(always)]
    pub fn dmsvalue(self) -> crate::common::RegisterField<1,0x7fffffff,1,0,u32,u32,Dms,common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32,u32,Dms,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Debug Context Save Area Pointer\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcx {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dcx {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dcx {}
unsafe impl crate::common::WriteCore for Dcx {}
impl Dcx {
     
#[doc = "Debug Context Save Area Pointer   DCXValue. Address where the debug context is stored following a breakpoint trap."]
    #[inline(always)]
    pub fn dcxvalue(self) -> crate::common::RegisterField<6,0x3ffffff,1,0,u32,u32,Dcx,common::RW> {
        crate::common::RegisterField::<6,0x3ffffff,1,0,u32,u32,Dcx,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Debug Trap Control Register\n resetvalue={Application Reset:0x1}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgtcr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dbgtcr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dbgtcr {}
unsafe impl crate::common::WriteCore for Dbgtcr {}
impl Dbgtcr {
     
#[doc = "Debug Trap Active Bit   DTA. A breakpoint trap may only be taken in the condition DTA    0. Taking a breakpoint trap sets the DTA bit to one. Further breakpoint traps are therefore disabled until such time as the breakpoint trap handler clears the DTA bit or until the breakpoint trap handler terminates with a RFM."]
    #[inline(always)]
    pub fn dta(self) -> crate::common::RegisterField<0,0x1,1,0,dbgtcr::Dta,dbgtcr::Dta,Dbgtcr,common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,dbgtcr::Dta,dbgtcr::Dta,Dbgtcr,common::RW>::from_register(self,0)
    }
}
pub mod dbgtcr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Dta(u8);
    
    impl Dta {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Dta {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Dta {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Dta> for u64 {
        #[inline(always)]
        fn from(value: Dta) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Dta {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Dta {
         
#[doc = "0 No breakpoint trap is active."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 A breakpoint Trap is active"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx SRI Error Generation Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segen {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Segen {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Segen {}
unsafe impl crate::common::WriteCore for Segen {}
impl Segen {
     
#[doc = "Address ECC Bit Flip   ADFLIP. SRI address ECC Bits to be flipped on the next read or write transaction from the DMI when enabled by AE."]
    #[inline(always)]
    pub fn adflip(self) -> crate::common::RegisterField<0,0xff,1,0,segen::Adflip,segen::Adflip,Segen,common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,segen::Adflip,segen::Adflip,Segen,common::RW>::from_register(self,0)
    }
     
#[doc = "Type of error   ADTYPE"]
    #[inline(always)]
    pub fn adtype(self) -> crate::common::RegisterField<8,0x3,1,0,segen::Adtype,segen::Adtype,Segen,common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,segen::Adtype,segen::Adtype,Segen,common::RW>::from_register(self,0)
    }
     
#[doc = "Activate Error Enable   AE. Enabled the selective inverting of SRI ECC packet bits defined by ADFLIP. This bit will be cleared by hardware after the next SRI read or write transaction from the DMI."]
    #[inline(always)]
    pub fn ae(self) -> crate::common::RegisterField<31,0x1,1,0,segen::Ae,segen::Ae,Segen,common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,segen::Ae,segen::Ae,Segen,common::RW>::from_register(self,0)
    }
}
pub mod segen {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Adflip(u8);
    
    impl Adflip {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Adflip {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Adflip {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Adflip> for u64 {
        #[inline(always)]
        fn from(value: Adflip) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Adflip {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Adflip {
         
#[doc = "0 No Flip"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Flip"]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Adtype(u8);
    
    impl Adtype {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Adtype {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Adtype {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Adtype> for u64 {
        #[inline(always)]
        fn from(value: Adtype) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Adtype {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Adtype {
         
#[doc = "00 Data Master Address Phase"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 Data Master Write Data"]
        pub const CONST_11:Self =Self(1);
         
#[doc = "10 Data Slave Read Data"]
        pub const CONST_22:Self =Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Ae(u8);
    
    impl Ae {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Ae {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Ae {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Ae> for u64 {
        #[inline(always)]
        fn from(value: Ae) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Ae {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Ae {
         
#[doc = "0 Not Enabled"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Enabled"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Data Control Register 2\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcon2 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dcon2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dcon2 {}
unsafe impl crate::common::WriteCore for Dcon2 {}
impl Dcon2 {
     
#[doc = "Data Cache Size   DCACHE SZE. In KBytes"]
    #[inline(always)]
    pub fn dcache_sze(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Dcon2,common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dcon2,common::R>::from_register(self,0)
    }
     
#[doc = "Data Scratch Size   DSCRATCH SZE. In KBytes"]
    #[inline(always)]
    pub fn dscratch_sze(self) -> crate::common::RegisterField<16,0xffff,1,0,u16,u16,Dcon2,common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dcon2,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Data Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dstr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dstr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dstr {}
unsafe impl crate::common::WriteCore for Dstr {}
impl Dstr {
     
#[doc = "Scratch Range Error   SRE. A scratch Range Error occurs whenever an access to the data scratch is outside the range of the SRAM."]
    #[inline(always)]
    pub fn sre(self) -> 
    crate::common::RegisterFieldBool<0,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<0,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Global Address Error   GAE. Load or store to local code scratch address outside of the lower 1MByte."]
    #[inline(always)]
    pub fn gae(self) -> 
    crate::common::RegisterFieldBool<1,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<1,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Load Bus Error   LBE. A Load Bus Error will be set whenever the SRI flags an error due a load from external memory."]
    #[inline(always)]
    pub fn lbe(self) -> 
    crate::common::RegisterFieldBool<2,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<2,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Local DLMU Range Error   DRE. A DLMU Range Error occurs whenever an access to the local DLMU region is outside the physically implemented memory."]
    #[inline(always)]
    pub fn dre(self) -> 
    crate::common::RegisterFieldBool<3,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<3,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Cache Refill Error   CRE. A Cache Refill Error will be set whenever the SRI flags an error due a cache refill from external memory."]
    #[inline(always)]
    pub fn cre(self) -> 
    crate::common::RegisterFieldBool<6,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<6,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "DTAG MSIST Error   DTME. Access to memory mapped DTAG range outside of physically implemented memory."]
    #[inline(always)]
    pub fn dtme(self) -> 
    crate::common::RegisterFieldBool<14,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<14,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Load Overlay Error   LOE. Load to invalid overlay address."]
    #[inline(always)]
    pub fn loe(self) -> 
    crate::common::RegisterFieldBool<15,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<15,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Segment Difference Error   SDE. Load or store access where base address is in different segment to access address."]
    #[inline(always)]
    pub fn sde(self) -> 
    crate::common::RegisterFieldBool<16,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<16,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Segment Crossing Error   SCE. Load or store access across segment boundary."]
    #[inline(always)]
    pub fn sce(self) -> 
    crate::common::RegisterFieldBool<17,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<17,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "CSFR Access Error   CAC. Load or store to local CSFR space."]
    #[inline(always)]
    pub fn cac(self) -> 
    crate::common::RegisterFieldBool<18,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<18,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Memory Protection Error   MPE. Data access violating memory protection."]
    #[inline(always)]
    pub fn mpe(self) -> 
    crate::common::RegisterFieldBool<19,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<19,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Context Location Error   CLE. Context operation to invalid location."]
    #[inline(always)]
    pub fn cle(self) -> 
    crate::common::RegisterFieldBool<20,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<20,1,0,Dstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Alignment Error   ALN. Data access causing alignment error."]
    #[inline(always)]
    pub fn aln(self) -> 
    crate::common::RegisterFieldBool<24,1,0,Dstr,common::RW> {
        
    crate::common::RegisterFieldBool::<24,1,0,Dstr,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Data Asynchronous Trap Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Datr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Datr {}
unsafe impl crate::common::WriteCore for Datr {}
impl Datr {
     
#[doc = "Store Bus Error   SBE"]
    #[inline(always)]
    pub fn sbe(self) -> 
    crate::common::RegisterFieldBool<3,1,0,Datr,common::RW> {
        
    crate::common::RegisterFieldBool::<3,1,0,Datr,common::RW>::from_register(self,0)
    }
     
#[doc = "Cache Writeback Error   CWE"]
    #[inline(always)]
    pub fn cwe(self) -> 
    crate::common::RegisterFieldBool<9,1,0,Datr,common::RW> {
        
    crate::common::RegisterFieldBool::<9,1,0,Datr,common::RW>::from_register(self,0)
    }
     
#[doc = "Cache Flush Error   CFE"]
    #[inline(always)]
    pub fn cfe(self) -> 
    crate::common::RegisterFieldBool<10,1,0,Datr,common::RW> {
        
    crate::common::RegisterFieldBool::<10,1,0,Datr,common::RW>::from_register(self,0)
    }
     
#[doc = "Store Overlay Error   SOE"]
    #[inline(always)]
    pub fn soe(self) -> 
    crate::common::RegisterFieldBool<14,1,0,Datr,common::RW> {
        
    crate::common::RegisterFieldBool::<14,1,0,Datr,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Data Error Address Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deadd {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Deadd {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Deadd {}
unsafe impl crate::common::WriteCore for Deadd {}
impl Deadd {
     
#[doc = "Error Address   ERROR ADDRESS"]
    #[inline(always)]
    pub fn error_address(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,Deadd,common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Deadd,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Data Memory Control Register\n resetvalue={Application Reset:0x2}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcon0 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Dcon0 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Dcon0 {}
unsafe impl crate::common::WriteCore for Dcon0 {}
impl Dcon0 {
     
#[doc = "Data Cache Bypass   DCBYP"]
    #[inline(always)]
    pub fn dcbyp(self) -> crate::common::RegisterField<1,0x1,1,0,dcon0::Dcbyp,dcon0::Dcbyp,Dcon0,common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,dcon0::Dcbyp,dcon0::Dcbyp,Dcon0,common::RW>::from_register(self,0)
    }
}
pub mod dcon0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Dcbyp(u8);
    
    impl Dcbyp {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Dcbyp {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Dcbyp {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Dcbyp> for u64 {
        #[inline(always)]
        fn from(value: Dcbyp) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Dcbyp {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Dcbyp {
         
#[doc = "0 DCache   DRB enabled"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 DCache   DRB Bypass  disabled"]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Program Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pstr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pstr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pstr {}
unsafe impl crate::common::WriteCore for Pstr {}
impl Pstr {
     
#[doc = "Fetch Range Error   FRE. A Fetch Range Error occurs whenever an access to the Program Scratch is outside the range of the SRAM."]
    #[inline(always)]
    pub fn fre(self) -> 
    crate::common::RegisterFieldBool<0,1,0,Pstr,common::RW> {
        
    crate::common::RegisterFieldBool::<0,1,0,Pstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Fetch Bus Error   FBE. A Fetch bus error will be set whenever the SRI flags an error due a fetch from external memory. This will be set for both direct fetches from the bus and for cache refills."]
    #[inline(always)]
    pub fn fbe(self) -> 
    crate::common::RegisterFieldBool<2,1,0,Pstr,common::RW> {
        
    crate::common::RegisterFieldBool::<2,1,0,Pstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Fetch Peripheral Error   FPE. A Fetch peripheral error will be flagged whenever a fetch is attempted to peripheral space."]
    #[inline(always)]
    pub fn fpe(self) -> 
    crate::common::RegisterFieldBool<12,1,0,Pstr,common::RW> {
        
    crate::common::RegisterFieldBool::<12,1,0,Pstr,common::RW>::from_register(self,0)
    }
     
#[doc = "Fetch MSIST Error   FME. During SIST mode  a fetch from the PTAG will cause a PSE trap to occur."]
    #[inline(always)]
    pub fn fme(self) -> 
    crate::common::RegisterFieldBool<14,1,0,Pstr,common::RW> {
        
    crate::common::RegisterFieldBool::<14,1,0,Pstr,common::RW>::from_register(self,0)
    }
}

 
#[doc = "CPUx Program Control 1\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon1 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pcon1 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pcon1 {}
unsafe impl crate::common::WriteCore for Pcon1 {}
impl Pcon1 {
     
#[doc = "Program Cache Invalidate   PCINV"]
    #[inline(always)]
    pub fn pcinv(self) -> crate::common::RegisterField<0,0x1,1,0,pcon1::Pcinv,pcon1::Pcinv,Pcon1,common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,pcon1::Pcinv,pcon1::Pcinv,Pcon1,common::RW>::from_register(self,0)
    }
     
#[doc = "Program Buffer Invalidate   PBINV. Write Operation  This field returns 0 when read."]
    #[inline(always)]
    pub fn pbinv(self) -> crate::common::RegisterField<1,0x1,1,0,pcon1::Pbinv,pcon1::Pbinv,Pcon1,common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,pcon1::Pbinv,pcon1::Pbinv,Pcon1,common::RW>::from_register(self,0)
    }
}
pub mod pcon1 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Pcinv(u8);
    
    impl Pcinv {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Pcinv {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Pcinv {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Pcinv> for u64 {
        #[inline(always)]
        fn from(value: Pcinv) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Pcinv {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Pcinv {
         
#[doc = "0 Write  No effect  normal instruction cache operation. Read   Normal operation  instruction cache available"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Write   Initiate invalidation of entire instruction cache. Read  Instruction cache invalidation in progress. Instruction cache unavailable."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Pbinv(u8);
    
    impl Pbinv {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Pbinv {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Pbinv {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Pbinv> for u64 {
        #[inline(always)]
        fn from(value: Pbinv) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Pbinv {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Pbinv {
         
#[doc = "0 Write  No effect. Normal program line buffer operation."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Write  Invalidate the program line buffer."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Program Control 2\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon2 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pcon2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pcon2 {}
unsafe impl crate::common::WriteCore for Pcon2 {}
impl Pcon2 {
     
#[doc = "Program Cache Size  ICACHE  in KBytes   PCACHE SZE. In KBytes"]
    #[inline(always)]
    pub fn pcache_sze(self) -> crate::common::RegisterField<0,0xffff,1,0,u16,u16,Pcon2,common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pcon2,common::R>::from_register(self,0)
    }
     
#[doc = "Program Scratch Size in KBytes   PSCRATCH SZE. In KBytes"]
    #[inline(always)]
    pub fn pscratch_sze(self) -> crate::common::RegisterField<16,0xffff,1,0,u16,u16,Pcon2,common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Pcon2,common::R>::from_register(self,0)
    }
}

 
#[doc = "CPUx Program Control 0\n resetvalue={Application Reset:0x2}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon0 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for Pcon0 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}




unsafe impl crate::common::ReadCore for Pcon0 {}
unsafe impl crate::common::WriteCore for Pcon0 {}
impl Pcon0 {
     
#[doc = "Program Cache Bypass   PCBYP"]
    #[inline(always)]
    pub fn pcbyp(self) -> crate::common::RegisterField<1,0x1,1,0,pcon0::Pcbyp,pcon0::Pcbyp,Pcon0,common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,pcon0::Pcbyp,pcon0::Pcbyp,Pcon0,common::RW>::from_register(self,0)
    }
}
pub mod pcon0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Pcbyp(u8);
    
    impl Pcbyp {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Pcbyp {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Pcbyp {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Pcbyp> for u64 {
        #[inline(always)]
        fn from(value: Pcbyp) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Pcbyp {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }
    impl Pcbyp {
         
#[doc = "0 Cache enabled"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Cache bypass  disabled"]
        pub const CONST_11:Self =Self(1);
    }
}

 
#[doc = "CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DprDpRyL {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for DprDpRyL {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct DprDpRyLT;
unsafe impl crate::common::AsPtr for DprDpRyLT {}
impl crate::common::Reg<DprDpRyL> for DprDpRyLT {}


unsafe impl crate::common::Read<DprDpRyL> for DprDpRyLT {}
unsafe impl crate::common::Write<DprDpRyL> for DprDpRyLT {}
impl DprDpRyL {
     
#[doc = "DPRy Lower Boundary Address   LOWBND"]
    #[inline(always)]
    pub fn lowbnd(self) -> crate::common::RegisterField<3,0x1fffffff,1,0,u32,u32,DprDpRyL,common::RW> {
        crate::common::RegisterField::<3,0x1fffffff,1,0,u32,u32,DprDpRyL,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DprDpRyL> for DprDpRyLT {
    #[inline(always)]
    fn reset_value(&self) -> DprDpRyL {
        DprDpRyL::new(0)
    }
}

 
#[doc = "CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DprDpRyU {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for DprDpRyU {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct DprDpRyUT;
unsafe impl crate::common::AsPtr for DprDpRyUT {}
impl crate::common::Reg<DprDpRyU> for DprDpRyUT {}


unsafe impl crate::common::Read<DprDpRyU> for DprDpRyUT {}
unsafe impl crate::common::Write<DprDpRyU> for DprDpRyUT {}
impl DprDpRyU {
     
#[doc = "DPRy Upper Boundary Address   UPPBND"]
    #[inline(always)]
    pub fn uppbnd(self) -> crate::common::RegisterField<3,0x1fffffff,1,0,u32,u32,DprDpRyU,common::RW> {
        crate::common::RegisterField::<3,0x1fffffff,1,0,u32,u32,DprDpRyU,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DprDpRyU> for DprDpRyUT {
    #[inline(always)]
    fn reset_value(&self) -> DprDpRyU {
        DprDpRyU::new(0)
    }
}


 
#[doc = "CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CprCpRyL {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for CprCpRyL {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct CprCpRyLT;
unsafe impl crate::common::AsPtr for CprCpRyLT {}
impl crate::common::Reg<CprCpRyL> for CprCpRyLT {}


unsafe impl crate::common::Read<CprCpRyL> for CprCpRyLT {}
unsafe impl crate::common::Write<CprCpRyL> for CprCpRyLT {}
impl CprCpRyL {
     
#[doc = "CPRy Lower Boundary Address   LOWBND"]
    #[inline(always)]
    pub fn lowbnd(self) -> crate::common::RegisterField<5,0x7ffffff,1,0,u32,u32,CprCpRyL,common::RW> {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,CprCpRyL,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<CprCpRyL> for CprCpRyLT {
    #[inline(always)]
    fn reset_value(&self) -> CprCpRyL {
        CprCpRyL::new(0)
    }
}

 
#[doc = "CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CprCpRyU {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for CprCpRyU {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct CprCpRyUT;
unsafe impl crate::common::AsPtr for CprCpRyUT {}
impl crate::common::Reg<CprCpRyU> for CprCpRyUT {}


unsafe impl crate::common::Read<CprCpRyU> for CprCpRyUT {}
unsafe impl crate::common::Write<CprCpRyU> for CprCpRyUT {}
impl CprCpRyU {
     
#[doc = "CPR0 m Upper Boundary Address   UPPBND"]
    #[inline(always)]
    pub fn uppbnd(self) -> crate::common::RegisterField<5,0x7ffffff,1,0,u32,u32,CprCpRyU,common::RW> {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,CprCpRyU,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<CprCpRyU> for CprCpRyUT {
    #[inline(always)]
    fn reset_value(&self) -> CprCpRyU {
        CprCpRyU::new(0)
    }
}


 
#[doc = "CPUx Temporal Protection System Control Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsCon {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsCon {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsConT;
unsafe impl crate::common::AsPtr for TpsConT {}
impl crate::common::Reg<TpsCon> for TpsConT {}


unsafe impl crate::common::Read<TpsCon> for TpsConT {}
unsafe impl crate::common::Write<TpsCon> for TpsConT {}
impl TpsCon {
     
#[doc = "Timer0 Expired Flag   TEXP0. Set when the corresponding timer expires. Cleared on any write to the  TIMER0 register."]
    #[inline(always)]
    pub fn texp0(self) -> 
    crate::common::RegisterFieldBool<0,1,0,TpsCon,common::R> {
        
    crate::common::RegisterFieldBool::<0,1,0,TpsCon,common::R>::from_register(self,0)
    }
     
#[doc = "Timer1 Expired Flag   TEXP1. Set when the corresponding timer expires. Cleared on any write to the  TIMER1 register."]
    #[inline(always)]
    pub fn texp1(self) -> 
    crate::common::RegisterFieldBool<1,1,0,TpsCon,common::R> {
        
    crate::common::RegisterFieldBool::<1,1,0,TpsCon,common::R>::from_register(self,0)
    }
     
#[doc = "Timer1 Expired Flag   TEXP2. Set when the corresponding timer expires. Cleared on any write to the  TIMER1 register."]
    #[inline(always)]
    pub fn texp2(self) -> 
    crate::common::RegisterFieldBool<2,1,0,TpsCon,common::R> {
        
    crate::common::RegisterFieldBool::<2,1,0,TpsCon,common::R>::from_register(self,0)
    }
     
#[doc = "Temporal Protection Trap   TTRAP. If set  indicates that a TAE trap has been requested. Any subsequent TAE traps are disabled. A write clears the flag and re enables TAE traps."]
    #[inline(always)]
    pub fn ttrap(self) -> 
    crate::common::RegisterFieldBool<16,1,0,TpsCon,common::R> {
        
    crate::common::RegisterFieldBool::<16,1,0,TpsCon,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsCon> for TpsConT {
    #[inline(always)]
    fn reset_value(&self) -> TpsCon {
        TpsCon::new(0)
    }
}

 
#[doc = "CPUx Temporal Protection System Timer Register 0\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsTimer {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsTimer {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsTimerT;
unsafe impl crate::common::AsPtr for TpsTimerT {}
impl crate::common::Reg<TpsTimer> for TpsTimerT {}


unsafe impl crate::common::Read<TpsTimer> for TpsTimerT {}
unsafe impl crate::common::Write<TpsTimer> for TpsTimerT {}
impl TpsTimer {
     
#[doc = "Temporal Protection Timer   Timer. Writing zero de activates the Timer. Writing a non zero value starts the Timer. Any write clears the corresponding TPS CON.TEXP flag. Read returns the current Timer value."]
    #[inline(always)]
    pub fn timer(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,TpsTimer,common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,TpsTimer,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsTimer> for TpsTimerT {
    #[inline(always)]
    fn reset_value(&self) -> TpsTimer {
        TpsTimer::new(0)
    }
}


 
#[doc = "CPUx Exception Entry Timer Load Value\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtimEntryLval {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsExtimEntryLval {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsExtimEntryLvalT;
unsafe impl crate::common::AsPtr for TpsExtimEntryLvalT {}
impl crate::common::Reg<TpsExtimEntryLval> for TpsExtimEntryLvalT {}


unsafe impl crate::common::Read<TpsExtimEntryLval> for TpsExtimEntryLvalT {}
unsafe impl crate::common::Write<TpsExtimEntryLval> for TpsExtimEntryLvalT {}
impl TpsExtimEntryLval {
     
#[doc = "Exception Entry Timer Load value   ENTRY LVAL. Value loaded into the exception entry timer on detection of an enabled exception. Bits  3 0  are constrained to be 0"]
    #[inline(always)]
    pub fn entry_lval(self) -> crate::common::RegisterField<4,0xff,1,0,u8,u8,TpsExtimEntryLval,common::RW> {
        crate::common::RegisterField::<4,0xff,1,0,u8,u8,TpsExtimEntryLval,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsExtimEntryLval> for TpsExtimEntryLvalT {
    #[inline(always)]
    fn reset_value(&self) -> TpsExtimEntryLval {
        TpsExtimEntryLval::new(0)
    }
}

 
#[doc = "CPUx Exception Entry Timer Current Value\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtimEntryCval {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsExtimEntryCval {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsExtimEntryCvalT;
unsafe impl crate::common::AsPtr for TpsExtimEntryCvalT {}
impl crate::common::Reg<TpsExtimEntryCval> for TpsExtimEntryCvalT {}


unsafe impl crate::common::Read<TpsExtimEntryCval> for TpsExtimEntryCvalT {}
unsafe impl crate::common::Write<TpsExtimEntryCval> for TpsExtimEntryCvalT {}
impl TpsExtimEntryCval {
     
#[doc = "Exception Entry Timer Current Value   ENTRY CVAL. Current value of the exception entry timer."]
    #[inline(always)]
    pub fn entry_cval(self) -> crate::common::RegisterField<0,0xfff,1,0,u16,u16,TpsExtimEntryCval,common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,TpsExtimEntryCval,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsExtimEntryCval> for TpsExtimEntryCvalT {
    #[inline(always)]
    fn reset_value(&self) -> TpsExtimEntryCval {
        TpsExtimEntryCval::new(0)
    }
}

 
#[doc = "CPUx Exception Exit  Timer Load Value\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtimExitLval {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsExtimExitLval {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsExtimExitLvalT;
unsafe impl crate::common::AsPtr for TpsExtimExitLvalT {}
impl crate::common::Reg<TpsExtimExitLval> for TpsExtimExitLvalT {}


unsafe impl crate::common::Read<TpsExtimExitLval> for TpsExtimExitLvalT {}
unsafe impl crate::common::Write<TpsExtimExitLval> for TpsExtimExitLvalT {}
impl TpsExtimExitLval {
     
#[doc = "Exception Exit Timer Load value   EXIT LVAL. Value loaded into the exception exit timer on detection of an enabled exception. Bits  3 0  are constrained to be 0"]
    #[inline(always)]
    pub fn exit_lval(self) -> crate::common::RegisterField<4,0xfffff,1,0,u32,u32,TpsExtimExitLval,common::RW> {
        crate::common::RegisterField::<4,0xfffff,1,0,u32,u32,TpsExtimExitLval,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsExtimExitLval> for TpsExtimExitLvalT {
    #[inline(always)]
    fn reset_value(&self) -> TpsExtimExitLval {
        TpsExtimExitLval::new(0)
    }
}

 
#[doc = "CPUx Exception Exit Timer Current Value\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtimExitCval {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsExtimExitCval {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsExtimExitCvalT;
unsafe impl crate::common::AsPtr for TpsExtimExitCvalT {}
impl crate::common::Reg<TpsExtimExitCval> for TpsExtimExitCvalT {}


unsafe impl crate::common::Read<TpsExtimExitCval> for TpsExtimExitCvalT {}
unsafe impl crate::common::Write<TpsExtimExitCval> for TpsExtimExitCvalT {}
impl TpsExtimExitCval {
     
#[doc = "Exception Exit Timer Current Value   EXIT CVAL. Current value of the exception exit timer."]
    #[inline(always)]
    pub fn exit_cval(self) -> crate::common::RegisterField<0,0xffffff,1,0,u32,u32,TpsExtimExitCval,common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,TpsExtimExitCval,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsExtimExitCval> for TpsExtimExitCvalT {
    #[inline(always)]
    fn reset_value(&self) -> TpsExtimExitCval {
        TpsExtimExitCval::new(0)
    }
}

 
#[doc = "CPUx Exception Timer Class Enable Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtimClassEn {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsExtimClassEn {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsExtimClassEnT;
unsafe impl crate::common::AsPtr for TpsExtimClassEnT {}
impl crate::common::Reg<TpsExtimClassEn> for TpsExtimClassEnT {}


unsafe impl crate::common::Read<TpsExtimClassEn> for TpsExtimClassEnT {}
unsafe impl crate::common::Write<TpsExtimClassEn> for TpsExtimClassEnT {}
impl TpsExtimClassEn {
     
#[doc = "Exception Timer Class Enables   EXTIM CLASS EN. Trap Class enables for exception timer."]
    #[inline(always)]
    pub fn extim_class_en(self) -> crate::common::RegisterField<0,0xff,1,0,u8,u8,TpsExtimClassEn,common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,TpsExtimClassEn,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsExtimClassEn> for TpsExtimClassEnT {
    #[inline(always)]
    fn reset_value(&self) -> TpsExtimClassEn {
        TpsExtimClassEn::new(0)
    }
}

 
#[doc = "CPUx Exception Timer Status Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtimStat {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsExtimStat {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsExtimStatT;
unsafe impl crate::common::AsPtr for TpsExtimStatT {}
impl crate::common::Reg<TpsExtimStat> for TpsExtimStatT {}


unsafe impl crate::common::Read<TpsExtimStat> for TpsExtimStatT {}
unsafe impl crate::common::Write<TpsExtimStat> for TpsExtimStatT {}
impl TpsExtimStat {
     
#[doc = "Exception Exit Timer TIN   EXIT TIN. Exception Exit Timer TIN of triggering trap."]
    #[inline(always)]
    pub fn exit_tin(self) -> crate::common::RegisterField<0,0xff,1,0,u8,u8,TpsExtimStat,common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,TpsExtimStat,common::RW>::from_register(self,0)
    }
     
#[doc = "Exception Exit Timer Class   EXIT CLASS. Exception exit Timer Class of triggering trap."]
    #[inline(always)]
    pub fn exit_class(self) -> crate::common::RegisterField<8,0x7,1,0,u8,u8,TpsExtimStat,common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,TpsExtimStat,common::RW>::from_register(self,0)
    }
     
#[doc = "Exception Exit Timer Alarm Triggered   EXIT AT. Exception Exit Timer Alarm triggered sticky bit. Alarm triggered since last cleared."]
    #[inline(always)]
    pub fn exit_at(self) -> 
    crate::common::RegisterFieldBool<15,1,0,TpsExtimStat,common::R> {
        
    crate::common::RegisterFieldBool::<15,1,0,TpsExtimStat,common::R>::from_register(self,0)
    }
     
#[doc = "Exception Entry Timer TIN   ENTRY TIN. Exception Entry Timer TIN of triggering trap."]
    #[inline(always)]
    pub fn entry_tin(self) -> crate::common::RegisterField<16,0xff,1,0,u8,u8,TpsExtimStat,common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,TpsExtimStat,common::RW>::from_register(self,0)
    }
     
#[doc = "Exception Entry Timer Class   ENTRY CLASS. Exception Entry Timer Class of triggering trap."]
    #[inline(always)]
    pub fn entry_class(self) -> crate::common::RegisterField<24,0x7,1,0,u8,u8,TpsExtimStat,common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,TpsExtimStat,common::RW>::from_register(self,0)
    }
     
#[doc = "Exception Entry Timer Alarm Triggered   ENTRY AT. Exception Entry Timer Alarm triggered sticky bit. Alarm triggered since last cleared."]
    #[inline(always)]
    pub fn entry_at(self) -> 
    crate::common::RegisterFieldBool<31,1,0,TpsExtimStat,common::R> {
        
    crate::common::RegisterFieldBool::<31,1,0,TpsExtimStat,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsExtimStat> for TpsExtimStatT {
    #[inline(always)]
    fn reset_value(&self) -> TpsExtimStat {
        TpsExtimStat::new(0)
    }
}

 
#[doc = "CPUx Exception Timer FCX Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtimFcx {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TpsExtimFcx {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TpsExtimFcxT;
unsafe impl crate::common::AsPtr for TpsExtimFcxT {}
impl crate::common::Reg<TpsExtimFcx> for TpsExtimFcxT {}


unsafe impl crate::common::Read<TpsExtimFcx> for TpsExtimFcxT {}
unsafe impl crate::common::Write<TpsExtimFcx> for TpsExtimFcxT {}
impl TpsExtimFcx {
     
#[doc = "Exception Exit Timer FCX   EXIT FCX. Exception Exit Timer FCX of triggering trap."]
    #[inline(always)]
    pub fn exit_fcx(self) -> crate::common::RegisterField<0,0xfffff,1,0,u32,u32,TpsExtimFcx,common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32,u32,TpsExtimFcx,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TpsExtimFcx> for TpsExtimFcxT {
    #[inline(always)]
    fn reset_value(&self) -> TpsExtimFcx {
        TpsExtimFcx::new(0)
    }
}


 
#[doc = "CPUx Trap Control Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrapCon {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for FpuTrapCon {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct FpuTrapConT;
unsafe impl crate::common::AsPtr for FpuTrapConT {}
impl crate::common::Reg<FpuTrapCon> for FpuTrapConT {}


unsafe impl crate::common::Read<FpuTrapCon> for FpuTrapConT {}
unsafe impl crate::common::Write<FpuTrapCon> for FpuTrapConT {}
impl FpuTrapCon {
     
#[doc = "Trap Status   TST"]
    #[inline(always)]
    pub fn tst(self) -> crate::common::RegisterField<0,0x1,1,0,fpu_trap_con::Tst,fpu_trap_con::Tst,FpuTrapCon,common::R> {
        crate::common::RegisterField::<0,0x1,1,0,fpu_trap_con::Tst,fpu_trap_con::Tst,FpuTrapCon,common::R>::from_register(self,0)
    }
     
#[doc = "Trap Clear   TCL. Read  always reads as 0."]
    #[inline(always)]
    pub fn tcl(self) -> crate::common::RegisterField<1,0x1,1,0,fpu_trap_con::Tcl,fpu_trap_con::Tcl,FpuTrapCon,common::W> {
        crate::common::RegisterField::<1,0x1,1,0,fpu_trap_con::Tcl,fpu_trap_con::Tcl,FpuTrapCon,common::W>::from_register(self,0)
    }
     
#[doc = "Captured Rounding Mode   RM. The rounding mode of the captured instruction. Only valid when TST is asserted. Note that this is the rounding mode supplied to the FPU for the exceptional instruction. UPDFL instructions may cause a trap and change the rounding mode. In this case the RM bits capture the input rounding mode"]
    #[inline(always)]
    pub fn rm(self) -> crate::common::RegisterField<8,0x3,1,0,u8,u8,FpuTrapCon,common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,FpuTrapCon,common::R>::from_register(self,0)
    }
     
#[doc = "FX Trap Enable   FXE. When set  an instruction generating an FX exception will trigger a trap."]
    #[inline(always)]
    pub fn fxe(self) -> 
    crate::common::RegisterFieldBool<18,1,0,FpuTrapCon,common::RW> {
        
    crate::common::RegisterFieldBool::<18,1,0,FpuTrapCon,common::RW>::from_register(self,0)
    }
     
#[doc = "FU Trap Enable   FUE. When set  an instruction generating an FU exception will trigger a trap."]
    #[inline(always)]
    pub fn fue(self) -> 
    crate::common::RegisterFieldBool<19,1,0,FpuTrapCon,common::RW> {
        
    crate::common::RegisterFieldBool::<19,1,0,FpuTrapCon,common::RW>::from_register(self,0)
    }
     
#[doc = "FZ Trap Enable   FZE. When set  an instruction generating an FZ exception will trigger a trap."]
    #[inline(always)]
    pub fn fze(self) -> 
    crate::common::RegisterFieldBool<20,1,0,FpuTrapCon,common::RW> {
        
    crate::common::RegisterFieldBool::<20,1,0,FpuTrapCon,common::RW>::from_register(self,0)
    }
     
#[doc = "FV Trap Enable   FVE. When set  an instruction generating an FV exception will trigger a trap."]
    #[inline(always)]
    pub fn fve(self) -> 
    crate::common::RegisterFieldBool<21,1,0,FpuTrapCon,common::RW> {
        
    crate::common::RegisterFieldBool::<21,1,0,FpuTrapCon,common::RW>::from_register(self,0)
    }
     
#[doc = "FI Trap Enable   FIE. When set  an instruction generating an FI exception will trigger a trap."]
    #[inline(always)]
    pub fn fie(self) -> 
    crate::common::RegisterFieldBool<22,1,0,FpuTrapCon,common::RW> {
        
    crate::common::RegisterFieldBool::<22,1,0,FpuTrapCon,common::RW>::from_register(self,0)
    }
     
#[doc = "Captured FX   FX. Asserted if the captured instruction asserted FX. Only valid when TST is asserted."]
    #[inline(always)]
    pub fn fx(self) -> 
    crate::common::RegisterFieldBool<26,1,0,FpuTrapCon,common::R> {
        
    crate::common::RegisterFieldBool::<26,1,0,FpuTrapCon,common::R>::from_register(self,0)
    }
     
#[doc = "Captured FU   FU. Asserted if the captured instruction asserted FU. Only valid when TST is asserted."]
    #[inline(always)]
    pub fn fu(self) -> 
    crate::common::RegisterFieldBool<27,1,0,FpuTrapCon,common::R> {
        
    crate::common::RegisterFieldBool::<27,1,0,FpuTrapCon,common::R>::from_register(self,0)
    }
     
#[doc = "Captured FZ   FZ. Asserted if the captured instruction asserted FZ. Only valid when TST is asserted"]
    #[inline(always)]
    pub fn fz(self) -> 
    crate::common::RegisterFieldBool<28,1,0,FpuTrapCon,common::R> {
        
    crate::common::RegisterFieldBool::<28,1,0,FpuTrapCon,common::R>::from_register(self,0)
    }
     
#[doc = "Captured FV   FV. Asserted if the captured instruction asserted FV. Only valid when TST is asserted"]
    #[inline(always)]
    pub fn fv(self) -> 
    crate::common::RegisterFieldBool<29,1,0,FpuTrapCon,common::R> {
        
    crate::common::RegisterFieldBool::<29,1,0,FpuTrapCon,common::R>::from_register(self,0)
    }
     
#[doc = "Captured FI   FI. Asserted if the captured instruction asserted FI. Only valid when TST is asserted"]
    #[inline(always)]
    pub fn fi(self) -> 
    crate::common::RegisterFieldBool<30,1,0,FpuTrapCon,common::R> {
        
    crate::common::RegisterFieldBool::<30,1,0,FpuTrapCon,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<FpuTrapCon> for FpuTrapConT {
    #[inline(always)]
    fn reset_value(&self) -> FpuTrapCon {
        FpuTrapCon::new(0)
    }
}
pub mod fpu_trap_con {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Tst(u8);
    
    impl Tst {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Tst {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Tst {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Tst> for u64 {
        #[inline(always)]
        fn from(value: Tst) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Tst {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Tst {
         
#[doc = "0 No instruction captured.  The next enabled exception will cause the exceptional instruction to be captured."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Instruction captured. No further enabled exceptions will be captured until TST is cleared."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Tcl(u8);
    
    impl Tcl {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Tcl {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Tcl {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Tcl> for u64 {
        #[inline(always)]
        fn from(value: Tcl) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Tcl {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Tcl {
         
#[doc = "0 No effect."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Clears the trapped instruction  TST will be negated ."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Trapping Instruction Program Counter Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrapPc {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for FpuTrapPc {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct FpuTrapPcT;
unsafe impl crate::common::AsPtr for FpuTrapPcT {}
impl crate::common::Reg<FpuTrapPc> for FpuTrapPcT {}


unsafe impl crate::common::Read<FpuTrapPc> for FpuTrapPcT {}
unsafe impl crate::common::Write<FpuTrapPc> for FpuTrapPcT {}
impl FpuTrapPc {
     
#[doc = "Captured Program Counter   PC. The program counter  virtual address  of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
    #[inline(always)]
    pub fn pc(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,FpuTrapPc,common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,FpuTrapPc,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<FpuTrapPc> for FpuTrapPcT {
    #[inline(always)]
    fn reset_value(&self) -> FpuTrapPc {
        FpuTrapPc::new(0)
    }
}

 
#[doc = "CPUx Trapping Instruction Opcode Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrapOpc {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for FpuTrapOpc {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct FpuTrapOpcT;
unsafe impl crate::common::AsPtr for FpuTrapOpcT {}
impl crate::common::Reg<FpuTrapOpc> for FpuTrapOpcT {}


unsafe impl crate::common::Read<FpuTrapOpc> for FpuTrapOpcT {}
unsafe impl crate::common::Write<FpuTrapOpc> for FpuTrapOpcT {}
impl FpuTrapOpc {
     
#[doc = "Captured Opcode   OPC. The secondary opcode of the captured instruction. When FPU TRAP OPC.FMT 0 only bits  3 0  are defined. OPC is valid only when FPU TRAP CON.TST is asserted."]
    #[inline(always)]
    pub fn opc(self) -> crate::common::RegisterField<0,0xff,1,0,u8,u8,FpuTrapOpc,common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,FpuTrapOpc,common::R>::from_register(self,0)
    }
     
#[doc = "Captured Instruction Format   FMT. The format of the captured instruction s opcode. Only valid when FPU TRAP CON.TST is asserted."]
    #[inline(always)]
    pub fn fmt(self) -> crate::common::RegisterField<8,0x1,1,0,fpu_trap_opc::Fmt,fpu_trap_opc::Fmt,FpuTrapOpc,common::R> {
        crate::common::RegisterField::<8,0x1,1,0,fpu_trap_opc::Fmt,fpu_trap_opc::Fmt,FpuTrapOpc,common::R>::from_register(self,0)
    }
     
#[doc = "Captured Destination Register   DREG. The destination register of the captured instruction. ... Only valid when FPU TRAP CON.TST is asserted."]
    #[inline(always)]
    pub fn dreg(self) -> crate::common::RegisterField<16,0xf,1,0,fpu_trap_opc::Dreg,fpu_trap_opc::Dreg,FpuTrapOpc,common::R> {
        crate::common::RegisterField::<16,0xf,1,0,fpu_trap_opc::Dreg,fpu_trap_opc::Dreg,FpuTrapOpc,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<FpuTrapOpc> for FpuTrapOpcT {
    #[inline(always)]
    fn reset_value(&self) -> FpuTrapOpc {
        FpuTrapOpc::new(0)
    }
}
pub mod fpu_trap_opc {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fmt(u8);
    
    impl Fmt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fmt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fmt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fmt> for u64 {
        #[inline(always)]
        fn from(value: Fmt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fmt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fmt {
         
#[doc = "0 RRR"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 RR"]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Dreg(u8);
    
    impl Dreg {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Dreg {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Dreg {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Dreg> for u64 {
        #[inline(always)]
        fn from(value: Dreg) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Dreg {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Dreg {
         
#[doc = "0 Data general purpose register 0."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "F Data general purpose register 15."]
        pub const CONST_1515:Self =Self(15);
    }
}
 
#[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrapSrc1 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for FpuTrapSrc1 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct FpuTrapSrc1T;
unsafe impl crate::common::AsPtr for FpuTrapSrc1T {}
impl crate::common::Reg<FpuTrapSrc1> for FpuTrapSrc1T {}


unsafe impl crate::common::Read<FpuTrapSrc1> for FpuTrapSrc1T {}
unsafe impl crate::common::Write<FpuTrapSrc1> for FpuTrapSrc1T {}
impl FpuTrapSrc1 {
     
#[doc = "Captured SRC1 Operand   SRC1. The SRC1 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
    #[inline(always)]
    pub fn src1(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,FpuTrapSrc1,common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,FpuTrapSrc1,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<FpuTrapSrc1> for FpuTrapSrc1T {
    #[inline(always)]
    fn reset_value(&self) -> FpuTrapSrc1 {
        FpuTrapSrc1::new(0)
    }
}

 
#[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrapSrc2 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for FpuTrapSrc2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct FpuTrapSrc2T;
unsafe impl crate::common::AsPtr for FpuTrapSrc2T {}
impl crate::common::Reg<FpuTrapSrc2> for FpuTrapSrc2T {}


unsafe impl crate::common::Read<FpuTrapSrc2> for FpuTrapSrc2T {}
unsafe impl crate::common::Write<FpuTrapSrc2> for FpuTrapSrc2T {}
impl FpuTrapSrc2 {
     
#[doc = "Captured SRC2 Operand   SRC2. The SRC2 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
    #[inline(always)]
    pub fn src2(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,FpuTrapSrc2,common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,FpuTrapSrc2,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<FpuTrapSrc2> for FpuTrapSrc2T {
    #[inline(always)]
    fn reset_value(&self) -> FpuTrapSrc2 {
        FpuTrapSrc2::new(0)
    }
}

 
#[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrapSrc3 {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for FpuTrapSrc3 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct FpuTrapSrc3T;
unsafe impl crate::common::AsPtr for FpuTrapSrc3T {}
impl crate::common::Reg<FpuTrapSrc3> for FpuTrapSrc3T {}


unsafe impl crate::common::Read<FpuTrapSrc3> for FpuTrapSrc3T {}
unsafe impl crate::common::Write<FpuTrapSrc3> for FpuTrapSrc3T {}
impl FpuTrapSrc3 {
     
#[doc = "Captured SRC3 Operand   SRC3. The SRC3 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
    #[inline(always)]
    pub fn src3(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,FpuTrapSrc3,common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,FpuTrapSrc3,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<FpuTrapSrc3> for FpuTrapSrc3T {
    #[inline(always)]
    fn reset_value(&self) -> FpuTrapSrc3 {
        FpuTrapSrc3::new(0)
    }
}


 
#[doc = "CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrTRiEvt {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TrTRiEvt {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TrTRiEvtT;
unsafe impl crate::common::AsPtr for TrTRiEvtT {}
impl crate::common::Reg<TrTRiEvt> for TrTRiEvtT {}


unsafe impl crate::common::Read<TrTRiEvt> for TrTRiEvtT {}
unsafe impl crate::common::Write<TrTRiEvt> for TrTRiEvtT {}
impl TrTRiEvt {
     
#[doc = "Event Associated   EVTA. Specifies the Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(self) -> crate::common::RegisterField<0,0x7,1,0,tr_trievt::Evta,tr_trievt::Evta,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,tr_trievt::Evta,tr_trievt::Evta,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM. Code triggers BBM or BAM selection. Data access and data code combination access triggers can only create BAM Debug Events. When these triggers occur  TRnEVT.BBM is ignored."]
    #[inline(always)]
    pub fn bbm(self) -> crate::common::RegisterField<3,0x1,1,0,tr_trievt::Bbm,tr_trievt::Bbm,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,tr_trievt::Bbm,tr_trievt::Bbm,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(self) -> crate::common::RegisterField<4,0x1,1,0,tr_trievt::Bod,tr_trievt::Bod,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,tr_trievt::Bod,tr_trievt::Bod,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
    #[inline(always)]
    pub fn susp(self) -> 
    crate::common::RegisterFieldBool<5,1,0,TrTRiEvt,common::RW> {
        
    crate::common::RegisterFieldBool::<5,1,0,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(self) -> crate::common::RegisterField<6,0x3,1,0,tr_trievt::Cnt,tr_trievt::Cnt,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,tr_trievt::Cnt,tr_trievt::Cnt,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Input Selection   TYP"]
    #[inline(always)]
    pub fn typ(self) -> crate::common::RegisterField<12,0x1,1,0,tr_trievt::Typ,tr_trievt::Typ,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,tr_trievt::Typ,tr_trievt::Typ,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Compare Type   RNG. Once an even numbered comparator has been set to range  the EVTR settings of its associated upper neighbour will be ignored."]
    #[inline(always)]
    pub fn rng(self) -> crate::common::RegisterField<13,0x1,1,0,tr_trievt::Rng,tr_trievt::Rng,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,tr_trievt::Rng,tr_trievt::Rng,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Enable ASI Comparison   ASI EN"]
    #[inline(always)]
    pub fn asi_en(self) -> crate::common::RegisterField<15,0x1,1,0,tr_trievt::AsiEn,tr_trievt::AsiEn,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,tr_trievt::AsiEn,tr_trievt::AsiEn,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Address Space Identifier   ASI. The ASI of the Debug Trigger process."]
    #[inline(always)]
    pub fn asi(self) -> crate::common::RegisterField<16,0x1f,1,0,u8,u8,TrTRiEvt,common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Address Store   AST. Used in conjunction with TYP 0"]
    #[inline(always)]
    pub fn ast(self) -> 
    crate::common::RegisterFieldBool<27,1,0,TrTRiEvt,common::RW> {
        
    crate::common::RegisterFieldBool::<27,1,0,TrTRiEvt,common::RW>::from_register(self,0)
    }
     
#[doc = "Address Load   ALD. Used in conjunction with TYP 0"]
    #[inline(always)]
    pub fn ald(self) -> 
    crate::common::RegisterFieldBool<28,1,0,TrTRiEvt,common::RW> {
        
    crate::common::RegisterFieldBool::<28,1,0,TrTRiEvt,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TrTRiEvt> for TrTRiEvtT {
    #[inline(always)]
    fn reset_value(&self) -> TrTRiEvt {
        TrTRiEvt::new(0)
    }
}
pub mod tr_trievt {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _,  CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
        ResetValue as _, Write as _,
    };
    
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Evta(u8);
    
    impl Evta {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Evta {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Evta {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Evta> for u64 {
        #[inline(always)]
        fn from(value: Evta) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Evta {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Evta {
         
#[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "011 BOD 0  Breakpoint trap and pulse BRKOUT Signal. BOD 1  Breakpoint trap."]
        pub const CONST_33:Self =Self(3);
         
#[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
        pub const CONST_44:Self =Self(4);
         
#[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
        pub const CONST_55:Self =Self(5);
         
#[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
        pub const CONST_66:Self =Self(6);
         
#[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
        pub const CONST_77:Self =Self(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bbm(u8);
    
    impl Bbm {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bbm {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bbm {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bbm> for u64 {
        #[inline(always)]
        fn from(value: Bbm) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bbm {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Bbm {
         
#[doc = "0 Code only triggers Break After Make  BAM ."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Code only triggers Break Before Make  BBM ."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Bod(u8);
    
    impl Bod {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Bod {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Bod {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Bod> for u64 {
        #[inline(always)]
        fn from(value: Bod) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Bod {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Bod {
         
#[doc = "0 BRKOUT signal asserted according to the action specified in the EVTA field."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Cnt(u8);
    
    impl Cnt {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Cnt {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Cnt {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Cnt> for u64 {
        #[inline(always)]
        fn from(value: Cnt) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Cnt {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Cnt {
         
#[doc = "00 No change."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "01 Start the performance counters."]
        pub const CONST_11:Self =Self(1);
         
#[doc = "10 Stop the performance counters."]
        pub const CONST_22:Self =Self(2);
         
#[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
        pub const CONST_33:Self =Self(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Typ(u8);
    
    impl Typ {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Typ {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Typ {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Typ> for u64 {
        #[inline(always)]
        fn from(value: Typ) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Typ {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Typ {
         
#[doc = "0 Address"]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 PC"]
        pub const CONST_11:Self =Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Rng(u8);
    
    impl Rng {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Rng {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Rng {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Rng> for u64 {
        #[inline(always)]
        fn from(value: Rng) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Rng {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Rng {
         
#[doc = "1 Range"]
        pub const CONST_11:Self =Self(1);
         
#[doc = "0 Equality"]
        pub const CONST_00:Self =Self(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct AsiEn(u8);
    
    impl AsiEn {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for AsiEn {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for AsiEn {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<AsiEn> for u64 {
        #[inline(always)]
        fn from(value: AsiEn) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for AsiEn {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl AsiEn {
         
#[doc = "0 No ASI comparison performed. Debug Trigger is valid for all processes."]
        pub const CONST_00:Self =Self(0);
         
#[doc = "1 Enable ASI comparison. Debug Events are only triggered when the current process ASI matches TRnEVT.ASI."]
        pub const CONST_11:Self =Self(1);
    }
}
 
#[doc = "CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrTRiAdr {
    pub(crate) data: u32,
    pub(crate) mask: u32
}

impl crate::common::RegisterValue for TrTRiAdr {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self {
            data,
            mask: 0x0,
        }
    }
}


#[doc(hidden)]
pub struct TrTRiAdrT;
unsafe impl crate::common::AsPtr for TrTRiAdrT {}
impl crate::common::Reg<TrTRiAdr> for TrTRiAdrT {}


unsafe impl crate::common::Read<TrTRiAdr> for TrTRiAdrT {}
unsafe impl crate::common::Write<TrTRiAdr> for TrTRiAdrT {}
impl TrTRiAdr {
     
#[doc = "Comparison Address   ADDR. For PC comparison  bit 0  is always zero."]
    #[inline(always)]
    pub fn addr(self) -> crate::common::RegisterField<0,0xffffffff,1,0,u32,u32,TrTRiAdr,common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,TrTRiAdr,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TrTRiAdr> for TrTRiAdrT {
    #[inline(always)]
    fn reset_value(&self) -> TrTRiAdr {
        TrTRiAdr::new(0)
    }
}








