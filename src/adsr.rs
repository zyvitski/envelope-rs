use envelope::{Envelope, EnvelopeState};
use util::{nonzero, normal};
use num::{Float, Zero};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Adsr<F>
where
    F: Float,
{
    attack: F,
    decay: F,
    sustain: F,
    release: F,
    initial: F,
    peak: F,
    end: F,
    state: EnvelopeState,
    slope: F,
    value: F,
    note_is_on: bool,
    attack_slope: F,
    decay_slope: F,
    release_slope: F,
}

impl<F> Adsr<F>
where
    F: Float,
{
    pub fn new(attack: F, decay: F, sustain: F, release: F, initial: F, peak: F, end: F) -> Self {
        let zero: F = Zero::zero();
        Adsr {
            attack: nonzero(attack),
            decay: nonzero(decay),
            sustain: normal(sustain),
            release: nonzero(release),
            initial: normal(initial),
            peak: normal(peak),
            end: normal(end),
            state: EnvelopeState::Ready,
            slope: zero,
            value: initial,
            note_is_on: false,
            attack_slope: zero,
            decay_slope: zero,
            release_slope: zero,
        }
    }

    fn calc_attack_slope(&mut self) -> F {
        (self.peak - self.initial) / self.attack
    }
    fn calc_decay_slope(&mut self) -> F {
        (self.sustain - self.peak) / self.decay
    }
    fn calc_release_slope(&mut self) -> F {
        (self.end - self.sustain) / self.release
    }
}

impl<F> Envelope<F> for Adsr<F>
where
    F: Float + Debug,
{
    fn reset(&mut self) {
        self.slope = Zero::zero();
        self.value = self.initial;
        self.note_is_on = false;
        self.state = EnvelopeState::Ready;
    }

    fn note_on(&mut self) {
        self.reset();
        self.note_is_on = true;
    }

    fn note_off(&mut self) {
        self.note_is_on = false;
        self.state = EnvelopeState::Release;
        self.slope = self.calc_release_slope();
    }

    fn set_attack(&mut self, value: F) {
        self.attack = nonzero(value);
        self.attack_slope = self.calc_attack_slope();
        if let EnvelopeState::Attack = self.state {
            self.slope = self.attack_slope;
        }
    }

    fn set_decay(&mut self, value: F) {
        self.decay = nonzero(value);
        self.decay_slope = self.calc_decay_slope();
        if let EnvelopeState::Decay = self.state {
            self.slope = self.decay_slope;
        }
    }

    fn set_sustain(&mut self, value: F) {
        self.sustain = normal(value);
    }

    fn set_release(&mut self, value: F) {
        self.release = nonzero(value);
        self.release_slope = self.calc_release_slope();
        if let EnvelopeState::Release = self.state {
            self.slope = self.release_slope;
        }
    }

    fn set_initial(&mut self, value: F) {
        self.initial = normal(value);
        if let EnvelopeState::Attack = self.state {
            self.attack_slope = self.calc_attack_slope();
        }
    }

    fn set_peak(&mut self, value: F) {
        self.peak = normal(value);
        if let EnvelopeState::Decay = self.state {
            self.decay_slope = self.calc_decay_slope();
        }
    }

    fn set_end(&mut self, value: F) {
        self.end = normal(value);
        if let EnvelopeState::Release = self.state {
            self.release_slope = self.calc_release_slope();
        }
    }

    fn is_done(&self) -> bool {
        if let EnvelopeState::Done = self.state {
            true
        } else {
            false
        }
    }
}

impl<F> Iterator for Adsr<F>
where
    F: Float,
    Self: Envelope<F>,
{
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        let zero = Zero::zero();
        match self.state {
            EnvelopeState::Ready => {
                if self.note_is_on {
                    self.state = EnvelopeState::Attack;
                    self.slope = self.calc_attack_slope();
                }
            }
            EnvelopeState::Attack => {
                if (self.initial > self.peak && self.value <= self.peak) ||
                    (self.initial <= self.peak && self.value >= self.peak)
                {
                    self.state = EnvelopeState::Decay;
                    self.slope = self.calc_decay_slope();
                }
            }
            EnvelopeState::Decay => {
                if (self.peak > self.sustain && self.value <= self.sustain) ||
                    (self.peak <= self.sustain && self.value >= self.sustain)
                {
                    self.state = EnvelopeState::Sustain;
                    self.slope = zero;
                }
            }
            EnvelopeState::Sustain => {
                if !self.note_is_on {
                    self.state = EnvelopeState::Release;
                    self.slope = self.calc_release_slope();
                } else {
                    self.value = self.sustain;
                }
            }
            EnvelopeState::Release => {
                if (self.end > self.sustain && self.value >= self.end) ||
                    (self.end <= self.sustain && self.value <= self.end)
                {
                    self.state = EnvelopeState::Done;
                    self.slope = zero;
                }
            }
            EnvelopeState::Done => self.reset(),
        }

        let mut out = None;
        if let EnvelopeState::Release = self.state {
            out = Some(self.value);
            self.value = self.value + self.slope;
        } else if self.note_is_on {
            out = Some(self.value);
            self.value = self.value + self.slope;
        }
        if self.value < zero {
            self.value = zero;
        }
        out
    }
}