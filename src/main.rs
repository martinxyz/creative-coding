use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct State {
    x: f32,
    r: f32,
    iteration: u32,
}

impl State {
    fn init() -> Self {
        Self {
            x: random_f32(),
            r: random_f32() * 4.0 + 0.5,
            iteration: 0,
        }
    }
    fn step(&mut self) {
        // logistic map formula
        self.x = self.r * self.x * (1.0 - self.x);
        self.iteration += 1;
    }
}

const N: usize = 20_000;

struct Model {
    _window: window::Id,
    states: Vec<State>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model {
        _window,
        states: (0..N).map(|_| State::init()).collect(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let prob = 1. - sigmoid((app.elapsed_frames() as f32 + 500.) as f32 / 500.);
    println!("frame {} reset probability {:?}", app.elapsed_frames(), prob);
    for state in model.states.iter_mut() {
        if random_f32() < prob {
            *state = State::init();
        } else {
            state.step()
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    const SCALE_X: f32 = 1024.0 * 1.4;
    const SCALE_Y: f32 = 1024.0 * 0.4;

    if app.elapsed_frames() == 1 {
        draw.background().color(WHITE);
    }

    let color_fac = sigmoid((500. - app.elapsed_frames() as f32) / 1500.);

    for state in model.states.iter() {
        let x = state.r / 5.0;
        let y = state.x;
        let it_fac = 1.0 / (state.iteration as f32 + 2.) + 0.5;
        draw.ellipse()
            .rgba(
                color_fac * 0.9,
                color_fac * 0.3,
                color_fac * 0.2,
                (1.0 - it_fac) * 0.5,
            )
            .x(x * SCALE_X - SCALE_X / 2.0)
            .y(y * SCALE_Y - SCALE_Y / 2.0)
            .radius(0.05 + it_fac * 0.0);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}
