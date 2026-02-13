use crate::song::{self, Note};
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

const SAMPLE_RATE: u32 = 44100;

pub struct AudioManager {
    sound: Option<Sound>,
    playing: bool,
}

impl AudioManager {
    pub async fn new() -> Self {
        let wav = render_to_wav();
        let sound = match load_sound_from_bytes(&wav).await {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("Failed to load generated audio: {}", e);
                None
            }
        };
        Self { sound, playing: false }
    }

    /// Start playing on first user interaction (browser autoplay policy).
    pub fn play(&mut self) {
        if self.playing {
            return;
        }
        if let Some(ref sound) = self.sound {
            play_sound(sound, PlaySoundParams { looped: true, volume: 0.6 });
            self.playing = true;
        }
    }
}

/// Render the intro song from note data into a complete WAV byte buffer.
fn render_to_wav() -> Vec<u8> {
    let duration = song::loop_duration();
    let num_samples = (duration * SAMPLE_RATE as f32) as usize;
    let mut buffer = vec![0.0f32; num_samples];

    // Render each channel and mix into buffer
    render_channel(&mut buffer, song::MELODY, Waveform::Square);
    render_channel(&mut buffer, song::BASS, Waveform::Triangle);
    render_channel(&mut buffer, song::DRUMS, Waveform::Noise);

    // Convert f32 mix to i16 samples
    let mut samples = Vec::with_capacity(num_samples);
    for &s in &buffer {
        let clamped = s.clamp(-1.0, 1.0);
        samples.push((clamped * 32767.0) as i16);
    }

    // Build WAV file
    encode_wav(&samples, SAMPLE_RATE)
}

#[derive(Clone, Copy)]
enum Waveform {
    Square,
    Triangle,
    Noise,
}

fn render_channel(buffer: &mut [f32], notes: &[Note], wave: Waveform) {
    let mut sample_pos: usize = 0;
    let mut noise_state: u32 = 0xDEAD_BEEF; // LFSR seed for noise

    for note in notes {
        let note_samples = (note.duration * SAMPLE_RATE as f32) as usize;

        if note.freq > 0.0 && note.volume > 0.0 {
            let period = SAMPLE_RATE as f32 / note.freq;

            for i in 0..note_samples {
                let idx = sample_pos + i;
                if idx >= buffer.len() {
                    break;
                }

                // Envelope: 10ms attack, 20ms release to avoid clicks
                let env = envelope(i, note_samples);

                let sample = match wave {
                    Waveform::Square => {
                        let phase = (i as f32 % period) / period;
                        if phase < 0.5 { note.volume } else { -note.volume }
                    }
                    Waveform::Triangle => {
                        let phase = (i as f32 % period) / period;
                        let tri = if phase < 0.5 {
                            4.0 * phase - 1.0
                        } else {
                            3.0 - 4.0 * phase
                        };
                        tri * note.volume
                    }
                    Waveform::Noise => {
                        // Linear feedback shift register for pseudo-random noise
                        noise_state ^= noise_state << 13;
                        noise_state ^= noise_state >> 17;
                        noise_state ^= noise_state << 5;
                        let noise = (noise_state as f32 / u32::MAX as f32) * 2.0 - 1.0;
                        noise * note.volume
                    }
                };

                buffer[idx] += sample * env;
            }
        }

        sample_pos += note_samples;
    }
}

/// Simple attack/release envelope to avoid clicks.
fn envelope(sample: usize, total: usize) -> f32 {
    let attack_samples = (0.01 * SAMPLE_RATE as f32) as usize; // 10ms
    let release_samples = (0.02 * SAMPLE_RATE as f32) as usize; // 20ms

    if sample < attack_samples {
        sample as f32 / attack_samples as f32
    } else if sample > total.saturating_sub(release_samples) {
        let remaining = total - sample;
        remaining as f32 / release_samples as f32
    } else {
        1.0
    }
}

/// Encode i16 samples as a WAV file (44100 Hz, 16-bit, mono).
fn encode_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let data_size = (samples.len() * 2) as u32;
    let file_size = 36 + data_size;

    let mut wav = Vec::with_capacity(44 + data_size as usize);

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes()); // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for &sample in samples {
        wav.extend_from_slice(&sample.to_le_bytes());
    }

    wav
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_header_is_valid() {
        let samples = vec![0i16; 100];
        let wav = encode_wav(&samples, 44100);

        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(&wav[36..40], b"data");
    }

    #[test]
    fn wav_size_is_correct() {
        let samples = vec![0i16; 100];
        let wav = encode_wav(&samples, 44100);

        // 44 byte header + 200 bytes data
        assert_eq!(wav.len(), 244);
    }

    #[test]
    fn render_to_wav_produces_valid_wav() {
        let wav = render_to_wav();

        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert!(wav.len() > 44, "WAV should have data beyond header");
    }

    #[test]
    fn render_to_wav_has_audible_content() {
        let wav = render_to_wav();

        // Check that not all samples are zero (skip 44-byte header)
        let has_nonzero = wav[44..].chunks(2).any(|chunk| {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            sample != 0
        });
        assert!(has_nonzero, "rendered audio should contain non-silent samples");
    }

    #[test]
    fn envelope_attack_starts_at_zero() {
        assert_eq!(envelope(0, 10000), 0.0);
    }

    #[test]
    fn envelope_sustain_is_one() {
        let mid = 5000;
        assert_eq!(envelope(mid, 10000), 1.0);
    }

    #[test]
    fn envelope_release_ends_near_zero() {
        let total = 10000;
        let near_end = total - 1;
        assert!(envelope(near_end, total) < 0.01);
    }
}
