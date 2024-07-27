use nannou::prelude::*;
use std::path::PathBuf;

    fn main() {
        nannou::app(model)
            .update(update)
            .run();
    }

    struct Model {


        texture: wgpu::Texture, 

    }

    fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024, 1024).view(view).build().unwrap();
    let path = PathBuf::from("src/images/image.png");

 
                                
    let texture = wgpu::Texture::from_path(app, path).unwrap();

        Model {
            texture
        }

    }

    fn update(app: &App, model: &mut Model, update: Update) {
        // Update logic here
    }

    fn view(app: &App, model: &Model, frame: Frame) {
        // View logic here
        let w = 100.0;
        let h = 100.0;
        
        let win = app.window_rect(); 
        
        let x1 = random_range(win.left(), win.right()); 
        let y1 = random_range(win.top(), win.bottom()); 
        let x2 = x1 + random_range(-10.0, 10.0); 
        let y2 = y1 + random_range(-10.0, 10.0); 
        
        
        
        let area = geom::Rect::from_x_y_w_h(
            map_range(x1, win.left(), win.right(), 0.0, 1.0),
            map_range(y1, win.left(), win.right(), 0.0, 1.0), 
            map_range(w, 0.0, win.w(), 0.0, 1.0), 
            map_range(h, 0.0, win.h(), 0.0, 1.0));
            
            
            
            let sampler = wgpu::SamplerBuilder::new()
            .min_filter(wgpu::FilterMode::Nearest)
            .mag_filter(wgpu::FilterMode::Nearest)
            .into_descriptor(); 

        
        
        let draw = app.draw().sampler(sampler);
        if frame.nth() == 0{
            frame.clear(WHITE); 
            draw.texture(&model.texture); 

        } 
        else {
            draw.texture(&frame.resolve_target().unwrap_or(frame.texture_view()))
                .x_y(x2, y2)
                .w_h(w, h)
                .area(area);

        }
    draw.to_frame(app, &frame).unwrap();
    }