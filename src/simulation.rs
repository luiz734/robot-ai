use crate::robot::{Nannou, Robot};
use crate::ui::UI;
use nannou::prelude::{Event, *};
use std::sync::mpsc;
use std::thread;
pub struct SensorsOutput {
    values: Vec<f32>,
}
impl SensorsOutput {
    fn new(values: Vec<f32>) -> Self {
        Self { values }
    }
}
impl ToString for SensorsOutput {
    fn to_string(&self) -> String {
        let mut message = String::new();
        for v in self.values.iter() {
            message += format!("{} ", v.to_string()).as_str();
        }
        message
    }
}
pub struct RobotInput {
    linear_velocity: f32,
    angular_velocity: f32,
}
impl RobotInput {
    fn new(linear_velocity: f32, angular_velocity: f32) -> Self {
        Self {
            linear_velocity,
            angular_velocity,
        }
    }
}

pub struct Model {
    _window: window::Id,
    robot: Robot,
    ui: UI,
    obstacles: Vec<Rect>,
    colliding_points: (Vec<Point2>, Vec<f32>),
    sensors_sender: mpsc::Sender<SensorsOutput>,
    input_receiver: mpsc::Receiver<RobotInput>,
}

// nannou
pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let (sensors_sender, sensors_receiver) = mpsc::channel();
    let (input_sender, input_receiver) = mpsc::channel();

    let mut model = Model {
        _window,
        robot: Robot::new(),
        ui: UI::new(&app.window_rect()),
        obstacles: Vec::new(),
        colliding_points: (Vec::new(), Vec::new()),
        sensors_sender,
        input_receiver,
    };

    thread::spawn(move || loop {
        match sensors_receiver.try_recv() {
            Ok(sensors_output) => {
                println!("{} ", sensors_output.to_string());
            }
            Err(_) => (),
        };
        // not a problem if doest send
        if let Ok(_) = input_sender.send(RobotInput::new(2.0, 1.0)) {}
    });

    create_obstacle(&mut model, app.window_rect());

    model
}
pub fn update(_app: &App, model: &mut Model, _event: Event) {
    // try to get new input. if get some, update the robot input value
    match model.input_receiver.try_recv() {
        Ok(robot_input) => {
            model.robot.set_linear_velocity(robot_input.linear_velocity);
            model
                .robot
                .set_angular_velocity(robot_input.angular_velocity);
        }
        Err(_) => {}
    };

    model.robot.update();
    model.colliding_points = model.robot.get_closest_collisions(&model.obstacles);
    // concats all values to be displayed and show them
    let mut display_values = model.colliding_points.1.to_owned();
    display_values.push(model.robot.get_mode_numeric());
    model.ui.update_display_text(&display_values);

    // send the current state of sensors. it's ok if if doesn't succeed
    if let Ok(_) = model
        .sensors_sender
        .send(SensorsOutput::new(display_values))
    {}
}
pub fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => match simple {
            Some(e) => match e {
                KeyPressed(key) => match key {
                    // Key::H => model.input.set_angular_velocity(-1.0),
                    // Key::L => model.input.set_angular_velocity(1.0),
                    // Key::K => model.input.set_linear_velocity(1.0),
                    // Key::J => model.input.set_linear_velocity(-1.0),
                    _ => {}
                },
                KeyReleased(key) => match key {
                    // Key::H => model.input.set_angular_velocity(0.0),
                    // Key::L => model.input.set_angular_velocity(0.0),
                    // Key::K => model.input.set_linear_velocity(0.0),
                    // Key::J => model.input.set_linear_velocity(0.0),
                    Key::Space => model.robot.toggle_mode(),

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
pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(INDIANRED);

    // draw obstacles
    for obstacle in model.obstacles.iter() {
        draw.rect().xy(obstacle.xy()).wh(obstacle.wh()).color(PLUM);
    }
    // draw points
    for point in model.colliding_points.0.iter() {
        draw.ellipse().xy(point.to_owned()).radius(4.0).color(RED);
    }

    model.robot.draw(&draw);

    model.ui.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

// helper functions
fn create_obstacle(model: &mut Model, win_rect: Rect) {
    let thickness: f32 = 40.0;
    let ui_rect = model.ui.get_rect();
    let valid_area = Rect::from_w_h(
        win_rect.w() - ui_rect.w() - 2.0 * thickness,
        win_rect.h() - 2.0 * thickness,
    )
    .top_right_of(win_rect)
    .shift_x(-thickness)
    .shift_y(-thickness);

    model.obstacles.append(&mut vec![valid_area]); //top_wall, right_wall, bottom_wall, left_wall]);
}
