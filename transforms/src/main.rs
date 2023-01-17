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
    shapes::sphere::Sphere,
    transform::Transform,
    vect::Vect,
};

fn main() {
    let width = 640;
    let height = 360;

    let camera = {
        let pos = Vect::new(0., -8., 6.);
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
        pos: Vect::new(0., 0., 100.),
        intensity: 1.,
    });

    let use_transform = true;
    if use_transform {
        let sphere = GeometricPrimitive::new(Box::new(Sphere::new(Vect::new(0., 0., 1.), 1.)));

        let transform = Transform::new_scaling(2., 2., 2.);
        let sphere = TransformedPrimitive::new(Box::new(sphere), transform);

        scene.add_primitive(Box::new(sphere));
    } else {
        let sphere = GeometricPrimitive::new(Box::new(Sphere::new(Vect::new(0., 0., 2.), 2.)));

        scene.add_primitive(Box::new(sphere));
    }

    let checkerboard = Checkerboard::new(
        Vect::new(-3., -3., 0.),
        6.,
        6.,
        12,
        12,
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

    output.save("output.png").expect("Could not save the image");
}
