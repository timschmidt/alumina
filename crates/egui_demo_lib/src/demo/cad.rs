use egui::*;
extern crate kiss3d;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::nalgebra::{UnitQuaternion, Vector3};

pub struct Cad {
    c: SceneNode,
    rot: UnitQuaternion<f32>,
}

impl Default for Cad {
    fn default() -> Self {
        // Provide appropriate default values for the fields
        Cad {
            c: SceneNode::new(/* Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>> */, /* Isometry<f32, kiss3d::nalgebra::Unit<Quaternion<f32>>, 3> */, /* std::option::Option<kiss3d::scene::Object> */),
            rot: UnitQuaternion::identity(),
        }
    }
}

impl super::Demo for Cad {
    fn name(&self) -> &'static str {
        "🗠 Mesh"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .default_size(vec2(1200.0, 800.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Cad {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {
        let mut window = kiss3d::window::Window::new("Kiss3d: wasm example");
        let mut c = window.add_cube(1.0, 1.0, 1.0);

        c.set_color(1.0, 0.0, 0.0);

        window.set_light(Light::StickToCamera);

        let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
        let state = Cad { c, rot };

        window.render_loop(state)
    }
}

impl kiss3d::window::State for Cad {
    fn step(&mut self, _: &mut Window) {
        self.c.prepend_to_local_rotation(&self.rot)
    }
}