use printpdf::*;
use std::fs::File;
use std::io::Write;
use std::io::{BufWriter, Read};

use bitcoin::Address;
use bitcoin::Network;
use bitcoin::PrivateKey;
// use bitcoin::address::Payload;
use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::PublicKey;
// use core::hash::Hash;
use crate::rand::rngs::OsRng;

// use reqwest;
// use serde_json::Value;
// Import the OsRng for generating a new private key

fn create_pdf_from_json(yes_or_no: &str) {
    if yes_or_no.to_lowercase() == "yes" {
        let mut file = match File::open("wallet_details.json") {
            Ok(file) => file,
            Err(e) => {
                println!("Unable to open the file: {}", e);
                return;
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            println!("Unable to read the file: {}", e);
            return;
        }

        // Parsing the JSON content to extract the keys and address
        let wallet_data: serde_json::Value = serde_json::from_str(&contents).unwrap();
        let public_key = wallet_data["public_key"].as_str().unwrap();
        let private_key = wallet_data["private_key"].as_str().unwrap();
        let address = wallet_data["address"].as_str().unwrap();

        // Formatting the content string
        let formatted_contents = format!(
            "Public Key: {}\nPrivate Key: {}\nAddress: {}",
            public_key, private_key, address
        );

        let (doc, page1, layer1) =
            PdfDocument::new("PDF_Document", Mm(247.0), Mm(210.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Load and use a font
        let font_file = match File::open("src/font/Roboto-Regular.ttf") {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening font file: {}. Please ensure the 'Roboto-Regular.ttf' font file is in the correct location.", e);
                return;
            }
        };

        let font = match doc.add_external_font(font_file) {
            Ok(font) => font,
            Err(e) => {
                println!("Error adding external font: {}", e);
                return;
            }
        };

        // Splitting the formatted_contents by line and printing each line separately
        let lines: Vec<&str> = formatted_contents.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            current_layer.use_text(line.to_string(), 12.0, Mm(10.0), Mm(200.0 - i as f32 * 15.0), &font);

        }

        let output_file = match File::create("wallet_details.pdf") {
            Ok(file) => file,
            Err(e) => {
                println!("Unable to create PDF file: {}", e);
                return;
            }
        };

        if let Err(e) = doc.save(&mut BufWriter::new(output_file)) {
            println!("Error saving PDF file: {}", e);
            return;
        }

        println!("PDF created successfully.");
    }
}


fn display_wallet_address() {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // Generate a new private key
    let (secret_key, _) = secp.generate_keypair(&mut rng);
    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    // Generate and display the corresponding public address
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    let p2pkh_address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("Bitcoin Wallet Address: {}", p2pkh_address);

    let wallet_details = format!(
        r#"{{
            "private_key": "{}",
            "public_key": "{}",
            "address": "{}"
        }}"#,
        private_key.to_wif(),
        public_key,
        p2pkh_address.to_string()
    );

    // Exporting the wallet details to a JSON file
    export_wallet_details(&wallet_details);
}

fn export_wallet_details(wallet_details: &str) {
    // Try to create a new file to store the wallet details
    let mut file = match File::create("wallet_details.json") {
        Ok(file) => file,
        Err(e) => {
            println!("Unable to create file: {}", e);
            return;
        }
    };

    // Try to write the wallet details to the file
    if let Err(e) = file.write_all(wallet_details.as_bytes()) {
        println!("Unable to write data: {}", e);
    } else {
        println!("Wallet details exported to wallet_details.json");
    }
}

fn main() {
    // tokio::spawn(async { send_bitcoin("DESTINATION_ADDRESS", AMOUNT).await });
    // tokio::spawn(async { check_balance("YOUR_ADDRESS").await });
    display_wallet_address();
    create_pdf_from_json("yes");
}

// const API_TOKEN: &str = "YOUR_BLOCKCYPHER_API_TOKEN";
// // Placeholder function for sending Bitcoin
// async fn send_bitcoin(to_address: &str, amount: u64) {
//     // Note: This is a simplified example. In a real-world scenario, you would need to create and sign a transaction.
//     let url = format!(
//         "https://api.blockcypher.com/v1/btc/main/txs/new?token={}",
//         API_TOKEN
//     );

//     let payload = json!({
//         "inputs": [], // You need to specify the inputs (source addresses and private keys)
//         "outputs": [
//             {
//                 "addresses": [to_address],
//                 "value": amount
//             }
//         ]
//     });

//     let client = reqwest::Client::new();
//     let res = client.post(&url).json(&payload).send().await;

//     match res {
//         Ok(response) => {
//             if response.status().is_success() {
//                 println!("Transaction sent successfully!");
//             } else {
//                 println!("Failed to send transaction: {:?}", response.text().await);
//             }
//         }
//         Err(e) => println!("Error: {:?}", e),
//     }
// }

// Placeholder function for checking balance
// async fn check_balance(address: &str) {
//     let url = format!(
//         "https://api.blockcypher.com/v1/btc/main/addrs/{}/balance",
//         address
//     );

//     let res = reqwest::get(&url).await;

//     match res {
//         Ok(response) => {
//             if response.status().is_success() {
//                 let data: Value = response.json().await.unwrap();
//                 let balance = data["balance"].as_u64().unwrap();
//                 println!("Balance for address {}: {} satoshis", address, balance);
//             } else {
//                 println!("Failed to check balance: {:?}", response.text().await);
//             }
//         }
//         Err(e) => println!("Error: {:?}", e),
//     }
// }
