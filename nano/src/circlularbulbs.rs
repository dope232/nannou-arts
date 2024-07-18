use std::vec;

use nannou::prelude::*; 

fn main(){
    nannou::app(model)
    .update(update)
    .run();

}

struct Model {
    p1: Vec2, 
    p2: Vec2, 
    p3: Vec2, 
}

fn model(
    app: &App, 

)->Model{
    app.new_window()
    .size(1024, 1024)
    .view(view)
    .build()
    .unwrap(); 
    Model {
        p1: vec2(0.0, 0.0),
        p2: vec2(0.0, 0.0), 
        p3: vec2(0.0,0.0),

    }

}




fn view(
    app: &App, 
    model : &Model,
    frame: Frame,   
){
    let draw = app.draw(); 

    




    
    
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(1024.0, 1024.0)
        .color(rgba(0.0, 0.0, 0.22, 0.005));
    
    draw.ellipse()
        .xy(model.p1)
        .wh(model.p2)
        .color(rgba(0.255, 0.255, 0.89, 0.01))
        .stroke(rgba(0.57, 0.02, 0.79, 0.1))
        .stroke_weight(1.0);


        draw.ellipse()
        .xy(model.p3)
        .wh(model.p2)
        .color(rgba(0.255, 0.255, 0.89, 0.01))
        .stroke(rgba(0.57, 0.02, 0.79, 0.1))
        .stroke_weight(1.0);

        draw.ellipse()
        .xy(model.p3/-2.0)
        .wh(model.p2)
        .color(rgba(0.255, 0.255, 0.89, 0.01))
        .stroke(rgba(0.57, 0.02, 0.79, 0.1))
        .stroke_weight(1.0);

        draw.ellipse()
        .xy(model.p1/3.23)
        .wh(model.p2)
        .color(rgba(0.255, 0.255, 0.89, 0.01))
        .stroke(rgba(0.57, 0.02, 0.79, 0.1))
        .stroke_weight(1.0);





    
    draw.to_frame(app, &frame).unwrap();
   
    




}

fn update(
    app : &App, 
    model : &mut Model, 
    update: Update
){

    let time = (app.elapsed_frames() as f32) / 60.0;
    let w = (time/2.0).sin() * 150.0 + 150.0;
    let h = (time/2.0).cos() * 150.0 + 150.0;
    let x = (time /7.0).cos() * 500.0;
    let y = (time /4.0).sin() * 500.0;
    
    

    model.p1 = vec2(x,y);
    model.p2 = vec2(w,h);
    model.p3 = vec2(-1.0*y, -1.0*x); 

    
}