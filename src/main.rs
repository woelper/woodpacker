#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::io::BufWriter;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Template {
    length: f64,
    width: f64,
    height: f64,
    name: String,
}

impl Template {
    fn equals(&self, piece: &Piece) -> bool{
        self.width == piece.width && self.height == piece.height
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Piece {
    length: f64,
    width: f64,
    height: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Order{
    name: String,
    remaining_length: f64,
    cuts: Vec<f64>,
    Number: usize
}

#[derive(Serialize, Deserialize, Debug)]
struct Orders {
    Items: Vec<Order>
}


impl Orders {
    fn add(&mut self, piece: &Piece, template: &Template) {

        for mut order in &mut self.Items {
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
            Number: self.Items.len()
        };
        // println!("Created new order {:?}", o);

        self.Items.push(o);

    }
}


fn main() {

    let reader = BufReader::new(File::open("templates.json").unwrap());
    let templates: Vec<Template> = serde_json::from_reader(reader).unwrap();

    let reader = BufReader::new(File::open("pieces.json").unwrap());
    let pieces: Vec<Piece> = serde_json::from_reader(reader).unwrap();

    let mut orders = Orders {
        Items: vec![]
    };



    for piece in pieces {
        for template in &templates {
            if template.equals(&piece) {
                // dbg!(&piece);
                // println!("{:?} == {:?}", template, piece);
                orders.add(&piece, &template);
            }
        }

    }
    
    for val in &orders.Items {
        println!("{}\tRestlaenge: {}\tSchnitte: {:?}",val.name, val.remaining_length, val.cuts);
    }

    let writer = BufWriter::new(File::create("order.json").unwrap());
    serde_json::to_writer_pretty(writer, &orders).unwrap();


}
