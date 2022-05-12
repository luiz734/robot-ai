use nannou::prelude::*;
use nannou::Event;
mod robot;
mod ui;
use robot::{Nannou, Robot};
use ui::UI;

fn main() {
    nannou::app(model).event(event).run();
}

struct Input {
    pub up: f32,
    pub right: f32,
    pub down: f32,
    pub left: f32,
    pub rotation: f32,
}

struct Model {
    _window: window::Id,
    robot: Robot,
    input: Input,
    ui: UI,
    obstacles: Vec<Rect>,
    colliding_points: (Vec<Point2>, Vec<f32>),
}
fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let model = Model {
        _window,
        robot: Robot::new(),
        input: Input {
            up: 0.0,
            right: 0.0,
            down: 0.0,
            left: 0.0,
            rotation: 0.0,
        },
        ui: UI::new(&app.window_rect()),
        obstacles: vec![Rect::from_xy_wh(pt2(-100.0, -100.0), vec2(40.0, 40.0))],
        colliding_points: (Vec::new(), Vec::new()),
    };
    model
}

fn update(_app: &App, model: &mut Model, _event: Event) {
    model.robot.set_linear_velocity(
        model.input.right - model.input.left,
        model.input.up - model.input.down,
    );
    model.robot.set_angular_velocity(model.input.rotation);
    model.robot.update();
    model.colliding_points = model.robot.get_closest_collisions(&model.obstacles);

    // concats all values to be displayed and show them
    let mut display_values = model.colliding_points.1.to_owned();
    display_values.push(model.robot.get_rotation());

    // TODO: fix rotation values
    model.ui.update_display_text(&display_values);
}
fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => match simple {
            Some(e) => match e {
                KeyPressed(key) => match key {
                    Key::W => model.input.up = 1.0,
                    Key::D => model.input.right = 1.0,
                    Key::S => model.input.down = 1.0,
                    Key::A => model.input.left = 1.0,
                    // rotation
                    Key::J => model.input.rotation = -1.0,
                    Key::K => model.input.rotation = 1.0,

                    _ => {}
                },
                KeyReleased(key) => match key {
                    Key::W => model.input.up = 0.0,
                    Key::D => model.input.right = 0.0,
                    Key::S => model.input.down = 0.0,
                    Key::A => model.input.left = 0.0,
                    // rotation
                    Key::J => model.input.rotation = 0.0,
                    Key::K => model.input.rotation = 0.0,
                    _ => {}
                },
                _ => {}
            },
            None => {}
        },
        Event::Update(_) => update(_app, model, event),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    // draw obstacles
    for obstacle in model.obstacles.iter() {
        draw.rect()
            .xy(obstacle.xy())
            .wh(obstacle.wh())
            .color(INDIANRED);
    }
    // draw points
    for point in model.colliding_points.0.iter() {
        draw.ellipse().xy(point.to_owned()).radius(4.0).color(RED);
    }

    model.robot.draw(&draw);

    model.ui.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
