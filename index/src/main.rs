use if_else_if_else::{find_loop, infinity_loop, learning, percent_count};

mod if_else_if_else;

fn main() {
    //?repeat rust basics
    let x: i32 = 13; // тип данных i32
    println!("{}", x);

    let y: f32 = 3.14159;
    println!("{}", y);

    fn float_num(num: f32) {
        let new_num: f32 = num + 4.56;
        println!("{}", new_num);
    }
    float_num(y);

    let mut z: i32; // mutable переменная
    z = 0;
    println!("{}", z);
    z = 1;
    println!("{}", z);

    let a: u8 = 3;
    let b: i32 = 2 + a as i32;
    println!("{}", b);

    let array_nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:#?}", array_nums);

    fn add(x: i32, y: i32) -> i32 {
        println!("{}", x + y);
        return x + y;
    }
    add(20, 40);

    fn make_nothing() -> () {
        return ();
    }

    make_nothing();
    let a = make_nothing();
    println!("The value of a: {:?}.", a);

    fn make_tuple(a: i32, b: i32) -> (i32, i32) {
        return (a, b);
    }

    let tuple: (i32, i32) = make_tuple(123, 456);
    println!(
        "First value of tuple: {}. Second valeu of tuple: {}",
        tuple.0, tuple.1
    );

    learning();
    infinity_loop();
    find_loop();
    percent_count();
}
