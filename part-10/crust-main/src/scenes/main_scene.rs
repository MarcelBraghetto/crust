use crate::{
    components::{model::Model, perspective_camera::PerspectiveCamera},
    core::{display_size::DisplaySize, failable::Failable, failable_unit::FailableUnit, renderer::Renderer, scene::Scene},
    scenes::player::Player,
};
use sdl2::keyboard::Scancode;

pub struct MainScene {
    camera: PerspectiveCamera,
    display_size: DisplaySize,
    models: Vec<Model>,
    player: Player,
}

impl MainScene {
    pub fn new(display_size: DisplaySize) -> Failable<Self> {
        let mut camera = PerspectiveCamera::new(&display_size);
        let player = Player::new(glm::vec3(0., 0., 2.), 0., 0., 0.);

        camera.configure(player.position(), player.direction());

        Ok(MainScene {
            camera: camera,
            player: player,
            models: create_models(),
            display_size: display_size,
        })
    }

    fn process_input(&mut self, delta: f32, event_pump: &sdl2::EventPump) -> FailableUnit {
        // We will see if the user is pressing arrow keys to move around the scene with.
        // https://wiki.libsdl.org/SDL_GetKeyboardState
        // https://github.com/Rust-SDL2/rust-sdl2/blob/master/src/sdl2/keyboard/mod.rs
        let key_state = event_pump.keyboard_state();

        if key_state.is_scancode_pressed(Scancode::Up) {
            self.player.move_forward(delta);
        }

        if key_state.is_scancode_pressed(Scancode::Down) {
            self.player.move_backward(delta);
        }

        if key_state.is_scancode_pressed(Scancode::A) {
            self.player.move_up(delta);
        }

        if key_state.is_scancode_pressed(Scancode::Z) {
            self.player.move_down(delta);
        }

        if key_state.is_scancode_pressed(Scancode::Left) {
            self.player.turn_left(delta);
        }

        if key_state.is_scancode_pressed(Scancode::Right) {
            self.player.turn_right(delta);
        }

        // We will also see if the user is pressing near the edges of the screen and move around accordingly.
        // This is kinda nice because it means on a mobile device we can touch the screen to move, though we'd
        // need to do more to support true multitouch input.
        // https://wiki.libsdl.org/SDL_GetMouseState
        // https://github.com/Rust-SDL2/rust-sdl2/blob/master/src/sdl2/mouse/mod.rs
        let mouse_state = event_pump.mouse_state();

        if mouse_state.left() {
            let x = mouse_state.x();
            let x_threshold = self.display_size.width / 3;

            if x < x_threshold {
                self.player.turn_left(delta);
            } else if x > self.display_size.width - x_threshold {
                self.player.turn_right(delta);
            }

            let y = mouse_state.y();
            let y_threshold = self.display_size.height / 3;

            if y < y_threshold {
                self.player.move_forward(delta);
            } else if y > self.display_size.height - y_threshold {
                self.player.move_backward(delta);
            }
        }

        Ok(())
    }
}

fn create_models() -> Vec<Model> {
    vec![
        Model::new(
            "assets/models/crate.obj",
            "assets/textures/crate.png",
            "default",
            glm::vec3(0.4, 0.6, 0.),
            glm::vec3(0.6, 0.6, 0.6),
        ),
        Model::new(
            "assets/models/torus.obj",
            "assets/textures/red_cross_hatch.png",
            "default",
            glm::vec3(-0.6, 0.4, 0.),
            glm::vec3(0.4, 0.4, 0.4),
        ),
        Model::new(
            "assets/models/crate.obj",
            "assets/textures/crate.png",
            "default",
            glm::vec3(-0.5, -0.5, 0.),
            glm::vec3(0.7, 0.3, 0.3),
        ),
        Model::new(
            "assets/models/torus.obj",
            "assets/textures/red_cross_hatch.png",
            "default",
            glm::vec3(0.6, -0.4, 0.),
            glm::vec3(0.4, 0.4, 0.4),
        ),
    ]
}

impl Scene for MainScene {
    fn update(&mut self, delta: f32, event_pump: &sdl2::EventPump) -> FailableUnit {
        self.process_input(delta, event_pump)?;

        self.camera.configure(self.player.position(), self.player.direction());

        for model in &mut self.models {
            let orientation = model.orientation();

            orientation.add_yaw(delta * 45.);
            orientation.add_pitch(delta * 35.);
            orientation.add_roll(delta * 15.);
        }

        Ok(())
    }

    fn render(&mut self, renderer: &mut dyn Renderer) -> FailableUnit {
        renderer.render_models(&self.models, &self.camera.projection_view())
    }

    fn on_display_size_changed(&mut self, display_size: DisplaySize) -> FailableUnit {
        self.display_size = display_size;
        self.camera = PerspectiveCamera::new(&self.display_size);

        Ok(())
    }
}
