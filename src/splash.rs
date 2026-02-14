use macroquad::prelude::*;

// Colors
const BG_COLOR: Color = Color::new(0.102, 0.102, 0.180, 1.0); // #1a1a2e
const GOLD: Color = Color::new(1.0, 0.843, 0.0, 1.0);
const SILVER: Color = Color::new(0.753, 0.753, 0.753, 1.0);
const BLUE_CAPE: Color = Color::new(0.255, 0.412, 0.882, 1.0);
const MONSTER_GREEN: Color = Color::new(0.176, 0.353, 0.153, 1.0);
const MONSTER_GREEN_LIGHT: Color = Color::new(0.3, 0.5, 0.25, 1.0);
const RED_EYES: Color = Color::new(1.0, 0.0, 0.0, 1.0);
const CHEST_BROWN: Color = Color::new(0.545, 0.271, 0.075, 1.0);
const CHEST_BROWN_DARK: Color = Color::new(0.4, 0.2, 0.05, 1.0);
const SKIN: Color = Color::new(0.87, 0.72, 0.53, 1.0);

pub struct SplashScreen {
    time: f32,
    user_interacted: bool,
    swing_timer: f32, // >0 means sword is swinging
}

const SWING_DURATION: f32 = 0.5;

impl SplashScreen {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            user_interacted: false,
            swing_timer: 0.0,
        }
    }

    pub fn user_interacted(&self) -> bool {
        self.user_interacted
    }

    pub fn update(&mut self) {
        self.time += get_frame_time();

        // Tick down sword swing
        if self.swing_timer > 0.0 {
            self.swing_timer -= get_frame_time();
        }

        // Touch, click, or key — any interaction counts (iPad + desktop)
        if is_mouse_button_pressed(MouseButton::Left)
            || touches().iter().any(|t| t.phase == TouchPhase::Started)
            || get_last_key_pressed().is_some()
        {
            self.user_interacted = true;
            self.swing_timer = SWING_DURATION;
        }
    }

    pub fn draw(&self) {
        clear_background(BG_COLOR);

        let sw = screen_width();
        let sh = screen_height();
        let px = (sw / 320.0).max(1.0); // pixel scale unit

        self.draw_title(sw, sh, px);
        self.draw_warrior(sw * 0.2, sh * 0.55, px);
        self.draw_monster(sw * 0.5, sh * 0.55, px);
        self.draw_treasure(sw * 0.8, sh * 0.55, px);
        self.draw_build_timestamp(sw, sh);
    }

    /// Draw the progress bar for dungeon generation.
    pub fn draw_progress_bar(&self, progress: f32) {
        let sw = screen_width();
        let sh = screen_height();
        let px = (sw / 320.0).max(1.0);

        let bar_y = sh * 0.72;

        // Label
        let label = "Generating Level...";
        let font_size = (px * 6.0).max(12.0);
        let dims = measure_text(label, None, font_size as u16, 1.0);
        draw_text(label, (sw - dims.width) / 2.0, bar_y - px * 4.0, font_size, SILVER);

        // Bar background
        let bar_w = px * 100.0;
        let bar_h = px * 6.0;
        let bar_x = (sw - bar_w) / 2.0;

        // Dark background
        draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.1, 0.1, 0.15, 1.0));
        // Fill
        let fill_w = bar_w * progress.clamp(0.0, 1.0);
        draw_rectangle(bar_x, bar_y, fill_w, bar_h, GOLD);
        // Border
        draw_rectangle(bar_x, bar_y, bar_w, 1.0, Color::new(0.5, 0.5, 0.5, 1.0));
        draw_rectangle(bar_x, bar_y + bar_h - 1.0, bar_w, 1.0, Color::new(0.5, 0.5, 0.5, 1.0));
        draw_rectangle(bar_x, bar_y, 1.0, bar_h, Color::new(0.5, 0.5, 0.5, 1.0));
        draw_rectangle(bar_x + bar_w - 1.0, bar_y, 1.0, bar_h, Color::new(0.5, 0.5, 0.5, 1.0));

        // Percentage text
        let pct_text = format!("{}%", (progress * 100.0) as u32);
        let pct_size = (px * 5.0).max(10.0);
        let pct_dims = measure_text(&pct_text, None, pct_size as u16, 1.0);
        let text_color = if progress > 0.5 {
            Color::new(0.1, 0.1, 0.1, 1.0)
        } else {
            SILVER
        };
        draw_text(
            &pct_text,
            (sw - pct_dims.width) / 2.0,
            bar_y + bar_h / 2.0 + pct_dims.height / 2.0,
            pct_size,
            text_color,
        );
    }

    /// Draw the "PLAY!" button with pulse animation.
    pub fn draw_play_button(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let px = (sw / 320.0).max(1.0);

        let btn_w = px * 40.0;
        let btn_h = px * 12.0;
        let btn_x = (sw - btn_w) / 2.0;
        let btn_y = sh * 0.72;

        // Glow pulse
        let pulse = (self.time * 3.0).sin() * 0.15 + 1.0;
        let glow_expand = px * 2.0 * pulse;
        let glow_alpha = (self.time * 2.0).sin() * 0.2 + 0.3;
        draw_rectangle(
            btn_x - glow_expand,
            btn_y - glow_expand,
            btn_w + glow_expand * 2.0,
            btn_h + glow_expand * 2.0,
            Color::new(1.0, 0.843, 0.0, glow_alpha),
        );

        // Button background
        draw_rectangle(btn_x, btn_y, btn_w, btn_h, Color::new(0.15, 0.15, 0.25, 1.0));
        // Border
        draw_rectangle(btn_x, btn_y, btn_w, 2.0, GOLD);
        draw_rectangle(btn_x, btn_y + btn_h - 2.0, btn_w, 2.0, GOLD);
        draw_rectangle(btn_x, btn_y, 2.0, btn_h, GOLD);
        draw_rectangle(btn_x + btn_w - 2.0, btn_y, 2.0, btn_h, GOLD);

        // Text
        let text = "PLAY!";
        let font_size = (px * 10.0).max(20.0);
        let dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
            text,
            btn_x + (btn_w - dims.width) / 2.0,
            btn_y + (btn_h + dims.height) / 2.0,
            font_size,
            GOLD,
        );
    }

    /// Check if the Play button was tapped/clicked this frame.
    pub fn check_play_button_tap(&self) -> bool {
        let sw = screen_width();
        let sh = screen_height();
        let px = (sw / 320.0).max(1.0);

        let btn_w = px * 40.0;
        let btn_h = px * 12.0;
        let btn_x = (sw - btn_w) / 2.0;
        let btn_y = sh * 0.72;

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                return true;
            }
        }

        for touch in touches() {
            if touch.phase == TouchPhase::Started {
                let tx = touch.position.x;
                let ty = touch.position.y;
                if tx >= btn_x && tx <= btn_x + btn_w && ty >= btn_y && ty <= btn_y + btn_h {
                    return true;
                }
            }
        }

        false
    }

    fn draw_title(&self, sw: f32, sh: f32, px: f32) {
        let title = "DUNGEON CREATOR";
        let font_size = (px * 12.0) as u16;
        let dims = measure_text(title, None, font_size, 1.0);
        let x = (sw - dims.width) / 2.0;
        let y = sh * 0.18;

        // Glow effect: oscillating brightness behind text
        let glow = ((self.time * 2.0).sin() * 0.3 + 0.7) as f32;
        let glow_color = Color::new(1.0, 0.843, 0.0, glow * 0.3);
        for offset in [2.0, 4.0, 6.0] {
            let o = offset * px;
            draw_text(title, x - o, y - o, font_size as f32, glow_color);
            draw_text(title, x + o, y - o, font_size as f32, glow_color);
            draw_text(title, x - o, y + o, font_size as f32, glow_color);
            draw_text(title, x + o, y + o, font_size as f32, glow_color);
        }

        // Shadow
        draw_text(title, x + px, y + px, font_size as f32, Color::new(0.3, 0.2, 0.0, 1.0));
        // Main title
        draw_text(title, x, y, font_size as f32, GOLD);
    }

    fn draw_warrior(&self, cx: f32, cy: f32, px: f32) {
        let breathe = (self.time * 2.5).sin() * px * 0.5;
        let s = px * 3.0; // block size

        // Helmet
        draw_rectangle(cx - s * 1.5, cy - s * 7.0 + breathe, s * 3.0, s, SILVER);
        draw_rectangle(cx - s, cy - s * 7.0 + breathe - s * 0.5, s * 2.0, s * 0.5, SILVER);
        // Helmet visor
        draw_rectangle(cx - s * 0.5, cy - s * 6.0 + breathe, s, s * 0.5, Color::new(0.2, 0.2, 0.3, 1.0));

        // Head/face
        draw_rectangle(cx - s, cy - s * 6.0 + breathe, s * 2.0, s * 1.5, SKIN);

        // Body (armor)
        draw_rectangle(cx - s * 1.5, cy - s * 4.5 + breathe, s * 3.0, s * 3.0, SILVER);
        // Armor detail line
        draw_rectangle(cx - s * 0.25, cy - s * 4.5 + breathe, s * 0.5, s * 3.0, Color::new(0.6, 0.6, 0.65, 1.0));

        // Cape (behind, left side)
        draw_rectangle(cx - s * 2.0, cy - s * 4.5 + breathe, s * 0.5, s * 4.5, BLUE_CAPE);
        draw_rectangle(cx - s * 2.5, cy - s * 3.0 + breathe, s * 0.5, s * 3.5, BLUE_CAPE);

        // Arms
        draw_rectangle(cx - s * 2.0, cy - s * 4.0 + breathe, s * 0.5, s * 2.5, SKIN);
        draw_rectangle(cx + s * 1.5, cy - s * 4.0 + breathe, s * 0.5, s * 2.5, SKIN);

        // Sword (right hand) — swings forward on tap
        let swing_progress = if self.swing_timer > 0.0 {
            // 0→1→0 over SWING_DURATION (forward then back)
            let t = 1.0 - (self.swing_timer / SWING_DURATION);
            if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 }
        } else {
            0.0
        };

        // Pivot point is at the warrior's hand
        let pivot_x = cx + s * 1.5;
        let pivot_y = cy - s * 1.5 + breathe;

        if swing_progress > 0.01 {
            // Swinging: sword goes from vertical to horizontal
            // Horizontal sword (pointing right)
            let blade_len = s * 4.5;
            let offset_x = blade_len * swing_progress;
            let offset_y = -blade_len * (1.0 - swing_progress);
            // Blade (horizontal when fully swung)
            draw_rectangle(
                pivot_x,
                pivot_y + offset_y,
                s * 0.4 + offset_x * 0.8,
                s * 0.4 + (-offset_y) * 0.1,
                SILVER,
            );
            // Flash effect at peak swing
            if swing_progress > 0.8 {
                draw_rectangle(pivot_x, pivot_y - s * 2.0, s * 6.0, s * 0.3, WHITE);
            }
        } else {
            // Idle: sword upright
            let sword_x = cx + s * 2.0;
            let sword_top = cy - s * 6.0 + breathe;
            draw_rectangle(sword_x, sword_top, s * 0.4, s * 4.5, SILVER);
            // Guard
            draw_rectangle(sword_x - s * 0.5, pivot_y, s * 1.5, s * 0.4, GOLD);
            // Handle
            draw_rectangle(sword_x, pivot_y + s * 0.4, s * 0.4, s * 1.2, CHEST_BROWN);

            // Sword glint
            let glint_phase = (self.time * 3.0).sin();
            if glint_phase > 0.7 {
                let glint_y = sword_top + (self.time * 60.0 % (s * 4.0));
                draw_rectangle(sword_x - s * 0.1, glint_y, s * 0.6, s * 0.3, WHITE);
            }
        }

        // Legs
        draw_rectangle(cx - s, cy - s * 1.5 + breathe, s * 0.8, s * 2.5, Color::new(0.3, 0.3, 0.4, 1.0));
        draw_rectangle(cx + s * 0.2, cy - s * 1.5 + breathe, s * 0.8, s * 2.5, Color::new(0.3, 0.3, 0.4, 1.0));

        // Boots
        draw_rectangle(cx - s * 1.2, cy + s * 1.0, s * 1.0, s * 0.5, CHEST_BROWN);
        draw_rectangle(cx + s * 0.2, cy + s * 1.0, s * 1.0, s * 0.5, CHEST_BROWN);

        // Label
        let label = "WARRIOR";
        let dims = measure_text(label, None, (px * 5.0) as u16, 1.0);
        draw_text(label, cx - dims.width / 2.0, cy + s * 3.0, px * 5.0, SILVER);
    }

    fn draw_monster(&self, cx: f32, cy: f32, px: f32) {
        let sway = (self.time * 1.8).sin() * px * 1.5;
        let s = px * 3.0;

        // Horns
        draw_rectangle(cx - s * 2.0 + sway, cy - s * 7.5, s * 0.6, s * 2.0, Color::new(0.5, 0.4, 0.2, 1.0));
        draw_rectangle(cx + s * 1.5 + sway, cy - s * 7.5, s * 0.6, s * 2.0, Color::new(0.5, 0.4, 0.2, 1.0));

        // Head
        draw_rectangle(cx - s * 1.5 + sway, cy - s * 6.0, s * 3.0, s * 2.5, MONSTER_GREEN);
        // Brow ridge
        draw_rectangle(cx - s * 1.5 + sway, cy - s * 5.5, s * 3.0, s * 0.5, MONSTER_GREEN_LIGHT);

        // Eyes (glowing red, pulsing)
        let eye_glow = ((self.time * 4.0).sin() * 0.3 + 0.7) as f32;
        let eye_color = Color::new(1.0, 0.0, 0.0, eye_glow);
        draw_rectangle(cx - s + sway, cy - s * 5.0, s * 0.7, s * 0.7, eye_color);
        draw_rectangle(cx + s * 0.3 + sway, cy - s * 5.0, s * 0.7, s * 0.7, eye_color);
        // Eye glow aura
        let aura = Color::new(1.0, 0.0, 0.0, eye_glow * 0.2);
        draw_rectangle(cx - s * 1.3 + sway, cy - s * 5.3, s * 1.3, s * 1.3, aura);
        draw_rectangle(cx + sway, cy - s * 5.3, s * 1.3, s * 1.3, aura);

        // Mouth / fangs
        draw_rectangle(cx - s * 0.8 + sway, cy - s * 3.8, s * 1.6, s * 0.3, Color::new(0.1, 0.1, 0.1, 1.0));
        // Fangs
        draw_rectangle(cx - s * 0.6 + sway, cy - s * 3.8, s * 0.3, s * 0.7, WHITE);
        draw_rectangle(cx + s * 0.3 + sway, cy - s * 3.8, s * 0.3, s * 0.7, WHITE);

        // Body
        draw_rectangle(cx - s * 2.0 + sway, cy - s * 3.5, s * 4.0, s * 4.0, MONSTER_GREEN);
        // Belly
        draw_rectangle(cx - s * 1.2 + sway, cy - s * 2.5, s * 2.4, s * 2.5, MONSTER_GREEN_LIGHT);

        // Arms (thick)
        draw_rectangle(cx - s * 3.0 + sway, cy - s * 3.0, s * 1.0, s * 3.0, MONSTER_GREEN);
        draw_rectangle(cx + s * 2.0 + sway, cy - s * 3.0, s * 1.0, s * 3.0, MONSTER_GREEN);
        // Claws
        for i in 0..3 {
            let offset = i as f32 * s * 0.35;
            draw_rectangle(cx - s * 3.0 + offset + sway, cy, s * 0.25, s * 0.6, Color::new(0.5, 0.4, 0.2, 1.0));
            draw_rectangle(cx + s * 2.0 + offset + sway, cy, s * 0.25, s * 0.6, Color::new(0.5, 0.4, 0.2, 1.0));
        }

        // Legs
        draw_rectangle(cx - s * 1.5 + sway, cy + s * 0.5, s * 1.2, s * 1.5, MONSTER_GREEN);
        draw_rectangle(cx + s * 0.3 + sway, cy + s * 0.5, s * 1.2, s * 1.5, MONSTER_GREEN);

        // Label
        let label = "BEAST";
        let dims = measure_text(label, None, (px * 5.0) as u16, 1.0);
        draw_text(label, cx - dims.width / 2.0, cy + s * 3.5, px * 5.0, RED_EYES);
    }

    fn draw_treasure(&self, cx: f32, cy: f32, px: f32) {
        let s = px * 3.0;

        // Chest body
        let chest_y = cy - s * 2.0;
        draw_rectangle(cx - s * 2.5, chest_y, s * 5.0, s * 3.0, CHEST_BROWN);
        // Chest dark edges
        draw_rectangle(cx - s * 2.5, chest_y, s * 0.3, s * 3.0, CHEST_BROWN_DARK);
        draw_rectangle(cx + s * 2.2, chest_y, s * 0.3, s * 3.0, CHEST_BROWN_DARK);

        // Lid (open, angled back)
        draw_rectangle(cx - s * 2.5, chest_y - s * 2.0, s * 5.0, s * 2.0, CHEST_BROWN);
        draw_rectangle(cx - s * 2.5, chest_y - s * 2.0, s * 5.0, s * 0.3, CHEST_BROWN_DARK);
        // Lid top edge
        draw_rectangle(cx - s * 2.7, chest_y - s * 2.2, s * 5.4, s * 0.4, CHEST_BROWN_DARK);

        // Metal bands
        draw_rectangle(cx - s * 2.5, chest_y + s * 1.0, s * 5.0, s * 0.3, GOLD);
        draw_rectangle(cx - s * 0.15, chest_y, s * 0.3, s * 3.0, GOLD);

        // Lock
        draw_rectangle(cx - s * 0.4, chest_y + s * 0.6, s * 0.8, s * 0.8, GOLD);
        draw_rectangle(cx - s * 0.2, chest_y + s * 0.8, s * 0.4, s * 0.4, CHEST_BROWN_DARK);

        // Gold coins inside (visible above the chest rim)
        let coin_colors = [GOLD, Color::new(1.0, 0.9, 0.2, 1.0), Color::new(0.9, 0.75, 0.0, 1.0)];
        let coin_positions = [
            (-1.5, -2.8), (-0.5, -3.2), (0.5, -2.9), (1.3, -3.0),
            (-1.0, -3.5), (0.0, -3.8), (1.0, -3.4),
            (-0.3, -4.1), (0.6, -4.0),
        ];
        for (i, (ox, oy)) in coin_positions.iter().enumerate() {
            let color = coin_colors[i % coin_colors.len()];
            draw_rectangle(cx + ox * s, cy + oy * s, s * 0.8, s * 0.6, color);
        }

        // Sparkle / shimmer effect
        let sparkle_count = 5;
        for i in 0..sparkle_count {
            let phase = self.time * 3.0 + i as f32 * 1.3;
            let alpha = ((phase).sin() * 0.5 + 0.5) as f32;
            if alpha > 0.6 {
                let sx = cx + (phase * 2.1).cos() * s * 2.0;
                let sy = cy - s * 3.0 + (phase * 1.7).sin() * s * 1.5;
                let sparkle_color = Color::new(1.0, 1.0, 0.8, alpha);
                // Cross-shaped sparkle
                draw_rectangle(sx - px, sy - px * 3.0, px * 2.0, px * 6.0, sparkle_color);
                draw_rectangle(sx - px * 3.0, sy - px, px * 6.0, px * 2.0, sparkle_color);
            }
        }

        // Label
        let label = "TREASURE";
        let dims = measure_text(label, None, (px * 5.0) as u16, 1.0);
        draw_text(label, cx - dims.width / 2.0, cy + s * 3.0, px * 5.0, GOLD);
    }

    fn draw_build_timestamp(&self, sw: f32, sh: f32) {
        let ts = env!("BUILD_TIMESTAMP");
        let font_size = (sw / 320.0 * 5.0).max(12.0);
        let color = Color::new(1.0, 1.0, 1.0, 0.5);
        let dims = measure_text(ts, None, font_size as u16, 1.0);
        draw_text(ts, (sw - dims.width) / 2.0, sh * 0.83, font_size, color);
    }

    /// Draw audio status text above the build timestamp. Called from main.
    pub fn draw_status(&self, status: &str) {
        let sw = screen_width();
        let sh = screen_height();
        let font_size = (sw / 320.0 * 5.0).max(12.0);
        let color = Color::new(1.0, 0.8, 0.2, 0.7);
        let dims = measure_text(status, None, font_size as u16, 1.0);
        draw_text(status, (sw - dims.width) / 2.0, sh * 0.78, font_size, color);
    }
}
