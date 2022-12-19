use std::{ops::Deref, rc::Rc};

fn main() {
    // usando rc diz que emquanto alguem estiver usando seu valor ele não será destruido
    let count_one = Rc::new(Number::Count(
        1,
        Rc::new(Number::Count(2, Rc::new(Number::Void))),
    ));
    let count_two = Number::Count(11, Rc::clone(&count_one));
    let count_three = Number::Count(12, Rc::clone(&count_one));

    println!("Count Two {count_two:#?}");
    println!("Count Three {count_three:#?}");

    let number = Rc::new(vec![1, 2, 3, 4, 5, 20]);
    let number_x_2 = x_2(Rc::clone(&number));
    let number_x_2_replay = x_2(Rc::clone(&number));

    println!("{number_x_2:?}");
    println!("{number_x_2_replay:?}");
}

#[derive(Debug)]
pub enum Number {
    Count(i32, Rc<Number>),
    Void,
}

pub fn x_2(vec: Rc<Vec<i32>>) -> Vec<i32> {
    let new_vec = vec.iter().map(|i| i * 2);
    new_vec.collect::<Vec<i32>>()
}

pub fn life_more(life: Rc<Vec<i32>>) {
    for item in life.iter() {
        print!("{item} ")
    }
}
