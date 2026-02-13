use crate::song::{self, Note};
use web_sys::{AudioContext, OscillatorNode, OscillatorType, GainNode};

pub struct AudioManager {
    ctx: Option<AudioContext>,
    loop_duration: f32,
    next_loop_time: f64,
    started: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            ctx: None,
            loop_duration: song::loop_duration(),
            next_loop_time: 0.0,
            started: false,
        }
    }

    /// Call on first user interaction to create AudioContext and start music.
    pub fn start(&mut self) {
        if self.started {
            return;
        }
        let ctx = match AudioContext::new() {
            Ok(c) => c,
            Err(_) => return,
        };
        let start_time = ctx.current_time();
        self.schedule_loop(&ctx, start_time);
        self.next_loop_time = start_time + self.loop_duration as f64;
        self.ctx = Some(ctx);
        self.started = true;
    }

    /// Call each frame to schedule the next loop before the current one ends.
    pub fn update(&mut self) {
        let ctx = match &self.ctx {
            Some(c) => c,
            None => return,
        };
        // Schedule next loop 0.5s before current one finishes
        if ctx.current_time() >= self.next_loop_time - 0.5 {
            self.schedule_loop(ctx, self.next_loop_time);
            self.next_loop_time += self.loop_duration as f64;
        }
    }

    fn schedule_loop(&self, ctx: &AudioContext, start: f64) {
        self.schedule_channel(ctx, song::MELODY, OscillatorType::Square, start);
        self.schedule_channel(ctx, song::BASS, OscillatorType::Triangle, start);
        self.schedule_channel(ctx, song::DRUMS, OscillatorType::Square, start);
    }

    fn schedule_channel(
        &self,
        ctx: &AudioContext,
        notes: &[Note],
        wave: OscillatorType,
        start: f64,
    ) {
        let dest = ctx.destination();
        let mut time = start;

        for note in notes {
            if note.freq > 0.0 && note.volume > 0.0 {
                // Create oscillator + gain for each note
                if let (Ok(osc), Ok(gain)) = (
                    ctx.create_oscillator(),
                    ctx.create_gain(),
                ) {
                    osc.set_type(wave);
                    let _ = osc.frequency().set_value(note.freq);
                    let _ = gain.gain().set_value(note.volume);

                    // Envelope: quick attack, sustain, quick release to avoid clicks
                    let attack = 0.01;
                    let release = 0.02;
                    let _ = gain.gain().set_value_at_time(0.0, time);
                    let _ = gain.gain().linear_ramp_to_value_at_time(note.volume, time + attack);
                    let end = time + note.duration as f64;
                    let _ = gain.gain().set_value_at_time(note.volume, end - release);
                    let _ = gain.gain().linear_ramp_to_value_at_time(0.0, end);

                    let _ = osc.connect_with_audio_node(&gain);
                    let _ = gain.connect_with_audio_node(&dest);
                    let _ = osc.start_with_when(time);
                    let _ = osc.stop_with_when(end + 0.01);
                }
            }
            time += note.duration as f64;
        }
    }
}
