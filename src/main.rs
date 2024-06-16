mod color;
mod position;

use std::{fs::File, io::Write};

use color::Rgb;
use position::Position;

const PATH: &str = "./image.ppm";

fn save(content: &str) -> anyhow::Result<()> {
    let mut file = File::create(PATH)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

#[derive(Debug)]
struct PPMImage {
    aspect_ratio: f32,
    width: u32,
    height: u32,
    pixels: Vec<Rgb>,
}

impl PPMImage {
    fn new(aspect_ratio: f32, width: u32) -> Self {
        Self {
            aspect_ratio,
            width,
            height: (width as f32 / aspect_ratio) as u32,
            pixels: Vec::new(),
        }
    }

    fn to_text(&self) -> String {
        let mut out = format!("P3 {} {} 255\n", self.width, self.height);
        for pixel in &self.pixels {
            out.push_str(&pixel.to_ppm())
        }
        out
    }
}

type Point = Position;

type Vector = Position;

#[derive(Debug)]
struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    fn new(origin: Position, direction: Vector) -> Self {
        Self { origin, direction }
    }

    fn at(&self, t: u32) -> Point {
        &self.direction * t + &self.origin
    }

    fn color(&self) -> Rgb {
        let a = 0.5 * (self.direction.y() + 1.0);
        Rgb::from_percent(1.0, 1.0, 1.0) * (1.0 - a) + Rgb::from_percent(0.5, 0.7, 1.0) * a
    }
}

#[derive(Debug)]
struct Camera {
    focal_length: f32,
    viewport_height: f32,
    viewport_width: u32,
    camera_center: Point,
    viewport_horizontal: Vector,
    viewport_vertical: Vector,
    pixel_delta_horizontal: Vector,
    pixel_delta_vertical: Vector,
    viewport_upper_left: Vector,
    pixel00_loc: Vector,
}

impl Camera {
    fn new(viewport_height: f32, image: &PPMImage) -> Self {
        let viewport_width = viewport_height as u32 * (image.width / image.height);
        let viewport_horizontal = Vector::new(viewport_width as f32, 0.0, 0.0);
        let viewport_vertical = Vector::new(0.0, -viewport_height, 0.0);
        let camera_center = Point::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_upper_left = camera_center
            - Vector::new(0.0, 0.0, focal_length)
            - viewport_horizontal / 2
            - viewport_vertical / 2;
        let pixel_delta_horizontal = viewport_horizontal / image.width;
        let pixel_delta_vertical = viewport_vertical / image.height;
        Self {
            focal_length,
            viewport_height: 2.0,
            viewport_width,
            camera_center,
            viewport_horizontal,
            viewport_vertical,
            pixel_delta_horizontal,
            pixel_delta_vertical,
            viewport_upper_left,
            pixel00_loc: dbg!(
                viewport_upper_left + (pixel_delta_horizontal + pixel_delta_vertical) * 0.5
            ),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let mut image = PPMImage::new(aspect_ratio, width);
    let camera: Camera = Camera::new(2.0, &image);

    let pixels: Vec<_> = (0..image.height)
        .flat_map(|scan_heigh| {
            (0..image.width)
                .map(|scan_width| {
                    let pixel_center = camera.pixel00_loc
                        + (camera.pixel_delta_horizontal * scan_width)
                        + (camera.pixel_delta_vertical * scan_heigh);
                    let ray_direction = pixel_center - camera.camera_center;
                    let ray = Ray::new(camera.camera_center, ray_direction);
                    ray.color()
                })
                .collect::<Vec<_>>()
        })
        .collect();
    image.pixels = pixels;

    save(&image.to_text())
}

#[test]
fn test_pixel_from_u32() {
    let p = Rgb::from_u32(0xA0FFB0);
    assert_eq!(p.red, 0xA0);
    assert_eq!(p.green, 0xFF);
    assert_eq!(p.blue, 0xB0);
}
