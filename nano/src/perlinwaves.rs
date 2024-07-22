use nannou::{noise::{self, Perlin, Seedable, NoiseFn}, prelude::*};


fn main() {
    nannou::app(model)
        .update(update)
        .run();
}


struct Agent {
    vector: Vec2,
    vector_old: Vec2,
    step_size: f32,
    angle: f32,
    is_outside: bool,
    win_rect: Rect,
}
impl Agent {



    fn new(win_rect: Rect) ->Self {

        let vector = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.top(), win_rect.bottom()),



        ); 
        Agent {
            vector,
            vector_old: vector,
            step_size: random_range(1.0, 5.0),
            angle: 0.0,
            is_outside: false,
            win_rect,}

    
}

    fn update_default(&mut self,){

        self.is_outside = false;
        self.vector_old = self.vector;
        self.vector.x += self.angle.cos() * self.step_size;
        self.vector.y += self.angle.sin() * self.step_size;
        self.is_outside = self.vector.x < self.win_rect.left() || self.vector.x > self.win_rect.right() || self.vector.y < self.win_rect.bottom() || self.vector.y > self.win_rect.top();
        if self.is_outside {
            self.vector = vec2(
                random_range(self.win_rect.left(), self.win_rect.right()),
                random_range(self.win_rect.top(), self.win_rect.bottom()),   
            ); 
            self.vector_old = self.vector;    
        }
    }

    fn display(&self, draw: &Draw, stroke_weight: f32, agent_alpha: f32) {
        draw.line()
            .start(self.vector_old)
            .end(self.vector)
            .rgba(1.0, 1.0, 1.0, agent_alpha)
            .stroke_weight(stroke_weight);
    }

    fn update(&mut self, noise: Perlin, noise_scale: f64, noise_strength: f64) {
        let n = noise.get([
            self.vector.x as f64 / noise_scale,
            self.vector.y as f64 / noise_scale,
        ]) * noise_strength;
        self.angle = n as f32;
    }


    }





struct Model {

    agents : Vec<Agent>, 
    noise_scale : f64,
    noise_strength : f64, 
    overlay: f32, 
    agent_alpha: f32, 
    stroke: f32, 
    draw_mode: u8, 
    noise_seed : u32, 



}

fn model(app: &App) -> Model {
let _window = app.new_window().size(1024, 1024).view(view).build().unwrap();

    const NO_OF_AGENTS : u32 = 4000; 
    let mut agents = Vec::new(); 

    for i in 0..NO_OF_AGENTS{
        let Agent = Agent::new(app.window_rect()); 
        agents.push(Agent); 


    }


    Model {
        agents, 
        noise_scale: 800.0,
        noise_strength: 50.0,
        overlay: 0.07,
        agent_alpha: 0.75,
        stroke: 0.2,
        draw_mode: 1,
        noise_seed: 3,


    }
}





fn update(app: &App, model: &mut Model, update: Update) {
    // Update logic here


    let noise = Perlin::new().set_seed(model.noise_seed); 

    for agent in &mut model.agents{
        match model.draw_mode {
            1 => agent.update(noise, model.noise_scale, model.noise_strength),
            
            
            _ => (),
        }
        agent.update_default();

    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    // View logic here
let draw = app.draw();

if frame.nth() == 0  {
    draw.background().color(BLACK);
} 
else {
   draw.rect().wh(app.window_rect().wh()).rgba(0.0, 0.0, 0.0, model.overlay);
}

model.agents.iter().for_each(|agent| {
    agent.display(&draw, model.stroke,  model.agent_alpha);
});

draw.to_frame(app, &frame).unwrap();
}




