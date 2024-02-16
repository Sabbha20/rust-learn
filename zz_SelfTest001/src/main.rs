
// Will print the largest number from a list
// will get 3 largest number
fn main() {
    let my_lst = vec![5, 2, 45, 21, 98, 112, 52, 5, 6, 149];

    let largest_num = get_largest_number(my_lst.clone());

    let largest_three = get_three_largest(my_lst.clone());

    println!("Largest Value is {:?}", largest_num);
    println!("Largest 3 values is {:?}", largest_three);
}


fn get_largest_number(some_lst: Vec<i32>) -> i32{
    let mut max_val = 0;

    for itm in some_lst{
        if itm >= max_val{
            max_val = itm;
        }
    }

    return max_val
}

fn get_three_largest(mut my_vec:Vec<i32>) -> Vec<i32> {
    my_vec.sort();
    return vec![my_vec[my_vec.len()-1], my_vec[my_vec.len()-2], my_vec[my_vec.len()-3]]

}