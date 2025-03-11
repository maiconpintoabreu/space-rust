use raylib::{core::math::Vector2, prelude::*};

#[derive(Default)]
struct Player {
    position: Vector2,
    speed: Vector2,
    acceleration: f32,
    rotation: f32,
    is_turn_left: bool,
    is_turn_right: bool,
    is_accelerating: bool,
    is_breaking: bool,
}
const PLAYER_SPEED: f32 = 50f32;
const PLAYER_ROTATION_SPEED: f32 = 100f32;
const SHIP_HEIGHT: f32 = 10f32 / 0.363970f32;
const ZERO_SPEED: f32 = 0f32;
const PHYSICS_TIME: f32 = 0.02f32;

fn main() {
    let mut player = Player::default();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Space Rust").build();

    let (mut width, mut height) = (rl.get_screen_width() as f32, rl.get_screen_height() as f32);
    let half_width = width / 2.0;
    let half_height = height / 2.0;

    player.position = Vector2::new(half_width, half_height - (SHIP_HEIGHT / 2f32));
    player.acceleration = 0f32;
    let mut frame_time_accumulator = 0f32;

    while !rl.window_should_close() {
        // Tick
        if rl.is_window_resized() {
            width = rl.get_screen_width() as f32;
            height = rl.get_screen_height() as f32;
        }
        frame_time_accumulator += rl.get_frame_time();

        // Input

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            player.is_turn_left = true;
        } else {
            player.is_turn_left = false;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            player.is_turn_right = true;
        } else {
            player.is_turn_right = false;
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            player.is_accelerating = true;
        } else {
            player.is_accelerating = false;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            player.is_breaking = true;
        } else {
            player.is_breaking = false;
        }

        // Physics

        if frame_time_accumulator > PHYSICS_TIME {
            frame_time_accumulator -= PHYSICS_TIME;

            let rotation_speed = PLAYER_ROTATION_SPEED * PHYSICS_TIME;
            let acceleration = PLAYER_SPEED * PHYSICS_TIME;

            if player.is_turn_left {
                player.rotation -= rotation_speed;
            } else if player.is_turn_right {
                player.rotation += rotation_speed;
            }

            if player.rotation > 180f32 {
                player.rotation -= 360f32;
            }
            if player.rotation < -180f32 {
                player.rotation += 360f32;
            }
            if player.is_accelerating {
                player.is_accelerating = true;
                if player.acceleration < PLAYER_SPEED {
                    player.acceleration += acceleration;
                }
            } else if player.acceleration > ZERO_SPEED {
                player.acceleration -= acceleration / 2.0;
            } else if player.acceleration < ZERO_SPEED {
                player.acceleration = ZERO_SPEED;
            }
            if player.is_breaking {
                if player.acceleration > ZERO_SPEED {
                    player.acceleration -= acceleration;
                } else if player.acceleration < ZERO_SPEED {
                    player.acceleration = ZERO_SPEED;
                }
            }

            let direction: Vector2 = Vector2::new(
                f32::sin(player.rotation * DEG2RAD as f32),
                -f32::cos(player.rotation * DEG2RAD as f32),
            );
            player.speed = direction.normalized() * player.acceleration * PHYSICS_TIME;
            player.position += player.speed;

            if player.position.x > width + SHIP_HEIGHT {
                player.position.x = -SHIP_HEIGHT;
            } else if player.position.x < -SHIP_HEIGHT {
                player.position.x = width + SHIP_HEIGHT;
            }

            if player.position.y > height + SHIP_HEIGHT {
                player.position.y = -SHIP_HEIGHT;
            } else if player.position.y < -SHIP_HEIGHT {
                player.position.y = height + SHIP_HEIGHT;
            }
        }
        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw UI
        let mut player_speed = "Speed: ".to_owned();
        player_speed.push_str(player.acceleration.to_string().as_str());
        d.draw_text(player_speed.as_str(), 20, 12, 20, Color::BLACK);
        let mut fps = "FPS: ".to_owned();
        fps.push_str(d.get_fps().to_string().as_str());
        d.draw_text(fps.as_str(), (width - 100f32) as i32, 12, 20, Color::BLACK);

        let cosf = f32::cos(player.rotation.to_radians());
        let sinf = f32::sin(player.rotation.to_radians());
        let v1 = Vector2::new(
            player.position.x + sinf * SHIP_HEIGHT,
            player.position.y - cosf * SHIP_HEIGHT,
        );
        let v2 = Vector2::new(
            player.position.x - cosf * 10f32,
            player.position.y - sinf * 10f32,
        );
        let v3 = Vector2::new(
            player.position.x + cosf * 10f32,
            player.position.y + sinf * 10f32,
        );

        // Draw Game
        d.draw_triangle(v1, v2, v3, Color::GRAY);
    }
}
