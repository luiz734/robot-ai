use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    position: Point2,
    velocity: Vec2,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        position: pt2(0.0, 0.0),
        velocity: vec2(10.0, 10.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.position += model.velocity * (1.0 / 60.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let radius = 100.0;
    let angles = vec![-PI / 4.0, -PI / 6.0, 0.0, PI / 4.0, PI / 6.0];
    let rotation = app.time.sin() * 2.0 * PI;
    let position = model.position;

    let robot: Rect<f32> = Rect::from_xy_wh(position, vec2(radius, radius));
    draw_robot(&draw, robot, rotation, angles);
    draw.to_frame(app, &frame).unwrap();
}
fn draw_robot(draw: &Draw, robot: Rect, rotation: f32, sensor_angles: Vec<f32>) {
    // robot
    draw.ellipse()
        .color(STEELBLUE)
        .xy(robot.xy())
        .wh(robot.wh());

    // front indicator
    let front_x = rotation.sin() * (robot.w() / 2.0);
    let front_y = rotation.cos() * (robot.w() / 2.0);

    let front_indicator: Rect<f32> = Rect::from_xy_wh(robot.xy(), pt2(20.0, 20.0));
    // let front_indicator = front_indicator.align_top_of(robot);
    draw.ellipse()
        .color(CORAL)
        .xy(front_indicator.xy() + pt2(front_x, front_y))
        .wh(front_indicator.wh());

    draw_raycast_sensors(draw, robot.xy(), robot.w() * 2.0, sensor_angles, rotation);
}

fn draw_raycast_sensors(
    draw: &Draw,
    pivot: Point2,
    length: f32,
    angles_from_center: Vec<f32>,
    robot_rotation: f32,
) -> Vec<Point2> {
    let mut collision_points = Vec::new();
    // offsets angle to match the robot front
    let angle_offset = PI / 2.0 - robot_rotation;
    for angle in angles_from_center {
        // calculates the scaled (x,y) endpoint based the robot rotation
        let x = (angle - angle_offset).cos() * length;
        let y = (angle - angle_offset).sin() * length;
        let end_pos = pivot + pt2(x, y);
        collision_points.push(end_pos);

        draw.line()
            .start(pivot)
            .end(end_pos)
            .weight(2.0)
            .color(DARKCYAN);
    }
    collision_points
}
