use ufbx;

#[test]
fn thread_safe() {
    assert!(ufbx::is_thread_safe());
}
