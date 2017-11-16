#![allow(dead_code)]

extern crate num;
use num::{Float, clamp, One, Zero, NumCast};

fn nonzero<F>(value: F) -> F
where
    F: Float,
{
    use std::f64::{MIN_POSITIVE, MAX};
    clamp(
        value,
        NumCast::from(MIN_POSITIVE).unwrap(),
        NumCast::from(MAX).unwrap(),
    )
}

fn normal<F>(value: F) -> F
where
    F: Float,
{
    clamp(value, Zero::zero(), One::one())
}

#[derive(Debug)]
enum AdsrState {
    Ready,
    Attack,
    Decay,
    Sustain,
    Release,
    Done,
}

#[derive(Debug)]
pub struct Envelope<F>
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
    state: AdsrState,
    slope: F,
    value: F,
    note_is_on: bool,
    attack_slope: F,
    decay_slope: F,
    release_slope: F,
}

impl<F> Envelope<F>
where
    F: Float,
{
    pub fn new(attack: F, decay: F, sustain: F, release: F, initial: F, peak: F, end: F) -> Self {
        let zero: F = Zero::zero();
        Envelope {
            attack: nonzero(attack),
            decay: nonzero(decay),
            sustain: normal(sustain),
            release: nonzero(release),
            initial: normal(initial),
            peak: normal(peak),
            end: normal(end),
            state: AdsrState::Ready,
            slope: zero,
            value: initial,
            note_is_on: false,
            attack_slope: zero,
            decay_slope: zero,
            release_slope: zero,
        }
    }

    pub fn reset(&mut self) {
        self.slope = Zero::zero();
        self.value = self.initial;
        self.note_is_on = false;
        self.state = AdsrState::Ready;
    }

    pub fn note_on(&mut self) {
        self.reset();
        self.note_is_on = true;
    }

    pub fn note_off(&mut self) {
        self.note_is_on = false;
        self.state = AdsrState::Release;
        self.slope = self.calc_release_slope();
    }

    pub fn set_attack(&mut self, value: F) {
        self.attack = nonzero(value);
        self.attack_slope = self.calc_attack_slope();
        if let AdsrState::Attack = self.state {
            self.slope = self.attack_slope;
        }
    }

    pub fn set_decay(&mut self, value: F) {
        self.decay = nonzero(value);
        self.decay_slope = self.calc_decay_slope();
        if let AdsrState::Decay = self.state {
            self.slope = self.decay_slope;
        }
    }

    pub fn set_sustain(&mut self, value: F) {
        self.sustain = normal(value);
    }

    pub fn set_release(&mut self, value: F) {
        self.release = nonzero(value);
        self.release_slope = self.calc_release_slope();
        if let AdsrState::Release = self.state {
            self.slope = self.release_slope;
        }
    }

    pub fn set_initial(&mut self, value: F) {
        self.initial = normal(value);
        if let AdsrState::Attack = self.state {
            self.attack_slope = self.calc_attack_slope();
        }
    }

    pub fn set_peak(&mut self, value: F) {
        self.peak = normal(value);
        if let AdsrState::Decay = self.state {
            self.decay_slope = self.calc_decay_slope();
        }
    }

    pub fn set_end(&mut self, value: F) {
        self.end = normal(value);
        if let AdsrState::Release = self.state {
            self.release_slope = self.calc_release_slope();
        }
    }

    pub fn is_done(&self) -> bool {
        if let AdsrState::Done = self.state {
            true
        } else {
            false
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

impl<F> Iterator for Envelope<F>
where
    F: Float,
{
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        let zero = Zero::zero();
        match self.state {
            AdsrState::Ready => {
                if self.note_is_on {
                    self.state = AdsrState::Attack;
                    self.slope = self.calc_attack_slope();
                }
            }
            AdsrState::Attack => {
                if (self.initial > self.peak && self.value <= self.peak) ||
                    (self.initial <= self.peak && self.value >= self.peak)
                {
                    self.state = AdsrState::Decay;
                    self.slope = self.calc_decay_slope();
                }
            }
            AdsrState::Decay => {
                if (self.peak > self.sustain && self.value <= self.sustain) ||
                    (self.peak <= self.sustain && self.value >= self.sustain)
                {
                    self.state = AdsrState::Sustain;
                    self.slope = zero;
                }
            }
            AdsrState::Sustain => {
                if !self.note_is_on {
                    self.state = AdsrState::Release;
                    self.slope = self.calc_release_slope();
                } else {
                    self.value = self.sustain;
                }
            }
            AdsrState::Release => {
                if (self.end > self.sustain && self.value >= self.end) ||
                    (self.end <= self.sustain && self.value <= self.end)
                {
                    self.state = AdsrState::Done;
                    self.slope = zero;
                }
            }
            AdsrState::Done => self.reset(),
        }

        let mut out = None;
        if let AdsrState::Release = self.state {
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
