use nannou::prelude::*;

fn main() {
    nannou::app(model)
    .update(update)
    .run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
    .new_window().
    view(view)
    .build()
    .unwrap();

    Model { 
        _window 
    }
}

fn update(
    _app: &App,
     _model: &mut Model,
      _update: Update) {}


fn view(
    app: &App, 
    _model: &Model, 
    frame :Frame
){
    let draw = app.draw(); 
    

    let sine  = app.time.sin(); 
    let cos = app.time.tan(); 

    let boundary = app.window_rect(); 

    let x = map_range(
        sine, 
        -1.0,
        1.0, 
        boundary.left(), 
        boundary.right()
    ); 
    let y = map_range(
        cos, 
        -1.0,
        1.0, 
        boundary.top(), 
        boundary.bottom()

    ); 

    draw.background().color(PURPLE);

    draw.ellipse().color(CYAN).x_y(x, y);

    draw.to_frame(app, &frame).unwrap(); 






}