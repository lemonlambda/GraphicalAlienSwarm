use image::io::Reader as ImageReader;
use seq_macro::seq;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Names of File
    let textures = seq!(N in 1..=15 {
        vec![
            "Air",
            "Dirt",
            #(concat!("Rock", N),)*
        ]
    });

    // Write Pixels for PNG
    let mut bytes = vec![];
    for x in textures.clone() {
        println!("{x}.png");
        let img = ImageReader::open(format!("assets/{x}.png"))?.decode()?;
        let mut temp_bytes = img.into_bytes();
        bytes.append(&mut temp_bytes);
    }

    // Write PNG
    let path = Path::new(r"assets/tiles.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 32, 32 * textures.len() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&bytes)?;

    Ok(())
}
