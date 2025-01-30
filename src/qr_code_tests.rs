#[cfg(test)]
mod tests {
    use super::super::qr_code::generate_qr_code;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_generate_qr_code_success() {
        let input = "Hello there";
        let result = generate_qr_code(input, ".");
        assert!(result.is_ok());
        let filename = result.unwrap();
        let expected_filename = PathBuf::from(".").join("4e47826698bb4630fb4451010062fadbf85d61427cbdfaed7ad0f23f239bed89.png");
        assert_eq!(Path::new(&filename), expected_filename.as_path());
        assert!(Path::new(&filename).exists());
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_generate_qr_code_empty_input() {
        let input = "";
        let result = generate_qr_code(input, ".");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_qr_code_invalid_input() {
        let input = "\0";
        let result = generate_qr_code(input, ".");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_qr_code_file_creation_error() {
        // Simulate file creation error by providing an invalid directory
        let input = "Hello there";
        let result = generate_qr_code(input, "/invalid_path");
        assert!(result.is_err());
    }
}