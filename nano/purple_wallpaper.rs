use nannou::{noise::{self, Perlin}, prelude::*}; 
use nannou::noise::*; 


fn main(){ 
    nannou::app(model)
    .update(update)
    .run();

}

struct Point {
    position: Vec2, 
}

impl Point {
    pub fn new (p:Vec2) -> Self {
        Point {
            position: p 
        }
    }
}

struct Model {
    Points: Vec<Point>,
    noise : OpenSimplex, 

}


const NO_OF_POINTS: usize = 2000; 

fn model(app : &App)->Model{
    let _window = app.new_window().size(1024, 1024).view(view).build().unwrap();
    let mut Points = Vec::new(); 

    for i in 0..NO_OF_POINTS{
        let Point = Point::new(vec2(
            (random_f32()-0.5)*1024.0,
            (random_f32()-0.5)*1024.0,
        )); 
        Points.push(Point); 

    }

    let noise = OpenSimplex::new();

    Model {
    Points, 
    noise, 
    }


}

fn update(
    app : &App, 
    model : &mut Model, 
    update: Update
){
    let scaling = 0.01; 

    for Point in model.Points.iter_mut(){
        Point.position += vec2(
            model.noise.get([scaling*Point.position.x as f64, scaling*Point.position.y as f64, 0.0]) as f32,
            model.noise.get([scaling*Point.position.x as f64,scaling*Point.position.y as f64, 1.0]) as f32 ); 

    }

}

fn view(
    app: &App, 
    model : &Model,
    frame: Frame,  

){

    let draw = app.draw(); 

    
    let time = app.elapsed_frames() as f32 / 60.0; 
    if app.elapsed_frames()== 1 {

        draw.background().color(PLUM);
    }

    

    for Point in model.Points.iter() { 
        draw.ellipse().xy(Point.position).radius(5.0).color(PURPLE); 
        
    }



    draw.to_frame(app, &frame).unwrap();



    
}


