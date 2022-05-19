mod robot;
mod simulation;
mod ui;
use simulation::{event, model};

fn main() {
    nannou::app(model).event(event).run();
}
