turbo::cfg! {r#"
    name = "Angry Dogs"
    version = "1.0.0"
    author = "Turbo"
    description = "Launch dogs to knock down structures!"
    [settings]
    resolution = [800, 600]
"#}

// Define the game state initialization using the turbo::init! macro
turbo::init! {
    struct GameState {
        dog_x: f32,
        dog_y: f32,
        dog_vel_x: f32,
        dog_vel_y: f32,
        placed_crates: i32,
        is_flying: bool,
        game_state: u8,
        current_frame: u32,
        targets: Vec<struct Target {
            x: f32,
            y: f32,
            vel_x: f32,
            vel_y: f32,
            width: f32,
            height: f32,
            is_hit: bool,
            sprite_data: SpriteSourceData,
            sprite_name: String,
        }>,
    } = {
        Self {
            dog_x: 100.0,
            dog_y: 450.0,
            dog_vel_x: 0.0,
            dog_vel_y: 0.0,
            placed_crates: 0,
            is_flying: false,
            game_state: 0,
            targets: vec![
                //Target { x: 600.0, y: 400.0, vel_x: 0.0, vel_y:0.0, width: 50.0, height: 100.0, is_hit: false, sprite_data: SpriteSourceData{width: 58, height: 58, frames: Vec::new() }},
            ],
            current_frame: 0,
        }
    }
}

// Implement the game loop using the turbo::go! macro
turbo::go! {
    // Load the game state
    let mut state = GameState::load();
    state.current_frame += 1;

    if state.game_state == 0 {
        if (mouse(0).left.just_pressed() || gamepad(0).start.just_pressed() || gamepad(1).start.just_pressed()) && !state.is_flying {
            let mut target = Target {
                x: mouse(0).position[0] as f32,
                 y: mouse(0).position[1] as f32,
                 vel_x: 0.0, vel_y:0.0,
                 width: 56.0, height: 56.0,
                 is_hit: false,
                 sprite_data: SpriteSourceData{width: 56, height: 56, frames: Vec::new()},
                 sprite_name: String::from("sheep")
            };

            let sprite_data = get_sprite_data("sheep");

            match sprite_data {
                Some(sprite_data) => {
                    target.width = sprite_data.width as f32;
                    target.height = sprite_data.height as f32;
                    target.sprite_data = sprite_data;
                }
                None => {
                    rect!(x = target.x as i32, y = target.y as i32, w = 56, h = 56, color = 0x8B4513ff);
                }
            }

            state.targets.push(target);
            state.placed_crates += 1;
        }

        if state.placed_crates > 0 {
            state.game_state = 1;
        }
    }else if state.game_state == 1 {
        if (mouse(0).left.just_pressed() || gamepad(0).start.just_pressed() || gamepad(1).start.just_pressed()) && !state.is_flying {
            let mut target = Target {
                x: mouse(0).position[0] as f32,
                 y: mouse(0).position[1] as f32,
                 vel_x: 0.0, vel_y:0.0,
                 width: 56.0, height: 56.0,
                 is_hit: false,
                 sprite_data: SpriteSourceData{width: 56, height: 56, frames: Vec::new(), },
                 sprite_name: String::from("crate-small")
            };

            let sprite_data = get_sprite_data("crate-small");

            // Check if new target overlap with one of the existing targets
            let mut overlap = false;
            for existing_target in &state.targets {
                if (target.x - existing_target.x).abs() < 56.0 && (target.y - existing_target.y).abs() < 56.0 {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                match sprite_data {
                    Some(sprite_data) => {
                        target.width = sprite_data.width as f32;
                        target.height = sprite_data.height as f32;
                        target.sprite_data = sprite_data;
                    }
                    None => {
                        rect!(x = target.x as i32, y = target.y as i32, w = 56, h = 56, color = 0x8B4513ff);
                    }
                }

                state.targets.push(target);
                state.placed_crates += 1;
            }
        }

        if state.placed_crates > 5 {
            state.game_state = 2;
        }

     } else if (mouse(0).left.just_pressed() || gamepad(0).start.just_pressed() || gamepad(1).start.just_pressed()) && !state.is_flying {
        // Calculate launch velocity based on mouse position
        let dx = mouse(0).position[0] as f32 - state.dog_x;
        let dy = mouse(0).position[1] as f32 - state.dog_y;
        state.dog_vel_x = dx * 0.1;
        state.dog_vel_y = dy * 0.1;
        state.is_flying = true;
    }

    physic_step(&mut state);

    // Update dog position if flying
    if state.is_flying {
        state.dog_x += state.dog_vel_x;
        state.dog_y += state.dog_vel_y;
        // Apply gravity
        state.dog_vel_y += 0.98;

        // Check for collisions with the ground
        if state.dog_y > 550.0 {
            state.is_flying = false;
            state.dog_x = 100.0;
            state.dog_y = 450.0;
            state.dog_vel_x = 0.0;
            state.dog_vel_y = 0.0;
        }
    }

    // Check for collisions with targets
    state.targets.retain_mut(|target| {
        let hit = !target.is_hit && state.is_flying && state.dog_x + 20.0 > target.x && state.dog_x < target.x + target.width && state.dog_y + 20.0 > target.y && state.dog_y < target.y + target.height;
        if hit {
            target.is_hit = true;
            state.is_flying = false;
            state.dog_x = 100.0;
            state.dog_y = 450.0;
            state.dog_vel_x = 0.0;
            state.dog_vel_y = 0.0;
        }
        !hit
    });

    // Set the background color
    clear(0x87CEEBff); // Sky blue

    // Draw the slingshot
    let start_x = 100;
    let start_y = 450;
    let end_x = mouse(0).position[0];
    let end_y = mouse(0).position[1];
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let distance = ((dx * dx + dy * dy) as f32).sqrt();
    let angle = (dy as f32).atan2(dx as f32);

    rect!(x = ((start_x + end_x) / 2) - (distance / 2.0) as i32, y = ((start_y +end_y) /2) -1, w = distance as u32, h = 2, color = 0x654321ff, rotate = (angle * 180.0 / std::f32::consts::PI) as i32);

    // Draw the dog
    if state.is_flying {
        sprite!("dog", x = state.dog_x as i32, y = state.dog_y as i32);
    }

    // Draw the targets
    for target in &state.targets {
        sprite!(&target.sprite_name, x = target.x as i32, y = target.y as i32);
        //draw_sprite(target.x as i32, target.y as i32, target.width as u32, target.height as u32, 0, 0, target.sprite_data.width as i32, target.sprite_data.height as i32 ,0xFFFFFFFF, 0);
    }

    // Save game state for the next frame
    state.save();
}

fn physic_step(gameState: &mut GameState) {
    let mut targets = &mut gameState.targets;

    for n in 0..targets.len() {
        let mut target = targets[n].clone();

        if target.is_hit {
            continue;
        }

        // Apply gravity
        let mut new_vel_y = target.vel_y + 0.98;

        // Update position
        let mut new_x = target.x + target.vel_x;
        let mut new_y = target.y + new_vel_y;

        // Check for collisions with the ground
        if new_y > 500.0 {
            new_y = 500.0;
            new_vel_y = 0.0;
        }

        for m in 0..targets.len() {
            if n == m {
                continue;
            }

            let other_target = &targets[m];
            if (other_target.x - new_x).abs() < 56.0 && (other_target.y - new_y).abs() < 56.0 {
                new_x = target.x;
                new_y = target.y;
                new_vel_y = 0.0;
                break;
            }
        }

        target.x = new_x;
        target.y = new_y;
        target.vel_y = new_vel_y;

        targets[n] = target;
    }
}
