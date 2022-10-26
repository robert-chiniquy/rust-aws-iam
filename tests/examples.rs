#![cfg(test)]
extern crate test_generator;

use aws_iam::io;
use aws_iam::model::{Principal, Statement};
use std::fs;
use std::path::PathBuf;
use test_generator::test_resources;

//fn init() {
//    let _ = env_logger::builder().is_test(true).try_init();
//}

#[test_resources("tests/data/good/*.json")]
fn verify_good_examples(resource: &str) {
    println!("verify_good_examples reading file {}", resource);
    let file_name = PathBuf::from(resource);
    let result = io::read_from_file(&file_name);
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[test_resources("tests/data/bad/*.json")]
fn verify_bad_examples(resource: &str) {
    println!("verify_bad_examples reading file {}", resource);
    let file_name = PathBuf::from(resource);
    let result = io::read_from_file(&file_name);
    println!("{:#?}", result);
    assert!(result.is_err());

    let expected_error = read_expected_error(&file_name.clone().with_extension("txt"));
    assert_eq!(format!("{:?}", result.err().unwrap()), expected_error);
}

fn read_expected_error(file_name: &PathBuf) -> String {
    match fs::read_to_string(file_name) {
        Ok(s) => s,
        Err(e) => panic!(
            "Could not read expected error from file {:?}, error: {:?}",
            &file_name, e
        ),
    }
}

// Assert that a resource-based policy with `"Principal": "*"` is correctly parsed
// https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html
#[test]
fn test_principal_star() {
    let policy =
        std::fs::read_to_string("tests/data/good/example-101.json").expect("example-101.json");
    let policy: aws_iam::model::Policy = serde_json::from_str(&policy).expect("policy parses");
    if let aws_iam::model::OneOrAll::One(Statement {
        principal: Some(Principal::Principal(p)),
        ..
    }) = policy.statement
    {
        assert_eq!(p.len(), 1);
    } else {
        println!("this test is currently expected to fail: {policy:#?}");
        panic!(".")
    }
}
