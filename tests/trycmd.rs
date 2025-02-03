#[test]
#[cfg(target_arch = "aarch64")]
fn trycmd() {
    trycmd::TestCases::new().case("tests/cmd/*.md");
}
