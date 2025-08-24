mod framebuffer;
mod sphere;
mod ray_intersect;
mod color;
mod light;
mod material;
mod camera;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use sphere::{render, Sphere};
use camera::CustomCamera;
use light::Light;
use material::Material;
use color::Color;
use std::f32::consts::PI;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 600; // Reducido para mejor rendimiento
    let framebuffer_height = 450;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Rust Graphics - Raytracer with Reflections & Refractions")
        .log_level(TraceLogLevel::LOG_WARNING)
        .resizable()
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, raylib::color::Color::BLACK);

    framebuffer.set_background_color(raylib::color::Color::new(50, 50, 100, 255));
    framebuffer.clear();

    // Materials and Objects - Mejor distribución y tamaños
    let objects = [
        // Esfera de goma (roja, izquierda)
        Sphere { 
            center: Vector3::new(-3.0, 0.0, -5.0), 
            radius: 1.2, 
            material: Material::rubber() 
        },
        // Esfera de marfil (centro)
        Sphere { 
            center: Vector3::new(0.0, 0.0, -4.0), 
            radius: 1.0, 
            material: Material::ivory() 
        },
        // Esfera espejo (derecha)
        Sphere { 
            center: Vector3::new(3.0, 0.0, -5.0), 
            radius: 1.2, 
            material: Material::mirror() 
        },
        // Esfera de vidrio (arriba)
        Sphere { 
            center: Vector3::new(0.0, 2.5, -4.5), 
            radius: 1.0, 
            material: Material::glass() 
        },
        // Esfera grande como "suelo" - Más lejos para mejor perspectiva
        Sphere { 
            center: Vector3::new(0.0, -1001.5, -1.0), 
            radius: 1000.0, 
            material: Material::new(
                Color::new(120, 120, 120),
                20.0,
                [0.7, 0.3, 0.1, 0.0], // Ligeramente reflectante
                1.0,
                0.0,
            )
        },
    ];

    // Initialize camera - Mejor posición inicial
    let mut camera = CustomCamera::new(
        Vector3::new(0.0, 1.0, 8.0),     // Más lejos y ligeramente elevada
        Vector3::new(0.0, 0.0, -4.0),    // Mirando hacia el centro de la escena
        Vector3::new(0.0, 1.0, 0.0)      // Up vector
    );
    let rotation_speed = PI / 50.0;

    // Múltiples luces - Mejor posicionamiento e intensidad
    let lights = [
        // Luz principal (blanca, arriba y adelante)
        Light::new(
            Vector3::new(-3.0, 4.0, 1.0),
            Color::new(255, 255, 255),
            1.5 // Más intensa
        ),
        // Luz de relleno (cálida, desde otro ángulo)
        Light::new(
            Vector3::new(3.0, 2.0, -1.0),
            Color::new(255, 220, 180),
            1.0
        ),
        // Luz trasera sutil (para destacar bordes)
        Light::new(
            Vector3::new(0.0, 1.0, -8.0),
            Color::new(200, 200, 255),
            0.6
        ),
    ];

    while !window.window_should_close() {
        // Camera orbit controls
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.orbit(0.0, rotation_speed);
        }

        // Check if window was resized
        let current_width = window.get_screen_width();
        let current_height = window.get_screen_height();
        
        if current_width != framebuffer.width() || current_height != framebuffer.height() {
            framebuffer.resize(current_width as u32, current_height as u32);
            framebuffer.clear();
        }

        // Render the scene
        render(&mut framebuffer, &objects, &camera, &lights);
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
