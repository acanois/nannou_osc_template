use nannou::prelude::*;

use nannou_osc as osc;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

/// Struct holding the state of the app
struct Model {
    sender: osc::Sender<osc::Connected>,
}

fn model(_app: &App) -> Model {
    let port = 10000;
    let target_addr = format!("{}:{}", "127.0.0.1", port);

    let sender = osc::sender()
        .expect("Could not bind to default socket")
        .connect(target_addr)
        .expect("Could not connect to socket at address");

    Model { sender }
}

/// Updates Model 
fn update(_app: &App, _model: &mut Model, _update: Update) {

}

/// What to draw to the screen
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();
    let boundary = app.window_rect();
    
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    let osc_addr = "/circle/position".to_string();
    let args = vec![osc::Type::Float(x), osc::Type::Float(y)];
    let packet = (osc_addr, args);

    model.sender.send(packet).ok();

    draw.background().color(DARKGRAY);
    draw.ellipse().color(INDIGO).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
