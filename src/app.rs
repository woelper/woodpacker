// #![feature(linked_list_cursors)]
#[macro_use]
use cut_optimizer_2d::*;
use serde::{Deserialize, Serialize};
use eframe::{egui::{self, Window}, epi};

use crate::{EguiDrawable, Orders, Piece, Template, solve_advanced};


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

impl epi::App for WoodPackerApp {

    fn name(&self) -> &str {
        "woodpacker"
    }

    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
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
                        if ui.button("-").clicked() {
                            removals.push(i);
                        }
                        if ui.button("+").clicked() {
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

            if ui.button("Add piece").clicked() {
                self.pieces.push(Piece::default())
            }

            if ui.button("Add template").clicked() {
                self.templates.push(Template::default())
            }

            if ui.button("Generate order").clicked() {
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

            if ui.button("Pack advanced").clicked() {
                dbg!("pck");

                self.solution = solve_advanced(&self.pieces, &self.templates[0]);
            }


            ui.add_space(16.0);

        });

        // integration_context.output.window_size = Some(ctx.used_size()); // resize the window to be just the size we need it to be
    }


}


