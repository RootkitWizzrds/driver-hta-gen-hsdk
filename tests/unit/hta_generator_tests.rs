use hta_gen::hta_generator::HtaGenerator;

#[test]
fn test_encode_driver() {
    let generator = HtaGenerator::new("output.hta", "driver.bin");
    let data = b"test driver data";
    let encoded = generator.encode_driver(data);

    // Base64 encoded version of "test driver data"
    let expected_encoded = "dGVzdCBkcml2ZXIgZGF0YQ==";
    assert_eq!(encoded, expected_encoded);
}
