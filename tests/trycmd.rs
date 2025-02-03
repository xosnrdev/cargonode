#[test]
#[cfg(unix)]
fn trycmd() {
    trycmd::TestCases::new().case("tests/cmd/*.md");
}
