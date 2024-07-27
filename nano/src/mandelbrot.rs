use nannou::prelude::*;
use num_complex::Complex;

struct Model {
    zoom: f32,
    offset: Vec2,
    max_iterations: u32,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 600).view(view).key_pressed(key_pressed).build().unwrap();
    Model {
        zoom: 0.25,
        offset: Vec2::new(-0.5, 0.0),
        max_iterations: 100,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //Update logic here
    model.max_iterations = (100.0 * model.zoom.log10()).max(100.0) as u32;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // View logic here
    frame.clear(BLACK);
    let draw = app.draw();
    let zoom = model.zoom;
    let offset = model.offset;

    let window = app.window_rect();
    let aspect_ratio = window.w() / window.h();

    for x in 0..window.w() as i32 {
        for y in 0..window.h() as i32 {
            let real = (x as f32 / window.w() - 0.5) * 4.0 * aspect_ratio / zoom + offset.x;
            let imag = (y as f32 / window.h() - 0.5) * 4.0 / zoom + offset.y;
            let c = Complex::new(real, imag);

            let (escaped, iterations) = mandelbrot(c, model.max_iterations);
            if escaped {
                let hue = (iterations as f32 / model.max_iterations as f32) * 360.0;
                let color = hsv(hue, 1.0, 1.0);
                draw.rect()
                    .x_y(x as f32 - window.w() / 2.0, y as f32 - window.h() / 2.0)
                    .w_h(1.0, 1.0)
                    .color(color);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn mandelbrot(c: Complex<f32>, max_iterations: u32) -> (bool, u32) {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..max_iterations {
        if z.norm_sqr() > 2.0 {
            return (true, i);
        }
        z = z * z + c;
    }
    (false, max_iterations)
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Z => {
            let zoom_factor = 1.3;
            let mouse_pos = app.mouse.position();
            model.zoom *= zoom_factor;
            model.offset += (mouse_pos / app.window_rect().wh()) * (4.0 / model.zoom) * (1.0 - 1.0 / zoom_factor);
        }
        Key::X => {
            let zoom_factor = 1.0 / 1.3;
            let mouse_pos = app.mouse.position();
            model.zoom *= zoom_factor;
            model.offset += (mouse_pos / app.window_rect().wh()) * (4.0 / model.zoom) * (1.0 - 1.0 / zoom_factor);
        }
        _ => {}
    }
}
