use xid::{self as kazk_xid};
mod xid_ref;

#[test]
fn test_xid_parse() {
    for _ in 0..10 {
        println!("{}", kazk_xid::new().to_string());
    }
    let gen = xid_ref::new_generator();
    for _ in 0..10 {
        println!("{}", gen.new_id().unwrap().encode());
    }

    for _ in 0..10 {
        let id = xid_ref::ID::decode(&kazk_xid::new().to_string());
        println!(
            "encoded: {:?}    machine: {:?}    counter: {:?}    time: {:?}",
            id.encode(),
            id.machine(),
            id.counter(),
            id.time()
        );
    }
    for _ in 0..10 {
        let id = gen.new_id().unwrap();
        println!(
            "encoded: {:?}    machine: {:?}    counter: {:?}    time: {:?}",
            id.encode(),
            id.machine(),
            id.counter(),
            id.time()
        );
    }
}
