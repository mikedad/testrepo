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
}

impl SplashScreen {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            user_interacted: false,
        }
    }

    pub fn user_interacted(&self) -> bool {
        self.user_interacted
    }

    pub fn update(&mut self) {
        self.time += get_frame_time();

        // Touch, click, or key — any interaction counts (iPad + desktop)
        if is_mouse_button_pressed(MouseButton::Left)
            || !touches().is_empty()
            || get_last_key_pressed().is_some()
        {
            self.user_interacted = true;
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
        self.draw_prompt(sw, sh);
        self.draw_build_timestamp(sw, sh);
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

        // Sword (right hand) with glint animation
        let sword_x = cx + s * 2.0;
        let sword_top = cy - s * 6.0 + breathe;
        // Blade
        draw_rectangle(sword_x, sword_top, s * 0.4, s * 4.5, SILVER);
        // Guard
        draw_rectangle(sword_x - s * 0.5, cy - s * 1.5 + breathe, s * 1.5, s * 0.4, GOLD);
        // Handle
        draw_rectangle(sword_x, cy - s * 1.1 + breathe, s * 0.4, s * 1.2, CHEST_BROWN);

        // Sword glint
        let glint_phase = (self.time * 3.0).sin();
        if glint_phase > 0.7 {
            let glint_y = sword_top + (self.time * 60.0 % (s * 4.0));
            draw_rectangle(sword_x - s * 0.1, glint_y, s * 0.6, s * 0.3, WHITE);
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

    fn draw_prompt(&self, sw: f32, sh: f32) {
        let alpha = ((self.time * 2.0).sin() * 0.4 + 0.6) as f32;
        let color = Color::new(1.0, 1.0, 1.0, alpha);
        let text = "~ Tap to continue ~";
        let font_size = (sw / 320.0 * 7.0).max(14.0);
        let dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(text, (sw - dims.width) / 2.0, sh * 0.88, font_size, color);
    }
}
