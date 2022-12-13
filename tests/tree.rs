use rust_triangle_star::generate_tree;

#[test]
fn verify_tree() {
    let expected = r#"    *
   ***
  *****
 *******
*********"#;
    let generated = generate_tree(5, 0, &[]);
    assert!(generated.is_ok());
    let generated = generated.unwrap();
    assert_eq!(generated.lines().count(), 5);
    assert_eq!(&generated, expected);
}
