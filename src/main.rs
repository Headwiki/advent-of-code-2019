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
    pixels: Vec<Row>,
}

#[derive(Debug, Clone)]
struct Row {
    row: Vec<u32>,
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
        let mut new_layer = Layer { pixels: Vec::new() };

        let mut z = numbers.chars().peekable();
        while z.peek().is_some() {
            let chunk: String = z.by_ref().take(self.width as usize).collect();
            new_layer.pixels.push(Row {
                row: chunk
                    .chars()
                    .map(|n| n.to_string().parse::<u32>().unwrap())
                    .collect(),
            });
        }
        self.layers.push(new_layer);
    }

    fn print_image(&self) -> Layer {
        let mut new_layer = create_empty_layer();

        for layer in self.layers.iter() {
            for (i, row) in layer.pixels.iter().enumerate() {
                for (k, pixel) in row.row.iter().enumerate() {
                    if (new_layer.pixels[i].row[k] == 0) || (new_layer.pixels[i].row[k] == 1) {
                    } else {
                        if *pixel < new_layer.pixels[i].row[k] {
                            new_layer.pixels[i].row[k] = *pixel
                        }
                    }
                }
            }
        }

        new_layer
    }
}

fn create_empty_layer() -> Layer {
    let mut new_layer = Layer { pixels: Vec::new() };
    let new_row = Row {
        row: vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ],
    };

    for x in 0..6 {
        new_layer.pixels.push(new_row.clone());
    }

    new_layer
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

    println!("{:?}", image.print_image());

    Ok(())
}
