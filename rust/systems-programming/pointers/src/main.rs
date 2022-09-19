use std::{cell::RefCell, rc::Rc, sync::Arc};

fn main() {
    // refcell();
    // kkk();
    let p1 = 6;

    let p = &p1 as *const i32 as *mut i32;

    let x = &p1;
    println!("{p:p}");;
    println!("{x:p}");;
    println!("{p:?}");;

    // let k = refcell();
    println!("{:p}", &refcell());;
}

fn refcell() {
    let p = Rc::new(RefCell::new("rer".to_owned()));
    let p2 = p.clone();

    p2.borrow_mut().push_str("rer");
    p2.borrow_mut().push_str("rerxxxx");

    let p3 = Rc::clone(&p);
    p3.borrow_mut().pop();

    dbg!(p2.clone());

    dbg!(p);
    dbg!(p2);
}

fn kkk() {
    use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, sync::Arc};
    let m = Arc::new(::parking_lot::Mutex::new(5.to_string()));

    let m2 = m.clone();

    m2.lock().borrow_mut().push_str("rerer");

    m.clone().lock().borrow_mut().push_str("vv");

    dbg!(m);
    dbg!(m2);
}
