use std::f32::INFINITY;

use nannou::prelude::*;

pub trait Nannou {
    fn draw(&self, draw: &Draw);
    fn update(&mut self);
}
// TODO: make Line a struct
type Line = (Point2, Point2);

pub struct Sensors {
    pub length: f32,
    pub angles: Vec<f32>,
    pub lines: Vec<Line>,
}
impl Sensors {
    pub fn new() -> Self {
        Self {
            length: 0.0,
            angles: vec![-PI / 4.0, -PI / 6.0, 0.0, PI / 6.0, PI / 4.0],
            lines: Vec::new(),
        }
    }
}
pub struct Robot {
    position: Point2,
    rotation: f32,
    linear_velocity: Point2,
    angular_velocity: f32,
    sensors: Sensors,
    speed: f32,
    radius: f32,
}
impl Robot {
    pub fn new() -> Self {
        let mut robot = Self {
            position: pt2(0.0, 0.0),
            rotation: 0.0,
            linear_velocity: vec2(0.0, 0.0),
            angular_velocity: 0.0,
            sensors: Sensors::new(),
            speed: 50.0,
            radius: 40.0,
        };
        robot.sensors.length = 200.0;

        return robot;
    }
    fn draw_sensors(&self, draw: &Draw) {
        // offsets angle to match the robot front
        for line in self.sensors.lines.iter() {
            draw.line()
                .start(line.0)
                .end(line.1)
                .weight(2.0)
                .color(DARKCYAN);
        }
    }
    fn update_sensors(&mut self) {
        // TODO: stop creating. instead, chage values
        self.sensors.lines = Vec::new();
        for angle in self.sensors.angles.iter() {
            // calculates the scaled (x,y) endpoint based the robot rotation
            let angle_offset = -PI / 2.0 + self.rotation;
            let x = (angle - angle_offset).cos() * self.sensors.length;
            let y = (angle - angle_offset).sin() * self.sensors.length;
            let end_pos = self.position + pt2(x, y);
            self.sensors.lines.push((self.position, end_pos));
        }
    }
    pub fn get_closest_collisions(&self, obstacles: &Vec<Rect>) -> (Vec<Point2>, Vec<f32>) {
        let mut collision_points: Vec<Point2> = Vec::new();
        let mut distances: Vec<f32> = Vec::new();

        for sensor in self.sensors.lines.iter() {
            for obstacle in obstacles.iter() {
                let points = collision_line_rect(sensor, obstacle);
                // gets the closest colliding point
                let mut min_distance = self.sensors.length;
                let mut min_point = pt2(INFINITY, INFINITY);
                for p in points {
                    if let Some(p) = p {
                        let distance = self.distance_to(p);
                        if distance < min_distance {
                            min_distance = distance;
                            min_point = p.clone();
                        }
                    }
                }
                let mapped_distance = map_range(min_distance, 0.0, 200.0, 1.0, 0.0);
                collision_points.push(min_point);
                distances.push(mapped_distance);
            }
        }
        (collision_points, distances)
    }
    pub fn distance_to(&self, point: Point2) -> f32 {
        return self.position.distance(point);
    }
    pub fn set_linear_velocity(&mut self, x: f32, y: f32) {
        self.linear_velocity = pt2(x, y);
    }
    pub fn set_angular_velocity(&mut self, value: f32) {
        self.angular_velocity = value;
    }
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
}
impl Nannou for Robot {
    fn draw(&self, draw: &Draw) {
        let rect = Rect::from_xy_wh(self.position, vec2(self.radius, self.radius));
        draw.ellipse().xy(rect.xy()).wh(rect.wh()).color(STEELBLUE);

        // front indicator
        let front_x = self.rotation.sin() * (self.radius / 2.0);
        let front_y = self.rotation.cos() * (self.radius / 2.0);
        let front_indicator: Rect<f32> = Rect::from_xy_wh(rect.xy(), pt2(8.0, 8.0));
        // let front_indicator = front_indicator.align_top_of(robot);
        draw.ellipse()
            .xy(front_indicator.xy() + pt2(front_x, front_y))
            .wh(front_indicator.wh())
            .color(CORAL);

        self.draw_sensors(draw);
    }
    fn update(&mut self) {
        self.position += self.speed * self.linear_velocity * (1.0 / 60.0);
        self.rotation += self.angular_velocity * (1.0 / 60.0);

        self.update_sensors();
    }
}
pub fn collision_line_rect(line: &(Point2, Point2), rect: &Rect) -> Vec<Option<Point2>> {
    let rect_sides = vec![
        (rect.top_left(), rect.top_right()),
        (rect.top_right(), rect.bottom_right()),
        (rect.bottom_right(), rect.bottom_left()),
        (rect.bottom_left(), rect.top_left()),
    ];
    // check collision for all rect sides
    let mut sensor_input = Vec::new();
    for side in rect_sides {
        let current = collision_line_line(line, &side);
        sensor_input.push(current);
    }
    sensor_input
}
/// Check for intersection between the lines. It there is an intersection, returns the intersection point
pub fn collision_line_line(line_a: &(Point2, Point2), line_b: &(Point2, Point2)) -> Option<Point2> {
    let (x1, y1, x2, y2) = (line_a.0.x, line_a.0.y, line_a.1.x, line_a.1.y);
    let (x3, y3, x4, y4) = (line_b.0.x, line_b.0.y, line_b.1.x, line_b.1.y);

    // ---
    // https://www.jeffreythompson.org/collision-detection/line-line.php
    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    // parallel lines
    if denominator == 0.0 {
        return None;
    }
    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denominator;

    if u_b >= 0.0 && u_b <= 1.0 && u_a >= 0.0 && u_a <= 1.0 {
        let intersection_x = x1 + (u_a * (x2 - x1));
        let intersection_y = y1 + (u_a * (y2 - y1));
        return Some(pt2(intersection_x, intersection_y));
    }
    return None;
    // ---
}
