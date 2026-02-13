use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

const SAMPLE_RATE: u32 = 44100;

pub struct AudioManager {
    wav_bytes: Vec<u8>,
    sound: Option<Sound>,
    status: String,
    started: bool,
    resumed: bool,
}

impl AudioManager {
    /// Generate WAV bytes on startup. Does NOT load into audio system yet.
    pub fn new() -> Self {
        let wav_bytes = render_test_tone();
        let len = wav_bytes.len();
        Self {
            wav_bytes,
            sound: None,
            status: format!("audio: TEST TONE wav ready ({} bytes), waiting for tap", len),
            started: false,
            resumed: false,
        }
    }

    /// Load and play. Call after first user interaction.
    /// Must be async because load_sound_from_bytes is async.
    pub async fn load_and_play(&mut self) {
        if self.started {
            return;
        }
        self.started = true;
        self.status = "audio: loading...".to_string();

        match load_sound_from_bytes(&self.wav_bytes).await {
            Ok(sound) => {
                play_sound(&sound, PlaySoundParams { looped: true, volume: 1.0 });
                self.sound = Some(sound);
                self.status = "audio: playing (tap again to resume)".to_string();
            }
            Err(e) => {
                self.status = format!("audio: load error: {}", e);
            }
        }
    }

    /// Call on every tap after audio has started, to trigger AudioContext resume.
    pub fn on_tap_after_start(&mut self) {
        if self.started && !self.resumed {
            if let Some(ref sound) = self.sound {
                // Re-issue play_sound on second tap — this happens in a direct
                // user gesture context, so the AudioContext resume listeners
                // in mq_js_bundle.js should fire.
                play_sound(sound, PlaySoundParams { looped: true, volume: 1.0 });
                self.resumed = true;
                self.status = "audio: playing (resumed)".to_string();
            }
        }
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn started(&self) -> bool {
        self.started
    }
}

/// Generate a simple 1-second 440Hz sine wave as WAV.
/// This is the simplest possible audio test — if this doesn't play,
/// the issue is in macroquad's audio pipeline on Safari, not our song data.
fn render_test_tone() -> Vec<u8> {
    let duration_secs = 1.0f32;
    let freq = 440.0f32;
    let num_samples = (duration_secs * SAMPLE_RATE as f32) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
        samples.push((sample * 32767.0) as i16);
    }

    encode_wav(&samples, SAMPLE_RATE)
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
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());   // PCM format
    wav.extend_from_slice(&1u16.to_le_bytes());   // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes());   // block align
    wav.extend_from_slice(&16u16.to_le_bytes());  // bits per sample

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
        assert_eq!(wav.len(), 244);
    }

    #[test]
    fn render_to_wav_produces_valid_wav() {
        let wav = render_to_wav();
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert!(wav.len() > 44);
    }

    #[test]
    fn render_to_wav_has_audible_content() {
        let wav = render_to_wav();
        let has_nonzero = wav[44..].chunks(2).any(|chunk| {
            i16::from_le_bytes([chunk[0], chunk[1]]) != 0
        });
        assert!(has_nonzero);
    }

    #[test]
    fn envelope_attack_starts_at_zero() {
        assert_eq!(envelope(0, 10000), 0.0);
    }

    #[test]
    fn envelope_sustain_is_one() {
        assert_eq!(envelope(5000, 10000), 1.0);
    }

    #[test]
    fn envelope_release_ends_near_zero() {
        assert!(envelope(9999, 10000) < 0.01);
    }
}
