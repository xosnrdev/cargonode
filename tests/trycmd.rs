#[test]
#[ignore = "trycmd is not supported on cross-compilation"]
fn trycmd() {
    trycmd::TestCases::new().case("tests/cmd/*.md");
}
