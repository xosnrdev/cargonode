use std::borrow::Cow;

// Generated at build time by build.rs script
include!(concat!(env!("OUT_DIR"), "/embedding.rs"));

pub fn load_template() -> Cow<'static, [u8]> {
    Cow::Borrowed(EMBEDDED_TEMPLATE)
}

#[test]
fn test_load_template() {
    let template = load_template();
    assert_eq!(template.len(), 364);
}
