use nannou::lyon::geom::arc;
use nannou::prelude::*;
use nannou::lyon; 



fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {

    recursion_level: u8, 
    start_radius: f32, 


}

fn model(app: &App) -> Model {
let _window = app.new_window()
.size(1024, 1024).
view(view)
.key_released(key_released)
.build()
.unwrap();


    Model {
        recursion_level: 6,
        start_radius: 200.0,

    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    // Update logic here
}

fn view(app: &App, model: &Model, frame: Frame) {
    // View logic here
let draw = app.draw();

draw.background().color(WHITE); 

draw_branch(
    &draw,
    0.0,
    0.0,
    model.start_radius,
    model.recursion_level,

);


draw.to_frame(app, &frame).unwrap();
}

// recurisve function 


fn draw_branch(draw: &Draw,
x: f32,
y: f32, 
radius : f32, 
level: u8, 
){



    use nannou::geom::path::Builder; 
    let mut builder = Builder::new().with_svg(); 
    builder.move_to(lyon::math::point(x-radius, y)); 
    builder.arc(
    lyon::math::point(x, y),
    lyon::math::vector(radius,radius),
    lyon::math::Angle::two_pi(),
    lyon::math::Angle::radians(90.0)
); 



       
    let arc_path = builder.build(); 


    // draw arc 

    draw.path()
    .stroke()
    .stroke_weight(level as f32 * 1.0)
    .rgba(0.0, 0.0, 0.0 , 1.0)
    .caps_round()
    .events(arc_path.iter()); 


   

    if level > 0 {
        draw_branch(&draw, x-radius, y-radius, radius/2.0, level-1,); //left
        draw_branch(&draw, x+radius, y-radius/2.0, radius/2.0, level-1,); //right 


    }






}


fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => model.recursion_level = 1,
        Key::Key2 => model.recursion_level = 2,
        Key::Key3 => model.recursion_level = 3,
        Key::Key4 => model.recursion_level = 4,
        Key::Key5 => model.recursion_level = 5,
        Key::Key6 => model.recursion_level = 6,
        Key::Key7 => model.recursion_level = 7,
        Key::Key8 => model.recursion_level = 8,
        Key::Key9 => model.recursion_level = 9,
        Key::Key0 => model.recursion_level = 0,
        
        _other_key => {}
    }
}
