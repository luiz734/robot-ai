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
        velocity: vec2(-10.0, -10.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.position += model.velocity * (1.0 / 60.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let obstacle = Rect::from_xy_wh(pt2(-100.0, -100.0), vec2(40.0, 40.0));
    let mut obstacle_color = INDIANRED;

    let radius = 100.0;
    let angles = vec![-PI / 4.0, -PI / 6.0, 0.0, PI / 4.0, PI / 6.0];
    let rotation = app.time.sin() * 2.0 * PI;
    let position = model.position;

    let robot: Rect<f32> = Rect::from_xy_wh(position, vec2(radius, radius));
    let sensors = draw_robot(&draw, robot, rotation, angles);

    for sensor in sensors {
        if collision_line_rect(sensor, obstacle) {
            obstacle_color = RED;
            break;
        }
    }
    draw.rect()
        .xy(obstacle.xy())
        .wh(obstacle.wh())
        .color(obstacle_color);

    draw.to_frame(app, &frame).unwrap();
}
fn collision_line_rect(line: (Point2, Point2), rect: Rect) -> bool {
    let rect_sides = vec![
        (rect.top_left(), rect.top_right()),
        (rect.top_right(), rect.bottom_right()),
        (rect.bottom_right(), rect.bottom_left()),
        (rect.bottom_left(), rect.top_left()),
    ];
    for side in rect_sides {
        if collision_line_line(line, side) {
            return true;
        }
    }
    return false;
}
fn collision_line_line(line_a: (Point2, Point2), line_b: (Point2, Point2)) -> bool {
    let (x1, y1, x2, y2) = (line_a.0.x, line_a.0.y, line_a.1.x, line_a.1.y);
    let (x3, y3, x4, y4) = (line_b.0.x, line_b.0.y, line_b.1.x, line_b.1.y);

    // ---
    // https://www.jeffreythompson.org/collision-detection/line-line.php
    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    // parallel lines
    if denominator == 0.0 {
        return false;
    }
    let u_a = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;
    let u_b = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;

    u_b >= 0.0 && u_b <= 1.0 && u_a >= 0.0 && u_a <= 1.0
    // ---
}
fn draw_robot(
    draw: &Draw,
    robot: Rect,
    rotation: f32,
    sensor_angles: Vec<f32>,
) -> Vec<(Point2, Point2)> {
    // robot
    draw.ellipse()
        .xy(robot.xy())
        .wh(robot.wh())
        .color(STEELBLUE);

    // front indicator
    let front_x = rotation.sin() * (robot.w() / 2.0);
    let front_y = rotation.cos() * (robot.w() / 2.0);

    let front_indicator: Rect<f32> = Rect::from_xy_wh(robot.xy(), pt2(20.0, 20.0));
    // let front_indicator = front_indicator.align_top_of(robot);
    draw.ellipse()
        .xy(front_indicator.xy() + pt2(front_x, front_y))
        .wh(front_indicator.wh())
        .color(CORAL);

    draw_raycast_sensors(draw, robot.xy(), robot.w() * 2.0, sensor_angles, rotation)
}

fn draw_raycast_sensors(
    draw: &Draw,
    pivot: Point2,
    length: f32,
    angles_from_center: Vec<f32>,
    robot_rotation: f32,
) -> Vec<(Point2, Point2)> {
    let mut raycasts = Vec::new();
    // offsets angle to match the robot front
    let angle_offset = -PI / 2.0 + robot_rotation;
    for angle in angles_from_center {
        // calculates the scaled (x,y) endpoint based the robot rotation
        let x = (angle - angle_offset).cos() * length;
        let y = (angle - angle_offset).sin() * length;
        let end_pos = pivot + pt2(x, y);
        raycasts.push((pivot, end_pos));

        draw.line()
            .start(pivot)
            .end(end_pos)
            .weight(2.0)
            .color(DARKCYAN);
    }
    raycasts
}
