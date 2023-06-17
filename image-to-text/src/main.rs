use std::fs;
use std::path::Path;
use rusty_tesseract::{LepTess, Image};

fn main() {
    // Create an Image object from the image file
    let img = Image::from_path(Path::new("src/img.jpeg")).unwrap();

    // Create a new LepTess instance with the English language
    let mut lt = LepTess::new(None, "eng").unwrap();

    // Set the image to be used for OCR
    lt.set_image_from_image(&img);

    // Get the recognized text
    let text = lt.get_utf8_text().unwrap();

    // Write the recognized text to a file
    fs::write("src/output.txt", text).expect("Unable to write file");
}
