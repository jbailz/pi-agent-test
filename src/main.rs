use macroquad::prelude::*;

const W: f32 = 800.0;
const H: f32 = 600.0;
const PLAYER_W: f32 = 48.0;
const PLAYER_H: f32 = 20.0;
const INVADER_W: f32 = 34.0;
const INVADER_H: f32 = 22.0;
const INVADER_COLS: usize = 11;
const INVADER_ROWS: usize = 5;
const MAX_WAVES: i32 = 3;

fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders".to_string(),
        window_width: W as i32,
        window_height: H as i32,
        high_dpi: true,
        ..Default::default()
    }
}

#[derive(Clone, Copy)]
struct Shot {
    pos: Vec2,
    vel: Vec2,
    from_enemy: bool,
}

#[derive(Clone, Copy)]
struct Invader {
    pos: Vec2,
    alive: bool,
}

struct Game {
    player: Vec2,
    shots: Vec<Shot>,
    invaders: Vec<Invader>,
    inv_dir: f32,
    inv_timer: f32,
    fire_cooldown: f32,
    lives: i32,
    score: i32,
    wave: i32,
    over: bool,
    won: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            player: vec2(W / 2.0, H - 55.0),
            shots: Vec::new(),
            invaders: Vec::with_capacity(INVADER_COLS * INVADER_ROWS),
            inv_dir: 1.0,
            inv_timer: 0.0,
            fire_cooldown: 0.0,
            lives: 3,
            score: 0,
            wave: 0,
            over: false,
            won: false,
        };
        game.spawn_wave();
        game
    }

    fn spawn_wave(&mut self) {
        self.invaders.clear();
        self.shots.clear();

        for row in 0..INVADER_ROWS {
            for col in 0..INVADER_COLS {
                self.invaders.push(Invader {
                    pos: vec2(90.0 + col as f32 * 56.0, 70.0 + row as f32 * 42.0),
                    alive: true,
                });
            }
        }

        self.inv_dir = 1.0;
        self.inv_timer = 0.0;
        self.wave += 1;
    }

    fn update(&mut self) {
        let dt = get_frame_time().min(1.0 / 20.0);

        if self.over || self.won {
            if is_key_pressed(KeyCode::Enter) {
                *self = Self::new();
            }
            return;
        }

        self.update_player(dt);

        let alive = self.invaders.iter().filter(|i| i.alive).count();
        if alive == 0 {
            if self.wave >= MAX_WAVES {
                self.won = true;
            } else {
                self.spawn_wave();
            }
            return;
        }

        self.update_invaders(dt, alive);
        self.update_shots(dt);

        if self.invaders.iter().any(|i| {
            i.alive && i.pos.y + INVADER_H / 2.0 >= self.player.y - PLAYER_H / 2.0
        }) {
            self.over = true;
        }
    }

    fn update_player(&mut self, dt: f32) {
        let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        let dx = right as i32 - left as i32;

        self.player.x = (self.player.x + dx as f32 * 360.0 * dt)
            .clamp(PLAYER_W / 2.0, W - PLAYER_W / 2.0);

        self.fire_cooldown -= dt;
        if (is_key_down(KeyCode::Space) || is_key_down(KeyCode::Up)) && self.fire_cooldown <= 0.0 {
            self.shots.push(Shot {
                pos: self.player + vec2(0.0, -24.0),
                vel: vec2(0.0, -520.0),
                from_enemy: false,
            });
            self.fire_cooldown = 0.28;
        }
    }

    fn update_invaders(&mut self, dt: f32, alive: usize) {
        self.inv_timer += dt;
        let step_time = (0.62 - self.wave as f32 * 0.05 - (55 - alive) as f32 * 0.008).max(0.08);
        if self.inv_timer < step_time {
            return;
        }

        self.inv_timer = 0.0;
        let at_edge = self.invaders.iter().filter(|i| i.alive).any(|i| {
            (self.inv_dir > 0.0 && i.pos.x + INVADER_W / 2.0 > W - 35.0)
                || (self.inv_dir < 0.0 && i.pos.x - INVADER_W / 2.0 < 35.0)
        });

        if at_edge {
            self.inv_dir *= -1.0;
            for invader in &mut self.invaders {
                invader.pos.y += 24.0;
            }
        } else {
            for invader in &mut self.invaders {
                invader.pos.x += self.inv_dir * 15.0;
            }
        }

        if macroquad::rand::gen_range(0, 100) < 28 {
            let n = macroquad::rand::gen_range(0, alive as i32) as usize;
            if let Some(invader) = self.invaders.iter().filter(|i| i.alive).nth(n) {
                self.shots.push(Shot {
                    pos: invader.pos + vec2(0.0, 18.0),
                    vel: vec2(0.0, 245.0),
                    from_enemy: true,
                });
            }
        }
    }

    fn update_shots(&mut self, dt: f32) {
        for shot in &mut self.shots {
            shot.pos += shot.vel * dt;
        }
        self.shots.retain(on_screen);

        let mut player_was_hit = false;
        for shot in &mut self.shots {
            if shot.from_enemy {
                if !player_was_hit && overlaps(shot.pos, vec2(4.0, 12.0), self.player, vec2(PLAYER_W, PLAYER_H)) {
                    shot.pos.y = H + 99.0;
                    player_was_hit = true;
                }
                continue;
            }

            for invader in &mut self.invaders {
                if invader.alive
                    && overlaps(shot.pos, vec2(4.0, 12.0), invader.pos, vec2(INVADER_W, INVADER_H))
                {
                    invader.alive = false;
                    shot.pos.y = -99.0;
                    self.score += 10;
                    break;
                }
            }
        }

        if player_was_hit {
            self.lives -= 1;
            self.over = self.lives <= 0;
        }
        self.shots.retain(on_screen);
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(8, 10, 20, 255));
        draw_text("SPACE INVADERS", 24.0, 34.0, 28.0, GREEN);
        draw_text(
            &format!("Score {}   Lives {}   Wave {}", self.score, self.lives, self.wave),
            24.0,
            64.0,
            24.0,
            WHITE,
        );

        for x in (0..W as i32).step_by(40) {
            draw_circle(
                x as f32,
                120.0 + (x % 97) as f32 * 3.7 % 420.0,
                1.5,
                DARKGRAY,
            );
        }

        draw_rectangle(
            self.player.x - PLAYER_W / 2.0,
            self.player.y - PLAYER_H / 2.0,
            PLAYER_W,
            PLAYER_H,
            SKYBLUE,
        );
        draw_triangle(
            self.player + vec2(0.0, -28.0),
            self.player + vec2(-18.0, -8.0),
            self.player + vec2(18.0, -8.0),
            SKYBLUE,
        );

        for invader in &self.invaders {
            if invader.alive {
                draw_rectangle(
                    invader.pos.x - INVADER_W / 2.0,
                    invader.pos.y - INVADER_H / 2.0,
                    INVADER_W,
                    INVADER_H,
                    LIME,
                );
                draw_rectangle(invader.pos.x - 10.0, invader.pos.y - 4.0, 6.0, 6.0, BLACK);
                draw_rectangle(invader.pos.x + 4.0, invader.pos.y - 4.0, 6.0, 6.0, BLACK);
            }
        }

        for shot in &self.shots {
            let color = if shot.from_enemy { RED } else { YELLOW };
            draw_rectangle(shot.pos.x - 2.0, shot.pos.y - 7.0, 4.0, 14.0, color);
        }

        if self.over || self.won {
            let msg = if self.won { "YOU WIN" } else { "GAME OVER" };
            draw_rectangle(0.0, H / 2.0 - 70.0, W, 140.0, Color::from_rgba(0, 0, 0, 180));
            let width = measure_text(msg, None, 56, 1.0).width;
            draw_text(msg, W / 2.0 - width / 2.0, H / 2.0 - 8.0, 56.0, WHITE);

            let hint = "Press Enter to restart";
            let width = measure_text(hint, None, 26, 1.0).width;
            draw_text(hint, W / 2.0 - width / 2.0, H / 2.0 + 36.0, 26.0, GRAY);
        }
    }
}

fn overlaps(a: Vec2, a_size: Vec2, b: Vec2, b_size: Vec2) -> bool {
    (a.x - b.x).abs() < (a_size.x + b_size.x) / 2.0
        && (a.y - b.y).abs() < (a_size.y + b_size.y) / 2.0
}

fn on_screen(shot: &Shot) -> bool {
    shot.pos.y > -20.0 && shot.pos.y < H + 20.0
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
