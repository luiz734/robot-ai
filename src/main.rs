use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let radius = 100.0;

    let robot: Rect<f32> = Rect::from_w_h(radius, radius);
    draw.ellipse()
        .color(STEELBLUE)
        .xy(robot.xy())
        .wh(robot.wh());

    let front_indicator = Rect::from_w_h(20.0, 20.0);
    let front_indicator = front_indicator.align_top_of(robot);
    draw.ellipse()
        .color(CORAL)
        .xy(front_indicator.xy())
        .wh(front_indicator.wh());

    let angles = vec![-45.0, -30.0, 0.0, 30.0, 45.0];
    draw_raycast_sensors(&draw, robot.xy(), radius * 2.0, angles);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_raycast_sensors(draw: &Draw, pivot: Point2, length: f32, angles_from_center: Vec<f32>) {
    let front_offset = 180.0;
    for angle in angles_from_center {
        draw.line()
            .start(pivot)
            .end(pt2(pivot.x, pivot.y - length))
            .z_degrees(angle + front_offset)
            .weight(2.0)
            .color(DARKCYAN);
    }
}
