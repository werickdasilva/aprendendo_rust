use std::rc::Rc;

pub fn usando_rc() {
    let a = ListRc {
        num: 10,
        list: Rc::new(Some(ListRc {
            num: 20,
            list: Rc::new(None),
        })),
    };
    let b = ListRc {
        num: 30,
        list: Rc::clone(&a.list),
    };

    println!("{:?}", b)
}

#[derive(Debug)]
struct ListRc {
    num: i32,
    list: Rc<Option<ListRc>>,
}
