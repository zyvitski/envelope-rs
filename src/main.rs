mod envelope;
use envelope::Envelope;
use std::time::{Instant, Duration};

fn main() {
    const SAMPLE_RATE: f64 = 44100.0;
    let attack = 0.05 * SAMPLE_RATE;
    let decay = 0.05 * SAMPLE_RATE;
    let sustain = 0.75;
    let release = 0.05 * SAMPLE_RATE;
    let inital = 0.0;
    let peak = 1.0;
    let end = 0.0;

    let mut envelope = Envelope::new(attack, decay, sustain, release, inital, peak, end);
    envelope.note_on();

    let now = Instant::now();
    let mut counter = 0;
    while let Some(_) = envelope.next() {
        println!("{}: {:?}", counter, envelope);
        counter += 1;
        if now.elapsed() >= Duration::from_millis(50) {
            envelope.note_off();
        }
        if envelope.is_done() {
            break;
        }
    }
}
