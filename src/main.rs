use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Image {
    layers: Vec<Layer>,
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Layer {
    pixels: Vec<u32>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            layers: Vec::new(),
            width: width,
            height: height,
        }
    }

    fn create_layer(&mut self, numbers: String) {
        self.layers.push(Layer {
            pixels: numbers
                .chars()
                .map(|n| n.to_string().parse::<u32>().unwrap())
                .collect(),
        });
    }

    fn layer_least_zeroes(&self) -> u32 {
        let mut num_of_zeroes = core::u32::MAX;
        let mut layer_num = 0;

        for (i, layer) in self.layers.iter().enumerate() {
            let new = layer.pixels.iter().filter(|n| **n == 0).count();
            if (new as u32) < num_of_zeroes {
                num_of_zeroes = new as u32;
                layer_num = i;
            }
        }

        layer_num as u32
    }

    fn find_layer_sum(&self, layer: u32) -> u32 {
        (self.layers[layer as usize]
            .pixels
            .iter()
            .filter(|n| **n == 1)
            .count()
            * self.layers[layer as usize]
                .pixels
                .iter()
                .filter(|n| **n == 2)
                .count()) as u32
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;
    let numbers = std::str::from_utf8(&buf).unwrap();

    //println!("{:?}", numbers);

    let mut image = Image::new(25, 6);

    let mut z = numbers.chars().peekable();
    while z.peek().is_some() {
        let chunk: String = z
            .by_ref()
            .take((image.width * image.height) as usize)
            .collect();
        image.create_layer(chunk);
    }

    println!("{:?}", image.find_layer_sum(image.layer_least_zeroes()));

    Ok(())
}
