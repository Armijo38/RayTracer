use ray_tracer::{process,Config};
use std::fs;
use image;


fn do_test(test_prefix: &str) {
    let config_file_raw: String = fs::read_to_string("tests/configs/".to_string() + test_prefix + ".cfg").expect("Should have been able to read config file");
    let config: Config = serde_json::from_str(&config_file_raw).expect("Should have been able to parse config file");

    let print_debug_objects = false;
    let img = process(config, print_debug_objects);

    let orig_img = image::open("tests/imgs/".to_string() + test_prefix + ".png").unwrap().to_rgb8();
    assert!(img == orig_img);
}

#[test]
fn test_one() {
    do_test("test1");
}

#[test]
fn test_two() {
    do_test("test2");
}

#[test]
fn test_three() {
    do_test("test3");
}

#[test]
fn test_four() {
    do_test("test4");
}

#[test]
fn test_five() {
    do_test("test5");
}
