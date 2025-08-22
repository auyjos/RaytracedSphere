mod framebuffer;
mod sphere;
mod ray_intersect;
mod color;
mod light;
mod material;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use sphere::render_sphere;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title( "Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .resizable()  // Enable window resizing
        .build();
        

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::BLACK);

    framebuffer.set_background_color(Color::new(50,50,100,255));
    framebuffer.clear();
    
    render_sphere(&mut framebuffer);

    while !window.window_should_close(){
        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        if current_width != framebuffer.width() || current_height != framebuffer.height() {
            // Resize the framebuffer to match the new window size
            framebuffer.resize(current_width as u32, current_height as u32);
            framebuffer.clear();
            render_sphere(&mut framebuffer);
        }
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
