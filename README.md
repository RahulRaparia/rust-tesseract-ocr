# rust-tesseract-ocr
This repository contains a simple Rust project that uses the rusty-tesseract library to recognize text from an image using Tesseract OCR. The project reads an image named img.jpeg from the src folder and outputs the recognized text to a text file named output.txt in the same folder.

## Install Rust From pwershell 

```
Invoke-WebRequest -Uri https://win.rustup.rs/ -OutFile rustup-init.exe

```


## Rust OCR Project
This is a simple Rust project that uses the rusty-tesseract library to recognize text from an image using Tesseract OCR.

## Getting Started
To get started with this project, you will need to have Rust installed on your system. You can install Rust by following the instructions on the official Rust website.

Once you have Rust installed, you can clone this repository and navigate to the project directory:

git clone https://github.com/your-username/rust-ocr-project.git
cd rust-ocr-project
Copy
Next, you will need to add the rusty-tesseract library to your Cargo.toml file:


[dependencies]
rusty-tesseract = "1.1.4"
Copy
You can then build and run the project using the cargo command:

cargo run
Copy
This will read an image named img.jpeg from the src folder and output the recognized text to a text file named output.txt in the same folder.

License
This project is licensed under the MIT License - see the LICENSE file for details.