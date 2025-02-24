mod examples {
    use cyclonedx_bom::models::bom::Bom;
    use cyclonedx_bom::validation::{Validate, ValidationResult};

    #[ignore]
    #[test]
    fn it_should_parse_all_of_the_valid_json_examples() {
        insta::with_settings!({
            snapshot_path => "examples/snapshots/1.4",
            prepend_module_to_snapshot => false,
        }, {
            insta::glob!("examples/1.4/valid*.cdx.json", |path| {
                let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read file: {path:?}"));
                let bom = Bom::parse_from_json_v1_4(file).unwrap_or_else(|_| panic!("Failed to parse the document as an BOM: {path:?}"));

                let validation_result = bom.validate();
                assert_eq!(
                    validation_result,
                    ValidationResult::Passed,
                    "{path:?} unexpectedly failed validation"
                );

                let mut output = Vec::new();
                bom.output_as_json_v1_4(&mut output)
                    .unwrap_or_else(|_| panic!("Failed to output the file: {path:?}"));
                let bom_output = String::from_utf8_lossy(&output).to_string();

                insta::assert_snapshot!(bom_output);
            });
        });
    }

    #[ignore]
    #[test]
    fn it_should_fail_all_of_the_invalid_json_examples() {
        insta::with_settings!({
            snapshot_path => "examples/snapshots/1.4",
            prepend_module_to_snapshot => false,
        }, {
            insta::glob!("examples/1.4/invalid*.json", |path| {
                let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read file: {path:?}"));
                if let Ok(bom) = Bom::parse_from_json_v1_4(file) {
                    let validation_result = bom.validate();
                    assert_ne!(
                        validation_result,
                        ValidationResult::Passed,
                        "{path:?} unexpectedly passed validation"
                    );
                }
            });
        });
    }
}
