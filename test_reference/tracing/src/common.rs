/*
Test license

*/
// Generated from SVD 1.2, with svd2pac 0.8.0 on Tue, 30 Jun 2026 14:31:13 +0000

use core::convert::From;
use core::marker::PhantomData;

#[cfg(feature = "tracing")]
use crate::tracing;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

pub(crate) mod sealed {
    use super::*;
    pub trait AccessBitfield {}
    impl AccessBitfield for R {}
    impl AccessBitfield for W {}
    impl AccessBitfield for RW {}
    use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr};

    pub trait RegNumberT:
        Copy
        + From<u8>
        + Into<u64>
        + CastFrom<u64>
        + Shr<usize, Output = Self>
        + Shl<usize, Output = Self>
        + BitAndAssign
        + BitAnd<Output = Self>
        + Not<Output = Self>
        + BitOrAssign
    {
    }
    impl RegNumberT for u8 {}
    impl RegNumberT for u16 {}
    impl RegNumberT for u32 {}
    impl RegNumberT for u64 {}
}

// It would be better with const fn
// waiting for RFC: const functions in traits #3490
pub trait CastFrom<A> {
    fn cast_from(val: A) -> Self;
}

impl CastFrom<u64> for u8 {
    #[inline(always)]
    fn cast_from(val: u64) -> Self {
        val as Self
    }
}

impl CastFrom<u64> for u16 {
    #[inline(always)]
    fn cast_from(val: u64) -> Self {
        val as Self
    }
}

impl CastFrom<u64> for u32 {
    #[inline(always)]
    fn cast_from(val: u64) -> Self {
        val as Self
    }
}

impl CastFrom<u64> for u64 {
    #[inline(always)]
    fn cast_from(val: u64) -> Self {
        val as Self
    }
}

pub trait AccessBitfield: sealed::AccessBitfield + Copy {}
impl AccessBitfield for R {}
impl AccessBitfield for W {}
impl AccessBitfield for RW {}

pub trait ReadBitfield: AccessBitfield {}
impl ReadBitfield for RW {}
impl ReadBitfield for R {}

pub trait WriteBitfield: AccessBitfield {}
impl WriteBitfield for RW {}
impl WriteBitfield for W {}

/// Trait for the `as_ptr` and `from_ptr` methods,
/// allowing register and cluster types to be converted to and from raw pointers.
///
/// # Safety
///
/// This trait is intended to be implemented by register and cluster types. The
/// `as_ptr` method must return a valid pointer to the register's MMIO address,
/// and calling `from_ptr` with the result of `as_ptr` (and vice versa) must
/// correctly roundtrip.
pub unsafe trait AsPtr: Sized {
    /// Returns a raw pointer with the address of `self`.
    fn as_ptr(&self) -> *mut u8 {
        self as *const _ as *mut u8
    }

    /// Creates a new instance of this type from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be non-null, as well as valid and properly aligned for the read and write operations
    /// performed on the resulting register instance.
    #[inline(always)]
    #[must_use]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        unsafe { &*(ptr as *const Self) }
    }
}

/// Trait for memory-mapped peripheral registers.
///
/// Extends [`AsPtr`] with utility methods to obtain a typed pointer to the register's
/// MMIO address. Implement this on the zero-sized spec type for each memory-mapped register.
pub trait Reg<RegValueT: RegisterValue>: Sized + AsPtr {
    #[inline(always)]
    #[must_use]
    fn ptr(&self) -> *mut RegValueT::DataType {
        self.as_ptr() as *mut RegValueT::DataType
    }

    /// Returns the address of the register.
    fn addr(&self) -> usize {
        (self as *const _) as usize
    }
}

/// Reset value of a register
pub trait ResetValue<RegValueT: RegisterValue>: Reg<RegValueT> {
    fn reset_value(&self) -> RegValueT;
}

use sealed::RegNumberT;

pub trait RegisterValue: Sized + Clone + Copy + Sized {
    type DataType: RegNumberT;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType);

    fn inner(&self) -> (Self::DataType, Self::DataType);

    /// Create a register value that could be written to a register from raw integer
    ///
    /// ```rust, ignore
    /// // example with generic names
    /// // needs: use test_pac::{timer, RegisterValue, TIMER}
    /// let to_write = timer::BitfieldReg::new(0xdeadbeef);
    /// TIMER.bitfield_reg().write(to_write);
    /// let to_write = to_write.boolw().set(true);
    /// TIMER.bitfield_reg().write(to_write);
    /// ```
    #[must_use]
    fn new(data: Self::DataType) -> Self;

    /// Get raw integer from value read from register
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{RegisterValue, TIMER}
    /// let x = TIMER.bitfield_reg().read().get_raw();
    /// ```
    #[must_use]
    #[inline(always)]
    fn get_raw(&self) -> Self::DataType {
        let (data, _) = self.inner();
        data
    }

    /// Prepare a register value that could be written to a register with an arbitrary value
    ///
    /// Use this function for setting a register to a custom value, independent
    /// of bitfields, enumerations, etc. No checks are performed on the passed
    /// value. The whole register is updated on write.
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{RegisterValue, TIMER}
    /// TIMER.bitfield_reg().init(|r| r.set_raw(0xdeadbeef))
    /// ```
    #[must_use]
    #[inline(always)]
    fn set_raw(mut self, value: Self::DataType) -> Self {
        let (data, mask) = self.inner_mut();
        *data = value;
        *mask = !(Into::<Self::DataType>::into(0x0u8));
        self
    }
}

pub trait NoBitfieldReg: RegisterValue
where
    Self: Sized,
{
    /// Get value read from register
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{NoBitfieldReg, TIMER}
    /// let x = TIMER.nobitfield_reg().read().get();
    /// ```
    #[inline(always)]
    #[must_use]
    fn get(&self) -> Self::DataType {
        self.get_raw()
    }

    /// Prepare value to be written to register
    ///
    /// ```rust,ignore
    /// // example with generic names
    /// // needs: use pac::{NoBitfieldReg, TIMER}
    /// TIMER.nobitfield_reg().init(|r| r.set(0xc0ffee));
    /// ```
    #[inline(always)]
    #[must_use]
    fn set(self, value: Self::DataType) -> Self {
        self.set_raw(value)
    }
}

/// Readable register trait
///
/// # Safety
/// Read operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
pub unsafe trait Read<RegValueT: RegisterValue>: Reg<RegValueT> {
    /// Read register and return a register value
    ///
    /// # Safety
    /// Read operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// let reg = unsafe { TIMER.bitfield_reg().read() };
    /// if reg.boolr().get() { /* ... */ }
    /// ```
    #[inline(always)]
    #[must_use]
    unsafe fn read(&self) -> RegValueT {
        #[cfg(feature = "tracing")]
        let val = {
            let mut buf: u64 = 0x0;
            tracing::READ_FN.with(|rf| {
                if let Some(rf) = rf.get() {
                    buf = rf(self.addr(), std::mem::size_of::<RegValueT::DataType>());
                } else {
                    #[cfg(not(feature = "tracing_dummy"))]
                    panic!(
                        "Please, provide an handler for read with tracing::set_read_fn(callback);"
                    );
                }
            });
            RegValueT::DataType::cast_from(buf)
        };
        #[cfg(not(feature = "tracing"))]
        let val = self.ptr().read_volatile();
        RegValueT::new(val)
    }
}

/// Writable register trait
///
/// # Safety
/// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
pub unsafe trait Write<RegValueT: RegisterValue>: Reg<RegValueT> {
    /// Write register value back to register
    ///
    /// # Arguments
    ///
    /// * `reg_value` - Register value to write back to the register
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developers shall read the device user manual.
    /// Register is Send and Sync to allow complete freedom. Developers are responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// // write with a previously read value
    /// let reg = unsafe { TIMER.bitfield_reg().read() };
    /// // or start with a known value
    /// let reg = timer::BitfieldReg::new(0).bitfieldw().set(0x55);
    /// // or start with the register reset value
    /// let reg = TIMER.bitfield_reg().reset_value();
    ///
    /// let reg = reg.bitfieldrw().set(0x77);
    ///
    /// // no change has taken place to the register due to `set` calls - do that now by writing back the result
    /// unsafe { TIMER.bitfield_reg().write(reg) }
    /// ```
    /// See also: [`Write::init`] which provides the reset value to a closure
    #[inline(always)]
    unsafe fn write(&self, reg_value: RegValueT) {
        #[cfg(feature = "tracing")]
        tracing::WRITE_FN.with(|wf| {
            if let Some(wf) = wf.get() {
                wf(
                    self.addr(),
                    std::mem::size_of::<RegValueT::DataType>(),
                    reg_value.get_raw().into(),
                )
            } else {
                #[cfg(not(feature = "tracing_dummy"))]
                panic!("Please, provide an handler for read with tracing::set_read_fn(callback);");
            }
        });
        #[cfg(not(feature = "tracing"))]
        self.ptr().write_volatile(reg_value.get_raw());
    }

    /// Write an arbitrary integer to register
    ///
    /// Use this function when e.g. loading data to be written from a config-page.
    /// For normal use prefer either [`Write::write`] if the value was read before, or [`Write::init`],
    /// both of which provide some restrictions available register fields, enums, etc.
    ///
    /// # Arguments
    ///
    /// * `value` - The unchecked value to be written to the register
    ///
    /// # Safety
    ///
    /// Write operation could cause undefined behavior for some peripheral. Developers shall read the device user manual.
    /// Register is Send and Sync to allow complete freedom. Developers are responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// unsafe { TIMER.bitfield_reg().write_raw(0xdead) }
    /// ```
    /// See also [`Write::init`] and [`Write::write`] both of which are the preferred functions.
    #[inline(always)]
    unsafe fn write_raw(&self, value: RegValueT::DataType) {
        #[cfg(feature = "tracing")]
        tracing::WRITE_FN.with(|wf| {
            if let Some(wf) = wf.get() {
                wf(
                    self.addr(),
                    std::mem::size_of::<RegValueT::DataType>(),
                    value.into(),
                )
            } else {
                #[cfg(not(feature = "tracing_dummy"))]
                panic!("Please, provide an handler for read with tracing::set_read_fn(callback);");
            }
        });
        #[cfg(not(feature = "tracing"))]
        self.ptr().write_volatile(value);
    }

    /// Write register with register value built from the register reset value
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receives as input the register reset value (value at Power On Reset).
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// TIMER
    ///     .bitfield_reg()
    ///     .init(|r| r.bitfieldw().set(0b1010).boolw().set(true));
    /// ```
    #[inline(always)]
    /// Write value computed by closure that receive as input the reset value of register
    unsafe fn init(&self, f: impl FnOnce(RegValueT) -> RegValueT)
    where
        Self: ResetValue<RegValueT>,
    {
        let val = self.reset_value();
        let res = f(val);
        self.write(res);
    }
}

pub trait Modify<RegValueT: RegisterValue>: Read<RegValueT> + Write<RegValueT> {
    /// Read/modify/write register
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value read from register. The result of the closure
    ///   is written back to the register.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// TIMER
    ///     .bitfield_reg()
    ///     .modify(|r| r.boolrw().set(!r.boolrw().get()));
    /// ```
    #[inline(always)]
    unsafe fn modify(&self, f: impl FnOnce(RegValueT) -> RegValueT) {
        let val = self.read();
        let res = f(val);
        self.write(res);
    }
}

impl<RegValueT: RegisterValue, T: Read<RegValueT> + Write<RegValueT>> Modify<RegValueT> for T {}

pub trait EnumBitfieldStruct: Clone + Copy + Sized {
    type RegNumberT: RegNumberT;
    fn value(&self) -> Self::RegNumberT;
}

/// Proxy struct for numeric bitfields
pub struct RegisterField<
    const START_OFFSET: usize,
    const MASK: u64,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    ValueTypeRead,
    ValueTypeWrite,
    T,
    A,
> where
    T: RegisterValue,
    A: AccessBitfield,
{
    data: T,
    index: u8,
    marker: PhantomData<(ValueTypeRead, ValueTypeWrite, A)>,
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueTypeRead,
        ValueTypeWrite,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueTypeRead, ValueTypeWrite, T, A>
where
    T: RegisterValue,
    A: AccessBitfield,
{
    #[allow(dead_code)]
    #[inline(always)]
    #[doc(hidden)]
    pub fn from_register(data: T, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }

    /// Get mask for bitfield, the mask is unshifted and at offset 0
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueTypeRead,ValueTypeWrite, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub fn mask(&self) -> T::DataType {
        T::DataType::cast_from(MASK)
    }

    /// Get offset of bitfield in containing register
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueTypeRead,ValueTypeWrite, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub const fn offset(&self) -> usize {
        START_OFFSET + (self.index * DIM_INCREMENT) as usize
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueTypeRead,
        ValueTypeWrite,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueTypeRead, ValueTypeWrite, T, A>
where
    T: RegisterValue,
    A: ReadBitfield,
    ValueTypeRead: CastFrom<u64>,
{
    /// Extract bitfield from read register value
    #[inline(always)]
    pub fn get(&self) -> ValueTypeRead {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered: T::DataType = (self.data.get_raw() >> offset) & T::DataType::cast_from(MASK);
        ValueTypeRead::cast_from(filtered.into())
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueTypeRead,
        ValueTypeWrite,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueTypeRead, ValueTypeWrite, T, A>
where
    T: RegisterValue,
    A: WriteBitfield,
    u64: From<ValueTypeWrite>,
{
    /// Prepare bitfield value that could be written to register
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// // get an instance by reading
    /// let values = TIMER.bitfield_reg().read();
    /// // or by starting with a known value
    /// let value = timer::BitfieldReg::new(0);
    /// // or by starting with the register reset value
    /// let value = TIMER.bitfield_reg().reset_value();
    ///
    /// // set bitfields
    /// let value = value
    ///     // set numeric bitfield
    ///     .bitfieldw()
    ///     .set(0x55)
    ///     // set enumerated bitfield with enumeration
    ///     .bitfieldenumerated()
    ///     .set(timer::bitfield_reg::BitfieldEnumerated::GPIOA_0)
    ///     // set enumerated bitfield from integer
    ///     .bitfieldenumerated()
    ///     .set(1.into());
    ///
    /// // up until now no hardware change has taken place, do that now by writing
    /// TIMER.bitfield_reg().write(value);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: ValueTypeWrite) -> T {
        let mask = T::DataType::cast_from(MASK);
        let value: T::DataType = T::DataType::cast_from(Into::<u64>::into(value)) & mask;
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset: T::DataType = mask << offset;
        let (data, mask) = self.data.inner_mut();
        *mask |= masked_offset;
        *data &= !masked_offset;
        *data |= value << offset;
        self.data
    }
}

/// Proxy struct for boolean bitfields
pub struct RegisterFieldBool<
    const START_OFFSET: usize,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    T,
    A,
> where
    T: RegisterValue,
    A: AccessBitfield,
{
    data: T,
    index: u8,
    marker: PhantomData<A>,
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegisterValue,
    A: ReadBitfield,
{
    /// Extract bitfield from read register value
    #[inline(always)]
    pub fn get(&self) -> bool {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered = (self.data.get_raw().into() >> offset) & 1;
        filtered == 1
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegisterValue,
    A: WriteBitfield,
{
    /// Prepare bitfield value to be written to register
    ///
    /// # Example
    /// ```rust,ignore
    /// // example with generic names
    /// // get an instance by reading
    /// let values = TIMER.bitfield_reg().read();
    /// // or by starting with a known value
    /// let value = timer::BitfieldReg::new(0);
    /// // or by starting with the register reset value
    /// let value = TIMER.bitfield_reg().reset_value();
    ///
    /// // set bitfield
    /// let value = value
    ///     .boolrw()
    ///     .set(true);
    ///
    /// // up until now no hardware change has taken place, do that now by writing
    /// TIMER.bitfield_reg().write(value);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: bool) -> T {
        let value: T::DataType = if value {
            T::DataType::cast_from(1u64)
        } else {
            T::DataType::cast_from(0u64)
        };
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset = T::DataType::cast_from(0x1u64) << offset;
        let (data, mask) = self.data.inner_mut();
        *mask |= masked_offset;
        *data &= !masked_offset;
        *data |= value << offset;
        self.data
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegisterValue,
    A: AccessBitfield,
{
    #[inline(always)]
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn from_register(data: T, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }

    /// Get mask for bitfield, the mask is unshifted and at offset 0
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub fn mask(&self) -> T::DataType {
        T::DataType::cast_from(1)
    }

    /// Get offset of bitfield in containing register
    ///
    /// Prefer the use of [`RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>::get()`] to
    /// extract a bitfield value.
    #[inline(always)]
    #[must_use]
    pub const fn offset(&self) -> usize {
        START_OFFSET + (self.index * DIM_INCREMENT) as usize
    }
}

/// An array of identical register clusters.
pub struct ClusterRegisterArray<T: Sized, const DIM: usize, const DIM_INCREMENT: usize> {
    _t: ::core::marker::PhantomData<T>,
}

impl<T: Sized, const DIM: usize, const DIM_INCREMENT: usize>
    ClusterRegisterArray<T, DIM, DIM_INCREMENT>
{
    /// Returns the number of register blocks in the cluster.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        DIM
    }

    /// Returns whether the cluster is empty (DIM == 0).
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        DIM == 0
    }

    /// Returns an iterator over the elements of this cluster.
    #[inline(always)]
    pub fn iter(&self) -> impl ::core::iter::ExactSizeIterator<Item = &T> {
        self.into_iter()
    }

    /// Returns the cluster element with the specified index.
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub const fn get(&self, index: usize) -> &T {
        assert!(index < DIM);
        unsafe { self.get_unchecked(index) }
    }

    /// Returns the cluster element with the specified index.
    ///
    /// # Safety
    ///
    /// `index` must be less than `DIM`.
    #[inline(always)]
    pub const unsafe fn get_unchecked(&self, index: usize) -> &T {
        &*(self.as_ptr().add(index * DIM_INCREMENT) as *const _)
    }

    #[inline(always)]
    #[doc(hidden)]
    pub const unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const Self)
    }

    #[inline(always)]
    const fn as_ptr(&self) -> *mut u8 {
        self as *const _ as *mut _
    }
}

unsafe impl<T: Sized + 'static, const DIM: usize, const DIM_INCREMENT: usize> AsPtr
    for ClusterRegisterArray<T, DIM, DIM_INCREMENT>
{
    #[inline(always)]
    fn as_ptr(&self) -> *mut u8 {
        self.as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        ClusterRegisterArray::from_ptr(ptr)
    }
}

impl<T: Sized, const DIM: usize, const DIM_INCREMENT: usize> ::core::ops::Index<usize>
    for ClusterRegisterArray<T, DIM, DIM_INCREMENT>
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

impl<'a, T: Sized, const DIM: usize, const DIM_INCREMENT: usize> IntoIterator
    for &'a ClusterRegisterArray<T, DIM, DIM_INCREMENT>
{
    type Item = &'a T;
    type IntoIter = ClusterRegisterArrayIterator<'a, T, DIM, DIM_INCREMENT>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        ClusterRegisterArrayIterator {
            array: self,
            index: 0,
        }
    }
}

pub struct ClusterRegisterArrayIterator<'a, T: Sized, const DIM: usize, const DIM_INCREMENT: usize>
{
    array: &'a ClusterRegisterArray<T, DIM, DIM_INCREMENT>,
    index: usize,
}

impl<'a, T: Sized, const DIM: usize, const DIM_INCREMENT: usize> Iterator
    for ClusterRegisterArrayIterator<'a, T, DIM, DIM_INCREMENT>
{
    type Item = &'a T;
    #[inline(always)]
    fn next(&mut self) -> Option<&'a T> {
        if self.index < self.array.len() {
            let result = &self.array[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.array.len() - self.index;
        (len, Some(len))
    }
}

impl<T: Sized, const DIM: usize, const DIM_INCREMENT: usize> ExactSizeIterator
    for ClusterRegisterArrayIterator<'_, T, DIM, DIM_INCREMENT>
{
}
