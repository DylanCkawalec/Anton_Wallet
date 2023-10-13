


# AntonWallet - Bitcoin Wallet Generator and PDF Exporter

AntonWallet is a simple Rust-based command-line application that generates Bitcoin wallet details, including the public key, private key, and address. It provides an option to export these details to a JSON file and create a PDF document for easy storage and sharing.

## Features

- Generate a new Bitcoin wallet address.
- Export wallet details to a JSON file.
- Generate a PDF document containing wallet details for easy printing and sharing.

## Getting Started

### Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust and Cargo: Make sure you have Rust and Cargo installed on your system. You can download them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

### Installation

1. Clone the AntonWallet repository:

   ```bash
   git clone https://github.com/yourusername/antonwallet.git
   ```

2. Change to the project directory:

   ```bash
   cd antonwallet
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

### Usage

To use AntonWallet, follow these steps:

1. Generate and display a new Bitcoin wallet address:

   ```bash
   cargo run
   ```

   AntonWallet will generate a new wallet address and display it on the console.

2. Export Wallet Details:

   After generating the wallet address, AntonWallet will give you the option to export wallet details to a JSON file. Choose option 2 to export the details.

3. Create a PDF (Optional):

   If you want to create a PDF document containing wallet details, choose 'yes' when prompted. AntonWallet will create a PDF file named `wallet_details.pdf` in the project directory.

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used.
- [Bitcoin Library for Rust](https://github.com/rust-bitcoin/rust-bitcoin) - Used for Bitcoin wallet generation.
- [PrintPDF](https://crates.io/crates/printpdf) - Used for PDF generation.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the need to provide a simple tool for generating and exporting Bitcoin wallet details.
- Thanks to Anton for the inspiration and testing!

