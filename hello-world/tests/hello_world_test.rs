use gtest::{Log, Program, System};

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();

    let program = Program::current(&sys);
    program.send(2, String::from("INIT MESSAGE"));

    let res = program.send(2, String::from("Hello"));

    assert!(!res.log().is_empty());
    assert!(!res.main_failed());

    let expected_log = Log::builder()
        .dest(2)
        .payload(String::from("Hello"));

    assert!(res.contains(&expected_log));
}
