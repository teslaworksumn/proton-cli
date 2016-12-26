extern crate proton_cli;


#[test]
fn proper_id_returned_with_existing_name() {
    let expected = 1;
    let actual = 2; // TODO: implement
    assert_eq!(expected, actual);
}

#[test]
#[should_panic(expected = "")]
fn fails_if_name_invalid() {
}

#[test]
#[should_panic(expected = "")]
fn fails_if_name_not_found() {
}
