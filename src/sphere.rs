use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;
use crate::light::Light;
use crate::color::Color;

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Intersect {
        // Vector from the ray origin to the center of the sphere
        let oc = *ray_origin - self.center;

        // Coefficients for the quadratic equation
        let a = ray_direction.dot(*ray_direction);
        let b = 2.0 * oc.dot(*ray_direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        // Discriminant of the quadratic equation
        let discriminant = b * b - 4.0 * a * c;

        // The ray intersects the sphere if the discriminant is greater than zero
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            // We want the closest positive intersection
            let t = if t1 > 0.0 { t1 } else { t2 };

            if t > 0.0 {
                let point = *ray_origin + *ray_direction * t;
                let mut normal = point - self.center;
                normal.normalize();
                return Intersect::new(point, normal, t, self.material.clone());
            }
        }

        Intersect::empty()
    }
}

// Phong lighting model implementation
fn phong_lighting(intersect: &Intersect, light: &Light, view_dir: &Vector3) -> Color {
    // Ambient component
    let ambient_intensity = 0.1;
    let ambient = intersect.material.diffuse * ambient_intensity;

    // Diffuse component (Lambert)
    let mut light_dir = light.position - intersect.point;
    light_dir.normalize();
    let diffuse_intensity = intersect.normal.dot(light_dir).max(0.0);
    let diffuse = intersect.material.diffuse * (diffuse_intensity * intersect.material.albedo[0]);

    // Specular component (Phong)
    let reflect_dir = reflect(&(-light_dir), &intersect.normal);
    let spec_intensity = view_dir.dot(reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = Color::WHITE * (spec_intensity * intersect.material.albedo[1]);

    // Combine all components
    ambient + diffuse + specular
}

fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

pub fn cast_ray(ray_origin: &Vector3, ray_direction: &Vector3, objects: &[Box<dyn RayIntersect>], lights: &[Light]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(4, 12, 36);
    }

    // Calculate lighting using Phong model
    let mut view_dir = *ray_origin - intersect.point;
    view_dir.normalize();
    let mut final_color = Color::BLACK;

    for light in lights {
        let light_contribution = phong_lighting(&intersect, light, &view_dir);
        final_color = final_color + light_contribution;
    }

    final_color
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>], lights: &[Light]) {
    let width = framebuffer.width() as f32;
    let height = framebuffer.height() as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let mut ray_direction = Vector3::new(screen_x, screen_y, -1.0);
            ray_direction.normalize();

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&Vector3::new(0.0, 0.0, 0.0), &ray_direction, objects, lights);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color.to_raylib());
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_sphere(framebuffer: &mut Framebuffer) {
    // Create multiple sphere objects with Phong materials
    let sphere1 = Sphere {
        center: Vector3::new(1.0, 0.0, -4.0),
        radius: 1.0,
        material: Material::new(
            Color::new(255, 255, 240), // ivory color
            50.0,                       // specular exponent
            [0.6, 0.3]                  // albedo [diffuse, specular]
        ),
    };

    let sphere2 = Sphere {
        center: Vector3::new(2.0, 0.0, -5.0),
        radius: 1.0,
        material: Material::new(
            Color::new(139, 69, 19),    // brown/rubber color
            10.0,                       // lower specular exponent
            [0.9, 0.1]                  // more diffuse, less specular
        ),
    };

    // Create lights
    let lights = vec![
        Light::new(
            Vector3::new(-3.0, 3.0, -2.0),
            Color::WHITE,
            1.0
        ),
        Light::new(
            Vector3::new(3.0, -3.0, -2.0),
            Color::new(255, 100, 100), // reddish light
            0.5
        ),
    ];

    // Create a vector of objects
    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(sphere1),
        Box::new(sphere2),
    ];

    // Render the scene with lighting
    render(framebuffer, &objects, &lights);
}
