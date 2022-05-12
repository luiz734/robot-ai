use nannou::prelude::*;
use nannou::Event;
mod robot;
use robot::{Nannou, Robot};

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    _window: window::Id,
    robot: Robot,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let model = Model {
        _window,
        robot: Robot::new(vec2(-10.0, -10.0), 1.0),
    };
    model
}

fn update(_app: &App, model: &mut Model, _event: Event) {
    model.robot.update();
}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id, simple } => match simple {
            Some(e) => match e {
                KeyPressed(key) => match key {
                    Key::Space => {
                        println!("hello");
                    }
                    _ => {}
                },
                _ => {}
            },
            None => {}
        },
        Event::Update(_) => update(_app, _model, event),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let obstacle = Rect::from_xy_wh(pt2(-100.0, -100.0), vec2(40.0, 40.0));
    let mut obstacle_color = INDIANRED;
    draw.rect()
        .xy(obstacle.xy())
        .wh(obstacle.wh())
        .color(obstacle_color);

    for point in model.robot.check_collision(&vec![obstacle]) {
        draw.ellipse().xy(point).radius(4.0).color(RED);
    }
    model.robot.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
