use std::{f64::consts::PI, fs::create_dir_all, process::Command};

use renderer::{
    camera::Camera,
    color::Color,
    light::Light,
    primitives::{
        checkerboard::Checkerboard, geometric_primitive::GeometricPrimitive,
        transformed_primitive::TransformedPrimitive,
    },
    renderers::whitted_ray_tracer::WhittedRayTracer,
    scene::Scene,
    shapes::implicit_shapes::cube::Cube,
    transform::Transform,
    vect::Vect,
};

const FRAMES_PER_TURN: i32 = 60;

fn main() {
    //let (width, height) = (640, 360);
    let (width, height) = (1920, 1080);

    // Create the output directory
    create_dir_all("output").expect("Can't create output folder");

    // Generate the frames
    for frame in 0..3 * FRAMES_PER_TURN {
        let camera = {
            let pos = Vect::new(-3., -8., 5.);
            let dir = -2.5 * pos.normalized();
            Camera {
                pos,
                dir,
                width,
                height,
            }
        };

        let mut scene = Scene::new(camera);

        scene.add_light(Light {
            pos: Vect::new(-3., -2., 10.),
            intensity: 1.,
        });

        let cube = GeometricPrimitive::new(Box::new(Cube {}));

        let transform = {
            let part = frame / FRAMES_PER_TURN;
            let pos = (frame % FRAMES_PER_TURN) as f64 / FRAMES_PER_TURN as f64;
            let theta = 2. * PI * pos;
            let scaling = 0.7 * (1. - 0.7 * (0.5 - (pos - 0.5).abs()));

            if part == 0 {
                Transform::new_identity()
                    .add(&Transform::new_z_rotation(theta))
                    .add(&Transform::new_translation(Vect::new(2., 0., 1.)))
                    .add(&Transform::new_uniform_scaling(scaling))
            } else if part == 1 {
                Transform::new_identity()
                    .add(&Transform::new_translation(Vect::new(2., 0., 1.)))
                    .add(&Transform::new_z_rotation(theta))
                    .add(&Transform::new_uniform_scaling(scaling))
            } else {
                Transform::new_identity()
                    .add(&Transform::new_z_rotation(PI / 4.))
                    .add(&Transform::new_scaling(1. + 3. * (0.5 - (pos - 0.5).abs()), 1., 1.))
                    .add(&Transform::new_z_rotation(PI / 4.))
                    .add(&Transform::new_translation(Vect::new(2., 0., 1.)))
                    .add(&Transform::new_uniform_scaling(0.7))
            }
        };

        let cube = TransformedPrimitive::new(Box::new(cube), transform);

        scene.add_primitive(Box::new(cube));

        let checkerboard = Checkerboard::new(
            Vect::new(-6., -6., 0.),
            12.,
            12.,
            24,
            24,
            Color::new(0., 1., 1.),
            Color::new(1., 0., 1.),
        );

        scene.add_primitive(Box::new(checkerboard));

        // Render
        let renderer = WhittedRayTracer {};

        let img = scene.render(&renderer);

        // Write the output image
        let mut output: image::RgbImage = image::ImageBuffer::new(width, height);

        for x in 0..width {
            for y in 0..height {
                output.put_pixel(
                    x,
                    height - y - 1,
                    image::Rgb([
                        img[x as usize][y as usize].0,
                        img[x as usize][y as usize].1,
                        img[x as usize][y as usize].2,
                    ]),
                );
            }
        }

        output
            .save(format!("output/{:04}.png", frame))
            .expect("Could not save the image");
    }

    // Generate the output video
    Command::new("ffmpeg")
        .args(["-y", "-framerate", "60", "-i", "%04d.png", "output.mp4"])
        .current_dir("output")
        .status()
        .expect("Can't create video");
}
