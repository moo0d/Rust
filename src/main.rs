mod qr_code;
#[cfg(test)]
mod qr_code_tests;

#[cfg(not(tarpaulin))]
fn main() {
    let input = std::env::args().nth(1).expect("Please provide an input string");
    qr_code::generate_qr_code(&input, ".").expect("Failed to generate QR code");
}