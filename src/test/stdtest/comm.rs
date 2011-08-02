use std;
import std::comm;

#[test]
fn create_port_and_chan() {
    let p = comm::mk_port[int]();
    let c = p.mk_chan();
}

#[test]
fn send_recv() {
    let p = comm::mk_port();
    let c = p.mk_chan();

    c.send(42);
    let v = 0;
    p.recv_into(v);

    assert(42 == v);
}