use std::fs::create_dir_all;

use renderer::{
    primitives::{GeometricPrimitive, TransformedPrimitive},
    renderers::WhittedRayTracer,
    shapes::implicit_shapes::Cube,
    Camera, Color, Light, Material, Scene, Transform, Vect,
};

fn main() {
    let (width, height) = (512, 512);
    //let (width, height) = (1920, 1080);

    // Create the output directory
    create_dir_all("output").expect("Can't create output folder");

    // Generate the frames
    let camera = {
        let pos = Vect::new(-3., 0., 0.);
        let dir = -1. * pos.normalized();
        Camera {
            pos,
            dir,
            width,
            height,
        }
    };

    let mut scene = Scene::new(camera);

    scene.add_light(Light {
        pos: Vect::new(-1., 0., 0.9),
        intensity: 1.,
    });

    let white = Material {
        color: Color::white(),
    };
    let red = Material {
        color: Color::red(),
    };
    let green = Material {
        color: Color::green(),
    };

    let cube_left = TransformedPrimitive::new(
        Box::new(GeometricPrimitive::new(Box::new(Cube {}), red)),
        Transform::new_translation(Vect::new(0., 2., 0.)),
    );
    scene.add_primitive(Box::new(cube_left));

    let cube_right = TransformedPrimitive::new(
        Box::new(GeometricPrimitive::new(Box::new(Cube {}), green)),
        Transform::new_translation(Vect::new(0., -2., 0.)),
    );
    scene.add_primitive(Box::new(cube_right));

    let cube_top = TransformedPrimitive::new(
        Box::new(GeometricPrimitive::new(Box::new(Cube {}), white)),
        Transform::new_translation(Vect::new(0., 0., 2.)),
    );
    scene.add_primitive(Box::new(cube_top));

    let cube_bottom = TransformedPrimitive::new(
        Box::new(GeometricPrimitive::new(Box::new(Cube {}), white)),
        Transform::new_translation(Vect::new(0., 0., -2.)),
    );
    scene.add_primitive(Box::new(cube_bottom));

    let cube_back = TransformedPrimitive::new(
        Box::new(GeometricPrimitive::new(Box::new(Cube {}), white)),
        Transform::new_translation(Vect::new(2., 0., 0.)),
    );
    scene.add_primitive(Box::new(cube_back));

    let small_cube = TransformedPrimitive::new(
        Box::new(GeometricPrimitive::new(Box::new(Cube {}), white)),
        Transform::new_scaling(0.3, 0.3, 0.5)
            .add(&Transform::new_translation(Vect::new(0., -0.2, -0.5)))
            .add(&Transform::new_z_rotation(1.)),
    );
    scene.add_primitive(Box::new(small_cube));

    let light_cube = TransformedPrimitive::new(
        Box::new(GeometricPrimitive::new(Box::new(Cube {}), Material {color: Color::new(1., 0., 1.)})),
        Transform::new_uniform_scaling(0.3)
            .add(&Transform::new_translation(Vect::new(0., 0., 1.2))),
    );
    scene.add_primitive(Box::new(light_cube));

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
        .save("output/output.png")
        .expect("Could not save the image");
}
