use num::{Num, Signed};
use std::fmt::Debug;
use util::{NonZero, Normal};

///Represents the current state of the envelope
#[derive(Debug)]
pub enum EnvelopeState {
    ///The Envelope is not active and ready to be triggered.
    Ready,
    ///The `Envelope` is in the `Attack` stage.
    Attack,
    ///The `Envelope` is in the `Decay` stage.
    Decay,
    ///The `Envelope` is in the `Sustain` stage.
    ///It will remain this way until `note_off` is called.
    Sustain,
    ///The `Envelope` is in the `Release` stage.
    Release,
    ///The `Envelope` has completed its cycle.
    Done,
}

///A generic trait representnig an `Envelope` generator.
pub trait Envelope<F>
where
    F: Num + Signed + PartialOrd + NonZero + Normal + Copy,
    Self: Iterator<Item = F> + Debug,
{
    ///Reset the `Envelope` to a `Ready` state.
    fn reset(&mut self);

    ///Trigger the `Envelope` to start its cycle.
    fn note_on(&mut self);

    ///Transition the `Envelope` into the `Release` state.
    fn note_off(&mut self);

    ///Set the attack time in samples.
    fn set_attack(&mut self, value: F);

    ///Set the decay time in samples.
    fn set_decay(&mut self, value: F);

    ///Set the sustain value.
    ///The valid range is 0..F::MAX.
    fn set_sustain(&mut self, value: F);

    ///Set the release time in samples.
    fn set_release(&mut self, value: F);

    ///Set the initial value.
    ///The valid range is 0..F::MAX.
    fn set_initial(&mut self, value: F);

    ///Set the peak value.
    ///The valid range is 0..F::MAX.
    fn set_peak(&mut self, value: F);

    ///Set the end value.
    ///The valid range is 0..F::MAX.
    fn set_end(&mut self, value: F);

    ///Indicates whether the `Envelope` is in the `Done` state.
    fn is_done(&self) -> bool;

    ///Indicates whether the `Envelope` is in the `Ready` state.
    fn is_ready(&self) -> bool;
}
