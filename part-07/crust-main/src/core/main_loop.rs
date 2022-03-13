use crate::{
    core::{engine::Engine, failable::Failable, scene::Scene},
    scenes::main_scene::MainScene,
};
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    EventPump,
};

fn current_time(timer: &sdl2::TimerSubsystem) -> u64 {
    sdl2::TimerSubsystem::performance_counter(timer)
}

pub struct MainLoop<T: Engine> {
    engine: T,
    scene: Box<dyn Scene>,
    timer: sdl2::TimerSubsystem,
    performance_frequency: f64,
    current_time: u64,
    previous_time: u64,
    event_pump: EventPump,
}

impl<T: Engine> MainLoop<T> {
    pub fn new(sdl: &sdl2::Sdl, engine: T) -> Failable<Self> {
        let timer = sdl.timer()?;
        let performance_frequency = sdl2::TimerSubsystem::performance_frequency(&timer) as f64;
        let current_time = current_time(&timer);
        let display_size = engine.get_display_size()?;
        let event_pump = sdl.event_pump()?;

        Ok(MainLoop {
            engine: engine,
            scene: Box::new(MainScene::new(display_size)?),
            timer: timer,
            performance_frequency: performance_frequency,
            current_time: current_time,
            previous_time: current_time,
            event_pump: event_pump,
        })
    }

    pub fn run(&mut self) -> Failable<bool> {
        for event in self.event_pump.poll_iter() {
            if let Event::Window {
                win_event: WindowEvent::Resized { .. },
                ..
            } = event
            {
                self.scene.on_display_size_changed(self.engine.on_display_size_changed()?)?;
            }

            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                return Ok(true);
            }
        }

        self.previous_time = self.current_time;
        self.current_time = current_time(&self.timer);

        let elapsed = (self.current_time - self.previous_time) as f64;
        let delta = (elapsed / self.performance_frequency) as f32;

        self.scene.update(delta, &self.event_pump)?;
        self.engine.render_begin()?;
        self.scene.render(&mut self.engine)?;
        self.engine.render_end()?;

        Ok(false)
    }
}
