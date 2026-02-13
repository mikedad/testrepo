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

/// Total loop duration in seconds (sum of melody durations).
pub fn loop_duration() -> f32 {
    MELODY.iter().map(|n| n.duration).sum()
}
