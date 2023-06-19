use egui::*;
use plot::{Plot, PlotResponse};
use rfd::FileDialog;
use std::fs;
use std::ffi::OsStr;
use std::io::{BufReader, Read, Write};
use std::process::exit;
use zip;
use svg2polylines::{self, Polyline};
use dxf::Drawing;
use dxf::entities::*;
use ngc::parse::parse;
use gen_gcode::*;

use crate::demo::Demo;

#[derive(PartialEq, Default, Debug)]
pub struct Toolpath {
    points_to_plot: Vec<[f64; 2]>
}

#[derive(Debug)]
struct BoundingBox {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64
}

impl super::Demo for Toolpath {
    fn name(&self) -> &'static str {
        "🗠 Toolpath"
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

impl super::View for Toolpath {
    #[allow(clippy::unused_self)]
        fn ui(&mut self, ui: &mut Ui) {
            let points = self.points_to_plot.clone();
            let mut line_to_plot = egui::widgets::plot::Line::new(points);
            let plot = Plot::new("Geometry").height(700.0).allow_scroll(false);

            let PlotResponse {
                response,
                inner: line_to_plot,
                ..
            } = plot.show(ui, |plot_ui| {
                (
                    plot_ui.line(line_to_plot),
                )
            });

            let ui_open_file = ui.button("Open file").on_hover_text("SVG and DXF are supported");
            let ui_toolpath_shrink = ui.button("Shrink").on_hover_text("Shrink the toolpath by 5mm");
            let ui_toolpath_grow = ui.button("Grow").on_hover_text("Grow the toolpath by 5mm");

            if ui_open_file.clicked(){
                let mut bounding_box = BoundingBox { min_x: 0.0, min_y: 0.0, max_x: 0.0, max_y: 0.0 };
                let mut file = FileDialog::new()
                    .add_filter("Cut files (zip, gcode, nc, ngc, svg, dxf)", &["zip", "gcode", "nc", "ngc", "svg", "dxf"])
                    .pick_file();

                match &file {
                    Some(path) => {
                        if path.extension().unwrap() == OsStr::new("gcode") { // Open a Gcode file

                        }
                        if path.extension().unwrap() == OsStr::new("nc") { // Open an NC file

                        }
                        if path.extension().unwrap() == OsStr::new("ngc") { // Open a LinuxCNC NGC file
                            let input = fs::read_to_string(path).unwrap();
                            match parse(path.file_name().unwrap().to_str().unwrap(), &input) {
                                Err(e) => eprintln!("Parse error: {}", e),
                                Ok(prog) => println!("{}", prog),
                            }
                        }
                        if path.extension().unwrap() == OsStr::new("dxf") { // Open a DXF file
                            let drawing = Drawing::load_file(path.as_os_str()).unwrap();

                            //Need to try to figure out how to get vviz crate to interpret circles. Probably will need to convert circle components to f32.
                            let mut list_of_vertices:Vec<[f32; 7]> = vec![[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]];
                            let mut indices:Vec<[i16; 2]> = vec![];
                            let mut index = 0;
                            self.points_to_plot = vec![];
                            for e in drawing.entities() {
                                println!("found entity on layer {}", e.common.layer);
                                match e.specific {
                                    EntityType::Circle(ref circle) => {
                                        // do something with the circle
                                        println!("{:#?}", circle);
                                        // Things that are needed are the center and the radius.
                                        // The center has an x, y, and z which are all f64, the radius is also f64.
                                    },
                                    EntityType::Line(ref line) => {
                                        println!("{:#?}", line);
                                        list_of_vertices.push([line.p1.x as f32, line.p1.y as f32, 0.0, 1.0, 0.0, 0.0, 1.0]);
                                        if line.p1.x > bounding_box.max_x { bounding_box.max_x = line.p1.x };
                                        if line.p1.x < bounding_box.min_x { bounding_box.min_x = line.p1.x };
                                        if line.p1.y > bounding_box.max_y { bounding_box.max_y = line.p1.y };
                                        if line.p1.y < bounding_box.min_y { bounding_box.min_y = line.p1.y };
                                        indices.push([index, index + 1]);
                                        index = index + 1;
                                        // Not sure if pushing p2 into the list of vertices is required since it seems like p2 in a line is the same as p1 of the next line.
                                        list_of_vertices.push([line.p2.x as f32, line.p2.y as f32, 0.0, 1.0, 0.0, 0.0, 1.0]);
                                        if line.p2.x > bounding_box.max_x { bounding_box.max_x = line.p2.x };
                                        if line.p2.x < bounding_box.min_x { bounding_box.min_x = line.p2.x };
                                        if line.p2.y > bounding_box.max_y { bounding_box.max_y = line.p2.y };
                                        if line.p2.y < bounding_box.min_y { bounding_box.min_y = line.p2.y };
                                        indices.push([index, index + 1]);

                                        self.points_to_plot.push([line.p1.x, line.p1.y]);
                                        self.points_to_plot.push([line.p2.x, line.p2.y]);

                                        index = index + 1;
                                    }, _ => (),
                                }
                            }

                            //if let Some(application_log) = self.state.demo.demo_windows.get_application_log() {
                            //    application_log.add_entry(format!("{:#?}", bounding_box));
                            //    application_log.add_entry(format!("{:#?}", self.points_to_plot));
                            //}

                            //super::application_log::ApplicationLog::add_entry(format!("{:#?}", bounding_box));
                            //super::application_log::ApplicationLog::add_entry(format!("{:#?}", self.points_to_plot));
                            println!("{:#?}", bounding_box);
                            println!("{:#?}", self.points_to_plot);

                        }
                    },
                    None    => println!("Please select a file"),
                }
            }
        }
}

fn is_approx_zero(val: f64) -> bool {
    val.abs() < 1e-6
}

fn is_approx_integer(val: f64) -> bool {
    val.fract().abs() < 1e-6
}