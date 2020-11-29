#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use egui;
use egui::containers::Window;
use egui::widgets::{DragValue, Slider};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

/// A `Template` is a piece that is available from a vendor.
#[derive(Serialize, Deserialize, Debug, Default)]
struct Template {
    length: f64,
    width: f64,
    height: f64,
    name: String,
    #[serde(default)]
    price: f32,
}

impl Template {
    fn equals(&self, piece: &Piece) -> bool {
        self.width == piece.width && self.height == piece.height
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("{} {} {} ", self.length, self.width, self.height));
        ui.horizontal(|ui| {
            ui.add(DragValue::f64(&mut self.length));
            ui.add(DragValue::f64(&mut self.width));
            ui.add(DragValue::f64(&mut self.height));
        });
    }
}

/// A `Piece`: A single entity that is part of your design
#[derive(Serialize, Deserialize, Debug, Default)]
struct Piece {
    length: f64,
    width: f64,
    height: f64,
}

impl Piece {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("{} {} {} ", self.length, self.width, self.height));
        ui.horizontal(|ui| {
            ui.add(DragValue::f64(&mut self.length));
            ui.add(DragValue::f64(&mut self.width));
            ui.add(DragValue::f64(&mut self.height));
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    name: String,
    remaining_length: f64,
    cuts: Vec<f64>,
    number: usize,
    price: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Orders {
    items: Vec<Order>,
    sum: HashMap<String, i32>,
    price: f32,
}

impl Orders {
    fn add(&mut self, piece: &Piece, template: &Template) {
        for mut order in &mut self.items {
            if order.name == template.name && piece.length <= order.remaining_length {
                order.cuts.push(piece.length);
                order.remaining_length -= piece.length;
                println!("Added segment {:?} to {:?}", piece, order);
                return;
            }
        }
        // add new order item
        let o = Order {
            name: template.name.clone(),
            remaining_length: template.length - piece.length,
            cuts: vec![piece.length],
            number: self.items.len(),
            price: template.price,
        };
        // println!("Created new order item {:?}", o);

        self.items.push(o);
    }

    fn sum(&mut self) {
        for item in &self.items {
            dbg!(&item.name);
            let stat = self.sum.entry(item.name.clone()).or_insert(0);
            *stat += 1;
            self.price += item.price;
            // self.sum.insert(item.name.clone(), 1);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleApp {
    templates: Vec<Template>,
    pieces: Vec<Piece>,
    orders: Orders,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            templates: vec![],
            pieces: vec![],
            orders: Orders::default(),
        }
    }
}

impl egui::app::App for ExampleApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        let ExampleApp {
            templates,
            pieces,
            orders,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My Egui Application");

            Window::new("Pieces").show(ui.ctx(), |ui| {
                for p in &mut self.pieces {
                    p.draw(ui)
                }
            });

            Window::new("Templates").show(ui.ctx(), |ui| {
                for t in &mut self.templates {
                    t.draw(ui)
                }
            });

            if ui.button("Add piece").clicked {
                self.pieces.push(Piece::default())
            }

            if ui.button("Add template").clicked {
                self.templates.push(Template::default())
            }

            if ui.button("comp").clicked {
                let mut orders = Orders {
                    items: vec![],
                    ..Default::default()
                };
                for piece in &self.pieces {
                    for template in &self.templates {
                        if template.equals(&piece) {
                            // dbg!(&piece);
                            // println!("{:?} == {:?}", template, piece);
                            orders.add(&piece, &template);
                        }
                    }
                }
                orders.sum();
                self.orders = orders;
            }

            ui.advance_cursor(16.0);
            if ui.button("Quit").clicked {
                integration_context.output.quit = true;
            }
        });

        // integration_context.output.window_size = Some(ctx.used_size()); // resize the window to be just the size we need it to be
    }

    fn on_exit(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, egui::app::APP_KEY, self);
    }
}

fn main() {
    // let storage = egui_glium::storage::FileStorage::from_path(".egui_demo_glium.json".into());
    let storage = egui::app::DummyStorage::default();

    //let app: egui::DemoApp = egui::app::get_value(&storage, egui::app::APP_KEY).unwrap_or_default();
    let app: ExampleApp = egui::app::get_value(&storage, egui::app::APP_KEY).unwrap_or_default();

    egui_glium::run("Woodpacker", Box::new(storage), app);

    //let reader = BufReader::new(File::open("templates.json").unwrap());
    //let templates: Vec<Template> = serde_json::from_reader(reader).unwrap();

    //let reader = BufReader::new(File::open("pieces.json").unwrap());
    //let pieces: Vec<Piece> = serde_json::from_reader(reader).unwrap();

    // let mut orders = Orders {
    //     items: vec![],
    //     ..Default::default()
    // };

    // for piece in pieces {
    //     for template in &templates {
    //         if template.equals(&piece) {
    //             // dbg!(&piece);
    //             // println!("{:?} == {:?}", template, piece);
    //             orders.add(&piece, &template);
    //         }
    //     }
    // }

    // orders.sum();

    // println!("{:?}, price {}", orders.sum, orders.price);

    // let writer = BufWriter::new(File::create("order.json").unwrap());
    // serde_json::to_writer_pretty(writer, &orders).unwrap();
}
