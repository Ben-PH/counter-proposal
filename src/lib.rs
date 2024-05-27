#![no_std]

pub trait Duration {}
impl Duration for u32 {}
impl Duration for u64 {}
/// Base encapsulation for reading a clock perihpereal. Intended typical use-case is
/// a HAL implementation adds this trait to a HAL type, and uses this interface to
/// read a clock/timer register. Through implementation of this trait, this object
/// will carry with it the means to read a clocks tick counter, and communicate the
/// total duration this represents.
pub trait TimeCount: Sized {
    /// Typically u32 or u64 (or held within a type-wrapper) that corresponds with the
    /// data type held within the peripheral register being read.
    type RawData: Copy;

    /// The length of time that each clock-tick measures. Setting up a clock with a 80MHz,
    /// a HAL might opt to set this as `fugit::TimerDuration<Self::RawData, 1, 80_000_000>`.
    ///
    /// Another option might something that expresses the measure as a frequency, in which
    /// case, something that encapsulates "eighty million" (as opposed to "one eighty
    /// millionth").
    ///
    /// META: This might merit being a const instead.
    /// META: Should it also be documented about the Mul constraint?
    type TickMeasure;

    type Error;

    /// Intended as an interface to a raw-register read.
    fn try_now_raw(&self) -> Result<Self::RawData, Self::Error>;

    /// Interprates the tick-count of `try_now_raw`, and scales based on `TickMeasure`
    fn try_now(&self) -> Result<Self::TickMeasure, Self::Error>{
        Ok(<Self as TimeCount>::raw_to_measure(self.try_now_raw()?))
    }
    fn raw_to_measure(from: Self::RawData) -> Self::TickMeasure;
}

/// Base encapsulation for reading a peripheral that tracks a count, e.g. pulse-counter.
pub trait Counter: Sized {
    /// Typically an integer, possibly type-wrappend. If it is only ever increasing from zero,
    /// then an unsigned would be generally recommended
    type RawData: Copy;

    /// The interpretation of the raw data. E.g. A rotary pulse encoder, with 1 increment per
    /// milliradians, a type-wrapping `MilliRadians<Self::RawData>` might be chosen
    type CountMeasure;

    type Error;

    /// Intended as an interface to a raw-register read.
    fn try_read_raw(&self) -> Result<Self::RawData, Self::Error>;

    /// Interprates the count of `try_read_raw`, and scales based on `CountMeasure`
    fn try_read(&self) -> Result<Self::CountMeasure, Self::Error> {
        Ok(<Self as Counter>::raw_to_measure(self.try_read_raw()?))
    }

    fn raw_to_measure(from: Self::RawData) -> Self::CountMeasure;
}
