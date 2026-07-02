fn closure_print() {
    let name = String::from("Hello Rust");
    let print_name = || println!("{}", name);
    print_name();
}

fn closure_print_mut() {
    let mut count = 0;
    let mut increment = || { count += 1 };
    increment();    
    println!("{}", count);
}

fn closure_single_parameter() {
    let print_number = |x: i32| println!("{}", x);
    print_number(5);
}

fn closure_multiple_parameters() {
    let print_sum = |x: i32, y: i32| println!("{}", x + y);
    print_sum(5, 10);
}

fn main() {
    closure_print();   
    closure_print_mut();
    closure_single_parameter();
    closure_multiple_parameters();

    let square = |x| x * x;
    println!("{}", square(7));

    let multiply = |x,y| x * y;
    println!("{}", multiply(4,5));

    let add = |x,y| x + y;
    println!("{}", add(8,9));



}