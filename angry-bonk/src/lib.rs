turbo::cfg!(
    r#"
    [solana]
    http-rpc-url = "https://api.devnet.solana.com:8899"
    ws-rpc-url = "https://api.devnet.solana.com:8900"
    name = "Angry Dogs"
    version = "1.0.0"
    author = "Turbo" 
    description = "Launch dogs to knock down structures!"
    [settings]
    resolution = [800, 600]
"#
);

// ALL YOUR SYSTEMS ARE BELONG TO US HAHAHA
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
enum GameStateEnum {
    Ready,
    PlacingCrates,
    Shooting,
    GameOver,
}

const SHEEP_CIRCLE_RADIUS: f32 = 40.0;

// Define the game state initialization using the turbo::init! macro
turbo::init! {

    struct GameState {
        dog_x: f32,
        dog_y: f32,
        dog_vel_x: f32,
        dog_vel_y: f32,
        placed_crates: i32,
        shots_fired: i32,
        is_shooting: u32,
        is_flying: bool,
        game_state: GameStateEnum,
        current_frame: u32,
        player_that_won_last_round: u32,
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
            shots_fired: 0,
            is_shooting: 0,
            is_flying: false,
            game_state: GameStateEnum::Ready,
            player_that_won_last_round: 0,
            targets: vec![
                //Target { x: 600.0, y: 400.0, vel_x: 0.0, vel_y:0.0, width: 50.0, height: 100.0, is_hit: false, sprite_data: SpriteSourceData{width: 58, height: 58, frames: Vec::new() }},
            ],
            current_frame: 0,
        }
    }
}

// Implement the game loop using the turbo::go! macro
turbo::go! {
    let dog_start_x: f32 = 100.0;
    let dog_start_y: f32 = 450.0;

    // Load the game state
    let mut state = GameState::load();
    state.current_frame += 1;
    draw_background(&mut state);

    // Draw the slingshot
    let start_x = 100;
    let start_y = 480;
    let end_x = mouse(0).position[0];
    let end_y = mouse(0).position[1];
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let distance = ((dx * dx + dy * dy) as f32).sqrt();
    let nx = (dx as f32) / (distance as f32);
    let ny = (dy as f32) / (distance as f32);
    let angle = (dy as f32).atan2(dx as f32);

    if state.game_state == GameStateEnum::Ready {
        if
            (mouse(0).left.just_pressed() || mouse(1).left.just_pressed() ||
            gamepad(0).start.just_pressed() ||
            gamepad(1).start.just_pressed() && state.placed_crates == 0) &&
            !state.is_flying
        {
            let sheep_start_pos_x = 450 + (rand() % 100) * 3;
            let mut target = Target {
                x: sheep_start_pos_x as f32,
                y: 200 as f32,
                vel_x: 0.0,
                vel_y: 0.0,
                width: 56.0,
                height: 56.0,
                is_hit: false,
                sprite_data: SpriteSourceData { width: 56, height: 56, frames: Vec::new() },
                sprite_name: String::from("sheep"),
            };

            let sprite_data = get_sprite_data("sheep");

            match sprite_data {
                Some(sprite_data) => {
                    target.width = sprite_data.width as f32;
                    target.height = sprite_data.height as f32;
                    target.sprite_data = sprite_data;
                }
                None => {
                    rect!(
                        x = target.x as i32,
                        y = target.y as i32,
                        w = 56,
                        h = 56,
                        color = 0x8b4513ff
                    );
                }
            }

            state.targets.push(target);
            state.placed_crates += 1;
        }

        if state.placed_crates > 0 {
            state.game_state = GameStateEnum::PlacingCrates;
        }
    } else if state.game_state == GameStateEnum::PlacingCrates {
        if
            (mouse(0).left.just_pressed() || mouse(1).left.just_pressed() ||
                gamepad(0).start.just_pressed() ||
                gamepad(1).start.just_pressed()) &&
            !state.is_flying
        {
            let mut target = Target {
                x: mouse(0).position[0] as f32,
                y: mouse(0).position[1] as f32,
                vel_x: 0.0,
                vel_y: 0.0,
                width: 56.0,
                height: 56.0,
                is_hit: false,
                sprite_data: SpriteSourceData { width: 56, height: 56, frames: Vec::new() },
                sprite_name: String::from("crate-small"),
            };

            let sprite_data = get_sprite_data("crate-small");

            // Check if new target overlap with one of the existing targets
            let mut overlap = false;
            for existing_target in &state.targets {
                if
                    (target.x - existing_target.x).abs() < 56.0 &&
                    (target.y - existing_target.y).abs() < 56.0
                {
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
                        rect!(
                            x = target.x as i32,
                            y = target.y as i32,
                            w = 56,
                            h = 56,
                            color = 0x8b4513ff
                        );
                    }
                }

                state.targets.push(target);
                state.placed_crates += 1;
            }
        }

        if state.placed_crates > 5 {
            state.game_state = GameStateEnum::Shooting;
        }
    } else if
        (mouse(0).left.just_pressed() ||
            gamepad(0).start.just_pressed() ||
            gamepad(1).start.just_pressed()) &&
        !state.is_flying &&
        state.game_state == GameStateEnum::Shooting
    {
        // Calculate launch velocity based on mouse position
        let dx = (mouse(0).position[0] as f32) - state.dog_x;
        let dy = (mouse(0).position[1] as f32) - state.dog_y;
        state.dog_vel_x = dx * 0.035;
        state.dog_vel_y = dy * 0.035;
        state.dog_x = (start_x as f32) + nx * 100.0;
        state.dog_y = (start_y as f32) + ny * 100.0;
        state.is_flying = true;
        state.is_shooting = state.current_frame + 20;
        state.shots_fired += 1;
    }

    if (state.game_state == GameStateEnum::GameOver) && (mouse(0).left.just_pressed() ||
            gamepad(0).start.just_pressed() || gamepad(1).start.just_pressed()) {
        state = GameState{
            dog_x: 100.0,
            dog_y: 450.0,
            dog_vel_x: 0.0,
            dog_vel_y: 0.0,
            placed_crates: 0,
            shots_fired: 0,
            is_shooting: 0,
            is_flying: false,
            game_state: GameStateEnum::Ready,
            player_that_won_last_round: state.player_that_won_last_round,
            targets: vec![
                //Target { x: 600.0, y: 400.0, vel_x: 0.0, vel_y:0.0, width: 50.0, height: 100.0, is_hit: false, sprite_data: SpriteSourceData{width: 58, height: 58, frames: Vec::new() }},
            ],
            current_frame: 0,
        }
    };

    physic_step(&mut state);

    // Update dog position if flying
    if state.is_flying {
        state.dog_x += state.dog_vel_x;
        state.dog_y += state.dog_vel_y;
        // Apply gravity
        state.dog_vel_y += 0.198;

        // Check for collisions with the ground
        if state.dog_y > 550.0 {
            state.is_flying = false;
            state.dog_x = dog_start_x;
            state.dog_y = dog_start_y;
            state.dog_vel_x = 0.0;
            state.dog_vel_y = 0.0;
            if state.shots_fired >= 3 {
                state.game_state = GameStateEnum::GameOver;
                state.player_that_won_last_round = 2;
            }
        }
    }
    // Check for collisions with targets
    let hit_index = get_target_index_by_circle_hit(
        &state.targets,
        state.dog_x,
        state.dog_y,
        16.0,
        -1
    );
    if hit_index >= 0 {
        state.is_flying = false;
        state.dog_x = dog_start_x;
        state.dog_y = dog_start_y;
        state.dog_vel_x = 0.0;
        state.dog_vel_y = 0.0;
        state.targets[hit_index as usize].is_hit = true;

        if state.targets[hit_index as usize].sprite_name == "sheep" {
            state.game_state = GameStateEnum::GameOver;
            state.player_that_won_last_round = 1;
        }
        else {
            // delete crate from list
            state.targets.remove(hit_index as usize);
            if (state.shots_fired >= 3) {
                state.game_state = GameStateEnum::GameOver;
                state.player_that_won_last_round = 2;
            }
        }
    }
    // how is the rotation of a sprite working - where's the pivot point? Recalc is complicated :(
    // how about push / pop of transforms?
    sprite!(
        "cannon_barrel",
        x = ((start_x as f32) - 64.0 + nx * 32.0) as i32,
        y = ((start_y as f32) - 64.0 + ny * 32.0) as i32,
        scale_x = 2.0,
        scale_y = 2.0,
        rotate = ((angle * 180.0) / std::f32::consts::PI) as i32
    );
    sprite!("cannon_wheel", x = (start_x as i32) - 20, y = start_y as i32);

    // DEBUG collider drawing
    // let mut line_color = 0xffffffff;
    // let hit_index = get_target_index_by_circle_hit(
    //     &state.targets,
    //     mouse(0).position[0] as f32,
    //     mouse(0).position[1] as f32,
    //     16.0,
    //     -1
    // );
    // if hit_index >= 0 {
    //     line_color = 0xff0000ff;
    //     // draw the collider of hit target
    //     draw_target_collider(&state.targets[hit_index as usize]);
    //     // draw mouse cursor as circle with radius 20
    //     circ!(
    //         x = (mouse(0).position[0] as i32) - 16,
    //         y = (mouse(0).position[1] as i32) - 16,
    //         d = 32,
    //         color = 0xff0000ff
    //     );
    // }

    // let current_x = mouse(0).position[0];
    // let current_y = mouse(0).position[1];

    // let formattedString = &format!("Mouse Position: ({}, {})", current_x, current_y).to_string();

    // text!(formattedString, x = current_x, y = current_y);

    // rect!(
    //     x = (start_x + end_x) / 2 - ((distance / 2.0) as i32),
    //     y = (start_y + end_y) / 2 - 1,
    //     w = distance as u32,
    //     h = 2,
    //     color = line_color,
    //     rotate = ((angle * 180.0) / std::f32::consts::PI) as i32
    // );

    sprite!("bonk-dog", x = (start_x as i32) - 180, y = (start_y as i32) - 100, fps = 10, scale_x = 2.0, scale_y = 2.0);
    // Draw the dog
    if state.is_flying {
        sprite!("ball", x = (state.dog_x as i32) - 8, y = (state.dog_y as i32) - 8);
    }

    if state.is_shooting > state.current_frame {
        let muzzle_x = (start_x as f32) + nx * 100.0;
        let muzzle_y = (start_y as f32) + ny * 100.0;
        sprite!(
            "explosion_sheet",
            x = (muzzle_x as i32) - 16,
            y = (muzzle_y as i32) - 16,
            rotate = ((angle * 180.0) / std::f32::consts::PI) as i32,
            fps = fps::SUPER_FAST
        );
    }

    // Draw the targets
    draw_targets(&state.targets);

    if state.game_state == GameStateEnum::GameOver {
        if state.player_that_won_last_round == 1 {
            text!("Bonk won", x = 300, y = 250, color = 0x000000ff, font = Font::XL);
        } else {
            text!("The Sheep won", x = 300, y = 250, color = 0x000000ff, font = Font::XL);
        }
    }

    if state.game_state == GameStateEnum::PlacingCrates {
        let remaining_crates = 6 - state.placed_crates;
        let formatted_string = &format!("Place {} more crates", remaining_crates).to_string();
        text!("Player Sheep", x = 300, y = 230, color = 0x000000ff, font = Font::XL);
        text!(formatted_string, x = 300, y = 250, color = 0x000000ff, font = Font::XL);
    }

    if state.game_state == GameStateEnum::Shooting {
        let remaining_crates = 3 - state.shots_fired;
        let formatted_string = &format!("{} shots left", remaining_crates).to_string();
        text!("Player Bonk", x = 300, y = 230, color = 0x000000ff, font = Font::XL);
        text!(formatted_string, x = 300, y = 250, color = 0x000000ff, font = Font::XL);
    }

    if state.game_state == GameStateEnum::Ready {
        if state.player_that_won_last_round == 2 {
            text!("The Sheep won", x = 300, y = 400, color = 0x000000ff, font = Font::XL);
        } else if state.player_that_won_last_round == 1 {
            text!("Bonk won", x = 300, y = 400, color = 0x000000ff, font = Font::XL);
        }

        sprite!("logo", x = 50, y = 50);
    }

    // Save game state for the next frame
    state.save();
}

fn draw_targets(targets: &Vec<Target>) {
    for target in targets {
        if target.sprite_name == "sheep" {
            if target.is_hit {
                sprite!(
                    "dead_sheep",
                    x = (target.x - target.width * 0.5) as i32,
                    y = (target.y - target.height * 0.5) as i32,
                    scale_x = 2.0,
                    scale_y = 2.0
                );
                continue;
            }
            sprite!(
                "sheep_animation",
                x = (target.x - SHEEP_CIRCLE_RADIUS * 1.6) as i32,
                y = (target.y - SHEEP_CIRCLE_RADIUS * 1.6) as i32,
                fps = 10,
                scale_x = 2.0,
                scale_y = 2.0
            );
        } else {
            sprite!(
                &target.sprite_name,
                x = (target.x - target.width * 0.5) as i32,
                y = (target.y - target.height * 0.5) as i32
            );
        }
    }
}

fn get_target_index_by_rect_hit(
    targets: &Vec<Target>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    ignore_index: i32,
) -> i32 {
    let left_x = x - w * 0.5;
    let right_x = x + w * 0.5;
    let top_y = y - h * 0.5;
    let bottom_y = y + h * 0.5;
    for i in 0..targets.len() {
        if i == (ignore_index as usize) {
            continue;
        }
        let target = &targets[i];
        // make circle hit test
        if target.sprite_name == "sheep" {
            if target.x > left_x && target.x < right_x && target.y > top_y && target.y < bottom_y {
                return i as i32;
            }
            if target.x + SHEEP_CIRCLE_RADIUS < left_x || target.x - SHEEP_CIRCLE_RADIUS > right_x {
                continue;
            }
            if target.y + SHEEP_CIRCLE_RADIUS < top_y || target.y - SHEEP_CIRCLE_RADIUS > bottom_y {
                continue;
            }
            // get the corner point of the rectangle that is closest to the circle
            let corner_x = if target.x < left_x { left_x } else { right_x };
            let corner_y = if target.y < top_y { top_y } else { bottom_y };
            let dx = target.x - corner_x;
            let dy = target.y - corner_y;
            let dsq = dx * dx + dy * dy;
            if dsq < SHEEP_CIRCLE_RADIUS * SHEEP_CIRCLE_RADIUS {
                return i as i32;
            }
            continue;
        }

        let crate_left = target.x - target.width / 2.0;
        let crate_right = target.x + target.width / 2.0;
        let crate_top = target.y - target.height / 2.0;
        let crate_bottom = target.y + target.height / 2.0;
        if left_x < crate_right
            && right_x > crate_left
            && top_y < crate_bottom
            && bottom_y > crate_top
        {
            return i as i32;
        }
    }
    -1
}

fn get_target_index_by_circle_hit(
    targets: &Vec<Target>,
    x: f32,
    y: f32,
    r: f32,
    ignore_index: i32,
) -> i32 {
    for i in 0..targets.len() {
        if i == (ignore_index as usize) {
            continue;
        }
        let target = &targets[i];
        let dx = target.x - x;
        let dy = target.y - y;
        let dsq = dx * dx + dy * dy;

        if target.sprite_name == "sheep" {
            // make circle hit test
            let d = SHEEP_CIRCLE_RADIUS + r;
            let d2 = d * d;
            if dsq < d2 {
                return i as i32;
            }
            continue;
        }
        let target_left = target.x - target.width / 2.0;
        let target_right = target.x + target.width / 2.0;
        let target_top = target.y - target.height / 2.0;
        let target_bottom = target.y + target.height / 2.0;
        if target_left < x + r
            && target_right > x - r
            && target_top < y + r
            && target_bottom > y - r
        {
            return i as i32;
        }
        if x + r < target_left || x - r > target_right {
            continue;
        }
        if y + r < target_top || y - r > target_bottom {
            continue;
        }
        // get the corner point of the rectangle that is closest to the circle
        let corner_x = if target.x < x {
            target_left
        } else {
            target_right
        };
        let corner_y = if target.y < y {
            target_top
        } else {
            target_bottom
        };
        let dx = target.x - corner_x;
        let dy = target.y - corner_y;
        let dsq = dx * dx + dy * dy;
        if dsq < r * r {
            return i as i32;
        }
    }
    -1
}

fn draw_target_collider(target: &Target) {
    if target.sprite_name == "sheep" {
        circ!(
            x = (target.x - SHEEP_CIRCLE_RADIUS) as i32,
            y = (target.y - SHEEP_CIRCLE_RADIUS) as i32,
            d = (SHEEP_CIRCLE_RADIUS * 2.0) as u32,
            color = 0x8b4513ff
        );

        circ!(
            x = (target.x - SHEEP_CIRCLE_RADIUS * 0.125) as i32,
            y = (target.y - SHEEP_CIRCLE_RADIUS * 0.125) as i32,
            d = (SHEEP_CIRCLE_RADIUS * 0.25) as u32,
            color = 0xffffffff
        );
    } else {
        rect!(
            x = (target.x as i32) - (target.width as i32) / 2,
            y = (target.y as i32) - (target.height as i32) / 2,
            w = target.width as u32,
            h = target.height as u32,
            color = 0x8b4513ff
        );
    }
}

fn draw_background(game_state: &mut GameState) {
    clear(0x87ceebff); // Sky blue
    for x in 0..4 {
        sprite!(
            "cloud-1",
            x = -300
                + ((x * 3821 + (((game_state.current_frame as f32) * 0.4) as i32))
                    % (1500 + x * 22)),
            y = 70 + (x % 4) * 20
        );
        sprite!(
            "cloud-2",
            x = -300 + ((x * 23945 + (((game_state.current_frame as f32) * 0.62) as i32)) % 1290),
            y = 150 + (x % 4) * 35
        );
    }
    //sprite!("cloud-3", x = 400, y = 80);
    sprite!("grass_patch", x = 0, y = 300);
    sprite!("grass_patch", x = 300, y = 310);
    // sprite!("grass_patch", x = 750, y = 360);
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

        if target.sprite_name == "sheep" {
            if get_target_index_by_circle_hit(targets, new_x, new_y, SHEEP_CIRCLE_RADIUS, n as i32)
                >= 0
            {
                new_vel_y = 0.0;
                new_x = target.x + target.vel_x;
                new_y = target.y + new_vel_y;
            }
        } else if get_target_index_by_rect_hit(
            targets,
            new_x,
            new_y,
            target.width,
            target.height,
            n as i32,
        ) >= 0
        {
            new_vel_y = 0.0;
            new_x = target.x;
            new_y = target.y;
        }

        target.x = new_x;
        target.y = new_y;
        target.vel_y = new_vel_y;

        targets[n] = target;
    }
}
