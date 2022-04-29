use fast_version::const_version;

#[test]
fn const_version_test() {
    const VERSION: fast_version::Version = const_version!("0.1.2");
    assert_eq!(VERSION.major, 0);
    assert_eq!(VERSION.minor, 1);
    assert_eq!(VERSION.patch, 2);
}
