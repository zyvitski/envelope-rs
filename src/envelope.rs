use num::Float;
use std::fmt::Debug;

#[derive(Debug)]
pub enum EnvelopeState {
    Ready,
    Attack,
    Decay,
    Sustain,
    Release,
    Done,
}

pub trait Envelope<F>
where
    F: Float,
    Self: Iterator<Item = F> + Debug,
{
    fn reset(&mut self);

    fn note_on(&mut self);

    fn note_off(&mut self);

    fn set_attack(&mut self, value: F);

    fn set_decay(&mut self, value: F);

    fn set_sustain(&mut self, value: F);

    fn set_release(&mut self, value: F);

    fn set_initial(&mut self, value: F);

    fn set_peak(&mut self, value: F);

    fn set_end(&mut self, value: F);

    fn is_done(&self) -> bool;
}