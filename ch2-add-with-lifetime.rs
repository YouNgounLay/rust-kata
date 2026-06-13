
fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j // Adds the values referred to by i and j rather than adding the references directly
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetime(&a, &b);
    
    println!("{}", res);
}

