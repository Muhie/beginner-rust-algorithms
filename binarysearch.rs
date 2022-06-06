use std::io::stdin;
use std::io;
fn main(){
    println!("Enter an integer value:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("failed to read input.");
    let user_input: i32 = user_input.trim().parse().expect("invalid input");
    binary_search(user_input);
}
fn binary_search(mut search_val: i32){
    let mut my_array: [i32; 7] = [10,3,74,2,88,89,12];
    println!("the value to be searched is: {}",search_val);
    let mut lower_bound = 0;
    let mut upperbound = my_array.len();
    let mut mid_point = 0;
    while my_array[mid_point] != search_val{
        if lower_bound > upperbound{
            println!("{}",upperbound);
            println!("The entered value is not in the array {:?}", my_array);
            break;
        }
        else{
            mid_point = lower_bound + (upperbound - lower_bound)/2
        }
        if my_array[mid_point] == search_val{
            println!("The entered value was found in the array at position {}",mid_point)
        }
        else{
            if search_val < my_array[mid_point]{
                upperbound = mid_point-1;
            }
            else{
                lower_bound = mid_point + 1
            }
        }

    }
}
