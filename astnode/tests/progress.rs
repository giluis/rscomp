#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/00-compiles.rs");
    t.pass("tests/01-simple_leaf.rs");
    t.pass("tests/02-linkedtypes.rs");
    // t.pass("tests/03-enumtype.rs");
    // t.pass("tests/04-call-build.rs");
    // t.pass("tests/05-method-chaining.rs");
    // t.pass("tests/06-optional-field.rs");
    // t.pass("tests/07-repeated-field.rs");
    // t.compile_fail("tests/08-unrecognized-attribute.rs");
    //t.pass("tests/09-redefined-prelude-types.rs");
}
