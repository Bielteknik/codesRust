/*fn closure_print() {
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

fn closure_ret_iter() {
    let numbers = vec![1,2,3,4,5,5];
    let result: Vec<i32> = numbers.into_iter()
        .filter(|x| x % 2 == 2)
        .map(|x| x * 2)
        .collect();
    println!("{:?}", result);
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
    closure_ret_iter();
}
*/

fn main(){
    //Closure with single parameter
    let get_square = |n| n * n;
    println!("{}", get_square(4));

    //Closure with multiple parameters
    let add = |x,y| x + y;
    println!("{}", add(5,6));

    //closure to get length of a string slice
    let length = |s: &str| s.len();
    println!("{}", length("Hello Rust"));
}