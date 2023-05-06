use std::fs::create_dir_all;

use renderer::{
    primitives::{GeometricPrimitive, TransformedPrimitive},
    renderers::{MonteCarloRenderer, WhittedRayTracer, monte_carlo_renderer::SamplingMethod},
    shapes::implicit_shapes::Cube,
    Camera, Color, Light, Material, Renderer, Scene, Transform, Vect,
};

// Size of the output image
static SIZE: usize = 512;

// Select the renderer used:
// - if false, WhittedRenderer
// - if true, MonteCarloRenderer
static USE_MONTE_CARLO: bool = false;

// If a Monte Carlo renderer is used, the sampling method
static SAMPLING_METHOD: SamplingMethod = SamplingMethod::RegularGrid;

fn main() {
    // Create the output directory
    create_dir_all("output").expect("Can't create output folder");

    let mut scene: Scene = Scene::new({
        let pos = Vect::new(-3., 0., 0.);
        let dir = Vect::new(1., 0., 0.);
        Camera {
            pos,
            dir,
            width: SIZE,
            height: SIZE,
        }
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

    let renderer: Box<dyn Renderer> = if USE_MONTE_CARLO {
        let light_cube = TransformedPrimitive::new(
            Box::new(GeometricPrimitive::new(
                Box::new(Cube {}),
                Material {
                    color: Color::new(1., 0., 1.),
                },
            )),
            Transform::new_uniform_scaling(0.3)
                .add(&Transform::new_translation(Vect::new(-1., 0., 1.2))),
        );
        scene.add_primitive(Box::new(light_cube));

        Box::new(MonteCarloRenderer {
            iterations_per_pixel: 100,
            sampling_method: SAMPLING_METHOD,
        })
    } else {
        scene.add_light(Light {
            pos: Vect::new(-1., 0., 0.9),
            intensity: 1.,
        });

        Box::new(WhittedRayTracer {})
    };

    renderer.render(scene);
}
