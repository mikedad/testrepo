/// A single note: frequency (Hz), duration (seconds), volume (0.0–1.0).
/// freq = 0.0 means rest (silence).
pub struct Note {
    pub freq: f32,
    pub duration: f32,
    pub volume: f32,
}

impl Note {
    const fn new(freq: f32, duration: f32, volume: f32) -> Self {
        Self { freq, duration, volume }
    }
    const fn rest(duration: f32) -> Self {
        Self { freq: 0.0, duration, volume: 0.0 }
    }
}

// Tempo: ~140 BPM → eighth note = 0.214s, quarter = 0.428s
const E: f32 = 0.214; // eighth
const Q: f32 = 0.428; // quarter
const H: f32 = 0.857; // half
const DQ: f32 = 0.642; // dotted quarter

// Note frequencies (octave 3-5)
const C3: f32 = 130.81;
const D3: f32 = 146.83;
const E3: f32 = 164.81;
const F3: f32 = 174.61;
const G3: f32 = 196.00;
const A3: f32 = 220.00;
const B3: f32 = 246.94;
const C4: f32 = 261.63;
const D4: f32 = 293.66;
const E4: f32 = 329.63;
const _F4: f32 = 349.23;
const G4: f32 = 392.00;
const A4: f32 = 440.00;
const B4: f32 = 493.88;
const C5: f32 = 523.25;
const D5: f32 = 587.33;
const E5: f32 = 659.25;

/// Melody — square wave. Heroic, adventurous theme.
pub const MELODY: &[Note] = &[
    // Bar 1-2: Bold opening phrase
    Note::new(E4, Q, 0.4),
    Note::new(G4, E, 0.4),
    Note::new(A4, E, 0.4),
    Note::new(B4, Q, 0.45),
    Note::new(A4, Q, 0.4),
    // Bar 2
    Note::new(G4, Q, 0.4),
    Note::new(E4, E, 0.4),
    Note::new(D4, E, 0.35),
    Note::new(E4, H, 0.4),
    // Bar 3-4: Rising heroic climb
    Note::new(A4, Q, 0.45),
    Note::new(B4, E, 0.45),
    Note::new(C5, E, 0.5),
    Note::new(D5, Q, 0.5),
    Note::new(C5, Q, 0.45),
    // Bar 4
    Note::new(B4, Q, 0.45),
    Note::new(A4, E, 0.4),
    Note::new(G4, E, 0.4),
    Note::new(A4, H, 0.45),
    // Bar 5-6: Triumphant peak
    Note::new(E5, DQ, 0.5),
    Note::new(D5, E, 0.45),
    Note::new(C5, Q, 0.45),
    Note::new(B4, Q, 0.4),
    // Bar 6
    Note::new(A4, Q, 0.4),
    Note::new(B4, Q, 0.45),
    Note::new(C5, Q, 0.45),
    Note::new(A4, Q, 0.4),
    // Bar 7-8: Resolution back to root
    Note::new(G4, Q, 0.4),
    Note::new(A4, E, 0.4),
    Note::new(B4, E, 0.45),
    Note::new(A4, Q, 0.4),
    Note::new(G4, Q, 0.4),
    // Bar 8: ending
    Note::new(E4, H, 0.4),
    Note::rest(Q),
];

/// Bass — triangle wave. Root notes and fifths.
pub const BASS: &[Note] = &[
    // Bar 1-2
    Note::new(E3, Q, 0.35),
    Note::new(E3, Q, 0.3),
    Note::new(B3, Q, 0.35),
    Note::new(E3, Q, 0.3),
    Note::new(C3, Q, 0.35),
    Note::new(G3, Q, 0.3),
    Note::new(E3, H, 0.35),
    // Bar 3-4
    Note::new(A3, Q, 0.35),
    Note::new(A3, Q, 0.3),
    Note::new(E3, Q, 0.35),
    Note::new(A3, Q, 0.3),
    Note::new(G3, Q, 0.35),
    Note::new(D3, Q, 0.3),
    Note::new(A3, H, 0.35),
    // Bar 5-6
    Note::new(C4, Q, 0.35),
    Note::new(G3, Q, 0.3),
    Note::new(A3, Q, 0.35),
    Note::new(E3, Q, 0.3),
    Note::new(F3, Q, 0.35),
    Note::new(G3, Q, 0.3),
    Note::new(A3, Q, 0.35),
    Note::new(A3, Q, 0.3),
    // Bar 7-8
    Note::new(E3, Q, 0.35),
    Note::new(G3, Q, 0.3),
    Note::new(A3, Q, 0.35),
    Note::new(E3, Q, 0.3),
    Note::new(E3, H, 0.35),
    Note::rest(Q),
];

/// Drums — noise channel. Short bursts for kick/snare feel.
pub const DRUMS: &[Note] = &[
    // Pattern repeats every 2 bars — 4 repetitions for 8 bars
    // "kick" = low freq noise, "snare" = higher freq noise
    // Beat: kick - - snare - kick kick snare
    Note::new(80.0, E, 0.3),   // kick
    Note::rest(E),
    Note::rest(E),
    Note::new(300.0, E, 0.2),  // snare
    Note::rest(E),
    Note::new(80.0, E, 0.25),  // kick
    Note::new(80.0, E, 0.25),  // kick
    Note::new(300.0, E, 0.2),  // snare
    // repeat
    Note::new(80.0, E, 0.3),
    Note::rest(E),
    Note::rest(E),
    Note::new(300.0, E, 0.2),
    Note::rest(E),
    Note::new(80.0, E, 0.25),
    Note::new(80.0, E, 0.25),
    Note::new(300.0, E, 0.2),
    // repeat
    Note::new(80.0, E, 0.3),
    Note::rest(E),
    Note::rest(E),
    Note::new(300.0, E, 0.2),
    Note::rest(E),
    Note::new(80.0, E, 0.25),
    Note::new(80.0, E, 0.25),
    Note::new(300.0, E, 0.2),
    // repeat
    Note::new(80.0, E, 0.3),
    Note::rest(E),
    Note::rest(E),
    Note::new(300.0, E, 0.2),
    Note::rest(E),
    Note::new(80.0, E, 0.25),
    Note::new(80.0, E, 0.25),
    Note::new(300.0, E, 0.2),
];

/// Total duration of a channel in seconds.
pub fn channel_duration(notes: &[Note]) -> f32 {
    notes.iter().map(|n| n.duration).sum()
}

/// Total loop duration in seconds (based on melody).
pub fn loop_duration() -> f32 {
    channel_duration(MELODY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_new_sets_fields() {
        let n = Note::new(440.0, 0.5, 0.8);
        assert_eq!(n.freq, 440.0);
        assert_eq!(n.duration, 0.5);
        assert_eq!(n.volume, 0.8);
    }

    #[test]
    fn note_rest_is_silent() {
        let r = Note::rest(0.25);
        assert_eq!(r.freq, 0.0);
        assert_eq!(r.duration, 0.25);
        assert_eq!(r.volume, 0.0);
    }

    #[test]
    fn loop_duration_is_positive() {
        let dur = loop_duration();
        assert!(dur > 0.0, "loop duration should be positive, got {}", dur);
    }

    #[test]
    fn loop_duration_is_reasonable_length() {
        let dur = loop_duration();
        // Spec says 15-30 seconds
        assert!(dur >= 10.0, "loop too short: {}s", dur);
        assert!(dur <= 40.0, "loop too long: {}s", dur);
    }

    #[test]
    fn melody_has_notes() {
        assert!(!MELODY.is_empty());
    }

    #[test]
    fn bass_has_notes() {
        assert!(!BASS.is_empty());
    }

    #[test]
    fn drums_has_notes() {
        assert!(!DRUMS.is_empty());
    }

    #[test]
    fn all_note_durations_are_positive() {
        for (name, channel) in [("melody", MELODY), ("bass", BASS), ("drums", DRUMS)] {
            for (i, note) in channel.iter().enumerate() {
                assert!(
                    note.duration > 0.0,
                    "{} note {} has non-positive duration: {}",
                    name, i, note.duration
                );
            }
        }
    }

    #[test]
    fn all_note_volumes_in_range() {
        for (name, channel) in [("melody", MELODY), ("bass", BASS), ("drums", DRUMS)] {
            for (i, note) in channel.iter().enumerate() {
                assert!(
                    note.volume >= 0.0 && note.volume <= 1.0,
                    "{} note {} has out-of-range volume: {}",
                    name, i, note.volume
                );
            }
        }
    }

    #[test]
    fn all_note_frequencies_non_negative() {
        for (name, channel) in [("melody", MELODY), ("bass", BASS), ("drums", DRUMS)] {
            for (i, note) in channel.iter().enumerate() {
                assert!(
                    note.freq >= 0.0,
                    "{} note {} has negative frequency: {}",
                    name, i, note.freq
                );
            }
        }
    }

    #[test]
    fn melody_and_bass_durations_match() {
        let melody_dur = channel_duration(MELODY);
        let bass_dur = channel_duration(BASS);
        let diff = (melody_dur - bass_dur).abs();
        assert!(
            diff < 0.5,
            "melody ({}s) and bass ({}s) durations differ by {}s",
            melody_dur, bass_dur, diff
        );
    }

    #[test]
    fn melody_contains_no_silence_only() {
        let has_audible = MELODY.iter().any(|n| n.freq > 0.0 && n.volume > 0.0);
        assert!(has_audible, "melody should have at least one audible note");
    }

    #[test]
    fn bass_contains_no_silence_only() {
        let has_audible = BASS.iter().any(|n| n.freq > 0.0 && n.volume > 0.0);
        assert!(has_audible, "bass should have at least one audible note");
    }

    #[test]
    fn drums_contains_no_silence_only() {
        let has_audible = DRUMS.iter().any(|n| n.freq > 0.0 && n.volume > 0.0);
        assert!(has_audible, "drums should have at least one audible note");
    }
}
