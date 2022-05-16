#[test]
#[cfg(feature = "v2")]
fn test_template_name_with_regex() {
    use paperclip_core::v2::models::{DefaultParameterRaw, Either, ParameterIn};

    let mut op: paperclip_core::v2::models::DefaultOperationRaw = Default::default();
    op.parameters = vec![Either::Right(DefaultParameterRaw {
        in_: ParameterIn::Path,
        ..Default::default()
    })];
    op.set_parameter_names_from_path_template("/test/{path:.*}");
    assert_eq!("path", op.parameters.first().unwrap().right().unwrap().name);
}
