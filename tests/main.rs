use std::path::PathBuf;
// FIXME gcc error while using dir2 with error_chain
#[test]
fn path_test() {
    let mut dir = PathBuf::from("C:\\hello");
    dir.push(".xx/yy.zz");
    assert_eq!(dir.to_str(), Some(r"C:\hello\.xx\yy.zz"))
}
