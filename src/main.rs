use nannou::prelude::*;
use rayon::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

const N: usize = 2048;

struct State {
    x: f32,
    r: f32,
}

impl State {
    fn init() -> Self {
        return Self {
            x: random_f32(),
            r: random_f32() * 4.0 + 0.5,
        }
    }
    fn update(&mut self) {
        self.x = self.r * self.x * (1.0 - self.x);
    }
}

struct Model {
    _window: window::Id,
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
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}


fn sample_states() -> Vec<State>
{
    let mut states: Vec<_> = (0..N).map(|_| State::init()).collect();
    states.par_chunks_mut(8).for_each(|chunk| {
        for _ in 0..1000_000_000 {
            chunk.iter_mut().for_each(State::update);
        }
    });
    states
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    const SCALE_X: f32 = 1024.0 * 1.4;
    const SCALE_Y: f32 = 1024.0 * 0.4;

    if app.elapsed_frames() < 2 {
        draw.background().color(WHITE);
    }


    let points = sample_states().into_iter().map(|state| {
        let x = state.r / 5.0;
        let y = state.x;
        pt2(x * SCALE_X - SCALE_X/2.0, y * SCALE_Y - SCALE_Y/2.0)
    });
    draw
    // .blend(BLEND_DARKEST)
        .point_mode()
        .polyline()
        .weight(0.1)
        .rgba(0., 0., 0., 0.2)
        .points(points);
    draw.to_frame(app, &frame).unwrap();
}

fn sigmoid(x: f32) -> f32 {
    return 1.0 / (1.0 + (-x).exp());
}
