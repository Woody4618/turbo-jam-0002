// Define the game configuration using the turbo::cfg! macro
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
        is_flying: bool,
        targets: Vec<struct Target {
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            is_hit: bool,
        }>,
    } = {
        Self {
            dog_x: 100.0,
            dog_y: 450.0,
            dog_vel_x: 0.0,
            dog_vel_y: 0.0,
            is_flying: false,
            targets: vec![
                Target { x: 600.0, y: 400.0, width: 50.0, height: 100.0, is_hit: false },
                Target { x: 650.0, y: 400.0, width: 50.0, height: 150.0, is_hit: false },
            ],
        }
    }
}

// Implement the game loop using the turbo::go! macro
turbo::go! {
    // Load the game state
    let mut state = GameState::load();

    // Handle user input
    if (mouse(0).left.pressed() || gamepad(0).start.pressed() || gamepad(1).start.pressed()) && !state.is_flying {
        // Calculate launch velocity based on mouse position
        let dx = mouse(0).position[0] as f32 - state.dog_x;
        let dy = mouse(0).position[1] as f32 - state.dog_y;
        state.dog_vel_x = dx * 0.1;
        state.dog_vel_y = dy * 0.1;
        state.is_flying = true;
    }

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
    let distance = (dx as f32).hypot(dy as f32);
    let angle = (dy as f32).atan2(dx as f32) as i32;
    rect!(x = start_x, y = start_y - 1, w = distance as u32, h = 2, color = 0x654321ff, rotate = angle);

    // Draw the dog
    if state.is_flying {
        sprite!("dog", x = state.dog_x as i32, y = state.dog_y as i32);
    }

    // Draw the targets
    for target in &state.targets {
        rect!(x = target.x as i32, y = target.y as i32, w = target.width as u32, h = target.height as u32, color = 0x8B4513ff);
    }

    // Save game state for the next frame
    state.save();
}
