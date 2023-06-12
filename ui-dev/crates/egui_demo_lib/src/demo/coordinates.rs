use egui::*;

#[derive(PartialEq, Default, Debug)]
pub struct Coordinates {}

pub struct Cartesian;

pub trait CoordinateSystem {
    fn to_cartesian(&self, position: &Self::Position) -> [f64; 3];
    fn from_cartesian(&self, position: &[f64; 3]) -> Self::Position;
    fn calculate_emus(&self, start: &Self::Position, end: &Self::Position) -> Vec<Self::Emu>;
    fn calculate_steps(&self, start: &Self::Position, end: &Self::Position) -> Vec<Self::Step>;
}

impl CoordinateSystem for Cartesian {
    type Position = [f64; 3];
    type Step = (usize, f64); // (axis, step)

    fn to_cartesian(&self, position: &Self::Position) -> [f64; 3] {
        *position // No conversion needed for Cartesian coordinates
    }

    fn from_cartesian(&self, position: &[f64; 3]) -> Self::Position {
        *position // No conversion needed for Cartesian coordinates
    }

    fn calculate_steps(&self, start: &Self::Position, end: &Self::Position) -> Vec<Self::Step> {
        // Implement the logic for calculating steps in Cartesian coordinate system
    }
}

pub struct Delta;

impl CoordinateSystem for Delta {
    type Position = [f64; 3];
    type Step = (usize, f64); // (axis, step)

    fn to_cartesian(&self, position: &Self::Position) -> [f64; 3] {
        // Implement the forward kinematics for Delta
    }

    fn from_cartesian(&self, position: &[f64; 3]) -> Self::Position {
        // Implement the inverse kinematics for Delta
    }

    fn calculate_steps(&self, start: &Self::Position, end: &Self::Position) -> Vec<Self::Step> {
        // Implement the logic for calculating steps in Delta coordinate system
    }
}

pub struct Polar;

impl CoordinateSystem for Polar {
    type Position = (f64, f64, f64); // (radius, theta, z)
    type Step = (usize, f64); // (axis, step)

    fn to_cartesian(&self, position: &Self::Position) -> [f64; 3] {
        let (radius, theta, z) = *position;
        let x = radius * theta.cos();
        let y = radius * theta.sin();

        [x, y, z]
    }

    fn from_cartesian(&self, position: &[f64; 3]) -> Self::Position {
        let [x, y, z] = *position;
        let radius = (x * x + y * y).sqrt();
        let theta = y.atan2(x);

        (radius, theta, z)
    }

    fn calculate_steps(&self, start: &Self::Position, end: &Self::Position) -> Vec<Self::Step> {
        // Implement the logic for calculating steps in Polar coordinate system
    }
}


impl Coordinates {
    pub fn from_string(s: &str) -> Option<Self> {
        let mut parts = s.split_whitespace();
        let code = parts.next().unwrap_or("").to_ascii_uppercase();
        match code.as_str() {
            "G0" => Some(GCode::G0 {
                x: Self::parse_optional_value(parts.next(), "X"),
                y: Self::parse_optional_value(parts.next(), "Y"),
                z: Self::parse_optional_value(parts.next(), "Z"),
                e: Self::parse_optional_value(parts.next(), "E"),
                f: Self::parse_optional_value(parts.next(), "F"),
            }),
            "M405" => Some(GCode::M405 {
                delay: Self::parse_optional_value(parts.next(), "D"),
            }),
            "M406" => Some(GCode::M406),
            "M407" => Some(GCode::M407),
        }
    }
}

impl super::Demo for Coordinates {
    fn name(&self) -> &'static str {
        "🗠 Coordinates"
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

impl super::View for Coordinates {
    #[allow(clippy::unused_self)]
    fn ui(&mut self, ui: &mut Ui) {

    }
}
