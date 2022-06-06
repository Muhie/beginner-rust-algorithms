use std::mem;
fn main(){
    let mut my_array: [i32; 7] = [10,3,74,2,88,89,12];
    println!("unsorted array is {:?}",my_array);
    let i = 0;
    let mut k = 0;
    for i in i..my_array.len(){
        let current_item = my_array[i];
        k = i;
        while k > 0 && my_array[k-1] > current_item{
            my_array[k] = my_array[k-1];
            k = k - 1;
        }
        my_array[k]=current_item;
    }
    println!("final array is {:?}",my_array); 
}
