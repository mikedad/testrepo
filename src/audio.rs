use crate::song::{self, Note};

extern "C" {
    fn sapp_js_eval(js: *const u8, len: usize);
}

fn eval_js(code: &str) {
    unsafe { sapp_js_eval(code.as_ptr(), code.len()) };
}

pub struct AudioManager {
    started: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self { started: false }
    }

    /// Call on first user interaction to create AudioContext and start music.
    pub fn start(&mut self) {
        if self.started {
            return;
        }
        eval_js("window._dc_audio = new AudioContext();");
        self.schedule_channel(song::MELODY, "square");
        self.schedule_channel(song::BASS, "triangle");
        self.schedule_channel(song::DRUMS, "square");
        // Schedule looping
        let loop_dur_ms = (song::loop_duration() * 1000.0) as u32;
        let js = format!(
            "window._dc_loop = setInterval(function() {{ {} {} {} }}, {});",
            Self::channel_js(song::MELODY, "square"),
            Self::channel_js(song::BASS, "triangle"),
            Self::channel_js(song::DRUMS, "square"),
            loop_dur_ms
        );
        eval_js(&js);
        self.started = true;
    }

    pub fn update(&mut self) {
        // Looping handled by JS setInterval
    }

    fn schedule_channel(&self, notes: &[Note], wave: &str) {
        eval_js(&Self::channel_js(notes, wave));
    }

    fn channel_js(notes: &[Note], wave: &str) -> String {
        let mut js = String::with_capacity(512);
        js.push_str("(function(){var c=window._dc_audio;if(!c)return;var t=c.currentTime+0.05;");

        for note in notes {
            if note.freq > 0.0 && note.volume > 0.0 {
                js.push_str(&format!(
                    "var o=c.createOscillator();var g=c.createGain();\
                     o.type='{}';o.frequency.value={:.1};\
                     g.gain.setValueAtTime(0,t);\
                     g.gain.linearRampToValueAtTime({:.2},t+0.01);\
                     g.gain.setValueAtTime({:.2},t+{:.3}-0.02);\
                     g.gain.linearRampToValueAtTime(0,t+{:.3});\
                     o.connect(g);g.connect(c.destination);\
                     o.start(t);o.stop(t+{:.3});",
                    wave, note.freq,
                    note.volume, note.volume,
                    note.duration, note.duration, note.duration
                ));
            }
            js.push_str(&format!("t+={:.3};", note.duration));
        }
        js.push_str("})();");
        js
    }
}
