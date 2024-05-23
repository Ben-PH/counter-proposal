#![no_std]

use core::ops::Mul;

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
    type RawData;

    /// The length of time that each clock-tick measures. Setting up a clock with a 80MHz,
    /// a HAL might opt to set this as `fugit::TimerDuration<Self::RawData, 1, 80_000_000>`.
    ///
    /// Another option might something that expresses the measure as a frequency, in which
    /// case, something that encapsulates "eighty million" (as opposed to "one eighty
    /// millionth").
    ///
    /// META: This might merit being a const instead.
    /// META: Should it also be documented about the Mul constraint?
    type TickMeasure: Mul<Self::RawData, Output = Self::TimeMeasure> + Default;

    /// A combinasion of raw count and measure. 80k ticks with 80MHz => 1 second.
    type TimeMeasure;

    type Error;

    /// Intended as an interface to a raw-register read.
    fn try_now_raw(&self) -> Result<Self::RawData, Self::Error>;

    /// Interprates the tick-count of `try_now_raw`, and scales based on `TickMeasure`
    fn try_now(&self) -> Result<Self::TimeMeasure, Self::Error> {
        Ok(Self::TickMeasure::default() * self.try_now_raw()?)
    }
}
