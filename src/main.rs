use crate::colour::Colour;

mod colour;
mod point;
mod ray;
mod vector;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("Scanlines remaining: {remaining}");

        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0f64;

            let colour = Colour::new(r, g, b);

            println!("{colour}");
        }
    }
}
