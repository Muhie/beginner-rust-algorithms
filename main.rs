use std::mem;
fn main(){
    println!("bubble sort algorithm in rust!");
    let mut my_array: [i32; 5] = [5,2,45,4,16];
    println!("unsorted array is {:?}",my_array);
    let mut swapped = 1;
    let mut temp = 0;
    while swapped == 1{
        swapped = 0;
        for i in 0..my_array.len()-1{
            if my_array[i] < my_array[i+1]{
                temp = my_array[i+1];
                my_array[i+1] = my_array[i];
                my_array[i] = temp;
                swapped = 1;
            }
        }
    }
    println!("sorted array is {:?}",my_array)
}
