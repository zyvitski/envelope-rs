#![allow(dead_code)]
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
pub struct Envelope {
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
    pub initial: f64,
    pub peak: f64,
    end: f64,
    state: AdsrState,
    slope: f64,
    value: f64,
    note_is_on: bool,
    attack_slope: f64,
    decay_slope: f64,
    release_slope: f64,
}

impl Envelope {
    pub fn new(
        attack: f64,
        decay: f64,
        sustain: f64,
        release: f64,
        initial: f64,
        peak: f64,
        end: f64,
    ) -> Self {
        Envelope {
            attack: make_nonzero(attack),
            decay: make_nonzero(decay),
            sustain: bound_0_1(sustain),
            release: make_nonzero(release),
            initial: bound_0_1(initial),
            peak: bound_0_1(peak),
            end: bound_0_1(end),
            state: AdsrState::Ready,
            slope: 0.0,
            value: initial,
            note_is_on: false,
            attack_slope: 0.0,
            decay_slope: 0.0,
            release_slope: 0.0,
        }
    }
    pub fn reset(&mut self) {
        self.slope = 0.0;
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
    pub fn set_attack(&mut self, value: f64) {
        self.attack = make_nonzero(value);
        self.attack_slope = self.calc_attack_slope();
        if let AdsrState::Attack = self.state {
            self.slope = self.attack_slope;
        }
    }
    pub fn set_decay(&mut self, value: f64) {
        self.decay = make_nonzero(value);
        self.decay_slope = self.calc_decay_slope();
        if let AdsrState::Decay = self.state {
            self.slope = self.decay_slope;
        }
    }
    pub fn set_sustain(&mut self, value: f64) {
        self.sustain = bound_0_1(value);
    }
    pub fn set_release(&mut self, value: f64) {
        self.release = make_nonzero(value);
        self.release_slope = self.calc_release_slope();
        if let AdsrState::Release = self.state {
            self.slope = self.release_slope;
        }
    }

    pub fn set_initial(&mut self, value: f64) {
        self.initial = bound_0_1(value);
        if let AdsrState::Attack = self.state {
            self.attack_slope = self.calc_attack_slope();
        }
    }
    pub fn set_peak(&mut self, value: f64) {
        self.peak = bound_0_1(value);
        if let AdsrState::Decay = self.state {
            self.decay_slope = self.calc_decay_slope();
        }
    }
    pub fn set_end(&mut self, value: f64) {
        self.end = bound_0_1(value);
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

    fn calc_attack_slope(&mut self) -> f64 {
        (self.peak - self.initial) / self.attack
    }
    fn calc_decay_slope(&mut self) -> f64 {
        (self.sustain - self.peak) / self.decay
    }
    fn calc_release_slope(&mut self) -> f64 {
        (self.end - self.sustain) / self.release
    }
}

fn make_nonzero(value: f64) -> f64 {
    use std::f64::MIN_POSITIVE;
    if value <= MIN_POSITIVE {
        MIN_POSITIVE
    } else {
        value
    }
}

fn bound_0_1(value: f64) -> f64 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

impl Iterator for Envelope {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
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
                    self.slope = 0.0;
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
                    self.slope = 0.0;
                }
            }
            AdsrState::Done => self.reset(),
        }
        let mut out = None;
        if let AdsrState::Release = self.state {
            out = Some(self.value);
            self.value += self.slope;
        } else if self.note_is_on {
            out = Some(self.value);
            self.value += self.slope;
        }
        if self.value < 0.0 {
            self.value = 0.0
        }
        out
    }
}