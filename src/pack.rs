use cut_optimizer_2d::*;
use eframe::egui::{self, Color32};
use egui::math::Pos2;
use egui::Stroke;
use egui::widgets::{DragValue, Slider};
use egui::{Painter};
use std::collections::{ HashMap};
use serde::{Serialize, Deserialize};

fn draw_rect(ui: &mut egui::Ui, dim: Pos2, offset: Pos2, fill: bool) {
    let mut paint_rect = ui.available_rect_before_wrap();
    paint_rect.min += offset.to_vec2();
    paint_rect.max = paint_rect.min + dim.to_vec2();
    // paint_rect.max.x = dim.x;
    // paint_rect.max.y = dim.y;
    // paint_rect.max.x = height;

    let color = Color32::from_rgba_unmultiplied(128, 128, 128, 255);
    let fill_color = Color32::from_rgba_unmultiplied(64, 64, 128, 128);

    if fill {
        ui.painter()
        .rect(paint_rect, 0., fill_color, Stroke::new(2., color));

    } else {

        ui.painter()
            .rect_stroke(paint_rect, 0., Stroke::new(2., color));
    }
}

pub trait EguiDrawable {
    fn draw(&self, ui: &mut egui::Ui);
}

impl EguiDrawable for ResultStockPiece {
    fn draw(&self, ui: &mut egui::Ui) {
        // ui.label("Result stock piece");

        draw_rect(
            ui,
            Pos2::new(self.width as f32, self.length as f32),
            Pos2::new(0., 0.),
            false
        );
        for p in &self.cut_pieces {
            p.draw(ui);
        }
    }
}

impl EguiDrawable for ResultCutPiece {
    fn draw(&self, ui: &mut egui::Ui) {
        // ui.label("Result cut piece");
        draw_rect(
            ui,
            Pos2::new(self.width as f32, self.length as f32),
            Pos2::new(self.x as f32, self.y as f32),
            true
        );
    }
}

pub fn solve_advanced(pieces: &Vec<Piece>, template: &Template) -> Option<Solution> {
    if let Ok(solution) = Optimizer::new()
        .add_cut_pieces(pieces.iter().map(|x| CutPiece {
            can_rotate: true,
            external_id: 0,
            length: x.length as usize,
            width: x.width as usize,
            pattern_direction: PatternDirection::None,
        }))
        .add_stock_piece(StockPiece {
            pattern_direction: PatternDirection::None,
            width: template.width as usize,
            length: template.length as usize,
        })
        .optimize_guillotine(|_| {})
    {
        return Some(solution);
        // for p in solution.stock_pieces {
        //     dbg!(&p);
        // }
    }
    None
}

/// A `Template` is a piece that is available from a vendor.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Template {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub name: String,
    #[serde(default)]
    pub price: f32,
}

impl Template {
    pub fn equals(&self, piece: &Piece) -> bool {
        self.width == piece.width && self.height == piece.height
    }

    pub fn name(&self) -> String {
        format!("{}x{}x{}", self.length, self.width, self.height)
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("{}", self.name));
        //ui.text_edit_singleline(&mut self.name);
        self.name = self.name();
        ui.horizontal(|ui| {
            ui.add(DragValue::f64(&mut self.length));
            ui.add(DragValue::f64(&mut self.width));
            ui.add(DragValue::f64(&mut self.height));
        });
        ui.horizontal(|ui| {
            ui.label("Price");
            ui.add(DragValue::f32(&mut self.price));
        });
    }
}

/// A `Piece`: A single entity that is part of your design
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Piece {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

impl Piece {
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        //ui.label(format!("{} {} {} ", self.length, self.width, self.height));
        ui.horizontal(|ui| {
            ui.label("Size");
            ui.add(DragValue::f64(&mut self.length));
            ui.add(DragValue::f64(&mut self.width));
            ui.add(DragValue::f64(&mut self.height));

            //ui.add(Painter::circle_filled(center, radius, fill_color))

            let color = Color32::from_rgb(128, 128, 128);

            let mut paint_rect = ui.available_rect_before_wrap();
            paint_rect.max.y = paint_rect.min.y + self.width as f32;
            paint_rect.max.x = paint_rect.min.x + self.length as f32;

            ui.painter()
                .rect_stroke(paint_rect, 0., Stroke::new(2., color));
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    name: String,
    remaining_length: f64,
    cuts: Vec<f64>,
    pieces: Vec<Piece>,
    number: usize,
    price: f32,
}

impl Order {
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.name);
        ui.label(format!("Remaining length {}", self.remaining_length));
        ui.label(format!("Cuts {:?}", self.cuts));
        ui.label(format!("Number {}", self.number));
        ui.label(format!("Price {}", self.price));
        for p in &mut self.pieces {
            p.draw(ui);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Orders {
    pub items: Vec<Order>,
    pub sum: HashMap<String, i32>,
    pub price: f32,
}

impl Orders {
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        for i in &mut self.items {
            i.draw(ui);
        }
        ui.label(format!("Sum {:?}", self.sum));
        ui.label(format!("Price {}", self.price));
    }

    pub fn add(&mut self, piece: &Piece, template: &Template) {
        dbg!(&piece, &template);
        for mut order in &mut self.items {
            if order.name == template.name && piece.length <= order.remaining_length {
                order.cuts.push(piece.length);
                order.remaining_length -= piece.length;
                order.pieces.push(piece.clone());
                println!("Added segment {:?} to {:?}", piece, order);
                return;
            }
        }
        // add new order item
        let o = Order {
            name: template.name.clone(),
            remaining_length: template.length - piece.length,
            cuts: vec![piece.length],
            pieces: vec![piece.clone()],
            number: self.items.len(),
            price: template.price,
        };
        println!("Created new order item {:?}", o);

        self.items.push(o);
    }

    pub fn sum(&mut self) {
        for item in &self.items {
            dbg!(&item.name);
            let stat = self.sum.entry(item.name.clone()).or_insert(0);
            *stat += 1;
            self.price += item.price;
            // self.sum.insert(item.name.clone(), 1);
        }
    }
}
