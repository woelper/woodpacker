#![feature(linked_list_cursors)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use cut_optimizer_2d::*;
use egui;
use egui::containers::Window;
use serde::{Deserialize, Serialize};

mod pack;
use pack::*;

#[derive(Serialize, Deserialize)]
pub struct WoodPackerApp {
    templates: Vec<Template>,
    pieces: Vec<Piece>,
    orders: Orders,
    solution: Option<Solution>,
}

impl Default for WoodPackerApp {
    fn default() -> Self {
        Self {
            templates: vec![],
            pieces: vec![],
            orders: Orders::default(),
            solution: None,
        }
    }
}

impl egui::app::App for WoodPackerApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        let WoodPackerApp {
            templates,
            pieces,
            orders,
            solution,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My Egui Application");

            Window::new("Pieces").show(ui.ctx(), |ui| {
                ui.label("Pieces are parts of your design. Measured in length * width * height");

                let mut removals: Vec<usize> = vec![];
                let mut clones: Vec<usize> = vec![];
                for (i, p) in self.pieces.iter_mut().enumerate() {

                    p.draw(ui);
                    ui.horizontal(|ui| {
                        if ui.button("-").clicked {
                            removals.push(i);
                        }
                        if ui.button("+").clicked {
                            clones.push(i);
                        }

                    });
                }

                for r in removals {
                    self.pieces.remove(r);
                }


                for r in clones {
                    self.pieces.push(self.pieces[r].clone());
                }

          
            });

            Window::new("Templates").show(ui.ctx(), |ui| {
                ui.label("Templates are the items available\nat a store. Measured length * width * height");

                for t in &mut self.templates {
                    t.draw(ui)
                }
            });

            Window::new("Order").show(ui.ctx(), |ui| {
                self.orders.draw(ui);
            });



            if let Some(solution) = &self.solution {
                Window::new("Solution").show(ui.ctx(), |ui| {
                    for s in &solution.stock_pieces {
                        s.draw(ui);
                    }
                });
            }

            if ui.button("Add piece").clicked {
                self.pieces.push(Piece::default())
            }

            if ui.button("Add template").clicked {
                self.templates.push(Template::default())
            }

            if ui.button("Generate order").clicked {
                let mut orders = Orders {
                    items: vec![],
                    ..Default::default()
                };

                for piece in &self.pieces {
                    for template in &self.templates {
                        //dbg!(&template);

                        // if template.equals(&piece) {
                            // dbg!(&piece);
                            // println!("{:?} == {:?}", template, piece);
                            orders.add(&piece, &template);
                        // }
                    }
                }
                orders.sum();
                self.orders = orders;
            }

            if ui.button("Pack advanced").clicked {
                dbg!("pck");

                self.solution = solve_advanced(&self.pieces, &self.templates[0]);
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
    let storage = egui_glium::storage::FileStorage::from_path(".woodpacker.json".into());
    // let storage = egui::app::DummyStorage::default();

    //let app: egui::DemoApp = egui::app::get_value(&storage, egui::app::APP_KEY).unwrap_or_default();
    let app: WoodPackerApp = egui::app::get_value(&storage, egui::app::APP_KEY).unwrap_or_default();

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
