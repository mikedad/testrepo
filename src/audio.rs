use crate::song::{self, Note};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

pub struct AudioManager {
    _stream: Option<Stream>,
}

impl AudioManager {
    /// Create and immediately start the audio stream.
    pub fn start() -> Self {
        let stream = match build_stream() {
            Some(s) => {
                let _ = s.play();
                Some(s)
            }
            None => {
                eprintln!("Failed to start audio stream");
                None
            }
        };
        Self { _stream: stream }
    }
}

fn build_stream() -> Option<Stream> {
    let host = cpal::default_host();
    let device = host.default_output_device()?;
    let config = device.default_output_config().ok()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    let mut synth = SynthState::new(sample_rate);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let sample = synth.next_sample();
                    for s in frame.iter_mut() {
                        *s = sample;
                    }
                }
            },
            |err| eprintln!("audio stream error: {}", err),
            None,
        )
        .ok()?;

    Some(stream)
}

/// Tracks playback position for one channel.
struct ChannelState {
    note_idx: usize,
    sample_in_note: usize,
    noise_state: u32,
}

impl ChannelState {
    fn new() -> Self {
        Self {
            note_idx: 0,
            sample_in_note: 0,
            noise_state: 0xDEAD_BEEF,
        }
    }

    /// Generate one sample for this channel and advance position.
    fn next_sample(&mut self, notes: &[Note], wave: Waveform, sample_rate: f32) -> f32 {
        if notes.is_empty() {
            return 0.0;
        }

        let note = &notes[self.note_idx];
        let note_samples = (note.duration * sample_rate) as usize;

        let sample = if note.freq > 0.0 && note.volume > 0.0 {
            let env = envelope(self.sample_in_note, note_samples, sample_rate);
            let period = sample_rate / note.freq;
            let phase = (self.sample_in_note as f32 % period) / period;

            let raw = match wave {
                Waveform::Square => {
                    if phase < 0.5 { note.volume } else { -note.volume }
                }
                Waveform::Triangle => {
                    let tri = if phase < 0.5 {
                        4.0 * phase - 1.0
                    } else {
                        3.0 - 4.0 * phase
                    };
                    tri * note.volume
                }
                Waveform::Noise => {
                    self.noise_state ^= self.noise_state << 13;
                    self.noise_state ^= self.noise_state >> 17;
                    self.noise_state ^= self.noise_state << 5;
                    let noise = (self.noise_state as f32 / u32::MAX as f32) * 2.0 - 1.0;
                    noise * note.volume
                }
            };
            raw * env
        } else {
            0.0
        };

        // Advance position
        self.sample_in_note += 1;
        if self.sample_in_note >= note_samples {
            self.sample_in_note = 0;
            self.note_idx += 1;
            if self.note_idx >= notes.len() {
                self.note_idx = 0; // loop
            }
        }

        sample
    }
}

#[derive(Clone, Copy)]
enum Waveform {
    Square,
    Triangle,
    Noise,
}

struct SynthState {
    melody: ChannelState,
    bass: ChannelState,
    drums: ChannelState,
    sample_rate: f32,
}

impl SynthState {
    fn new(sample_rate: f32) -> Self {
        Self {
            melody: ChannelState::new(),
            bass: ChannelState::new(),
            drums: ChannelState::new(),
            sample_rate,
        }
    }

    fn next_sample(&mut self) -> f32 {
        let m = self.melody.next_sample(song::MELODY, Waveform::Square, self.sample_rate);
        let b = self.bass.next_sample(song::BASS, Waveform::Triangle, self.sample_rate);
        let d = self.drums.next_sample(song::DRUMS, Waveform::Noise, self.sample_rate);
        (m + b + d).clamp(-1.0, 1.0)
    }
}

/// Per-note envelope: 10ms attack, 20ms release.
fn envelope(sample: usize, total: usize, sample_rate: f32) -> f32 {
    let attack = (0.01 * sample_rate) as usize;
    let release = (0.02 * sample_rate) as usize;

    if sample < attack {
        sample as f32 / attack as f32
    } else if sample > total.saturating_sub(release) {
        let remaining = total - sample;
        remaining as f32 / release as f32
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_attack_starts_at_zero() {
        assert_eq!(envelope(0, 10000, 44100.0), 0.0);
    }

    #[test]
    fn envelope_sustain_is_one() {
        assert_eq!(envelope(5000, 10000, 44100.0), 1.0);
    }

    #[test]
    fn envelope_release_ends_near_zero() {
        assert!(envelope(9999, 10000, 44100.0) < 0.01);
    }

    #[test]
    fn synth_produces_nonzero_samples() {
        let mut synth = SynthState::new(44100.0);
        let has_nonzero = (0..44100).any(|_| synth.next_sample() != 0.0);
        assert!(has_nonzero, "synth should produce audible samples");
    }

    #[test]
    fn synth_output_stays_in_range() {
        let mut synth = SynthState::new(44100.0);
        for _ in 0..44100 {
            let s = synth.next_sample();
            assert!(s >= -1.0 && s <= 1.0, "sample out of range: {}", s);
        }
    }

    #[test]
    fn channel_loops_back_to_start() {
        let mut ch = ChannelState::new();
        let notes = &[Note { freq: 440.0, duration: 0.01, volume: 0.5 }];
        let samples_per_note = (0.01 * 44100.0) as usize;
        // Play through the single note twice
        for _ in 0..(samples_per_note * 2 + 10) {
            ch.next_sample(notes, Waveform::Square, 44100.0);
        }
        // Should have looped — note_idx back to 0
        assert_eq!(ch.note_idx, 0);
    }
}
