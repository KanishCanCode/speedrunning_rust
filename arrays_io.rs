use std::io;

fn main() {
    // Define the size of the array
    println!("Enter the size of the array:");
    let mut size = String::new();
    io::stdin().read_line(&mut size)
        .expect("Failed to read line");

    let size: usize = size.trim().parse()
        .expect("Please enter a valid number");

    // Create an array to store user input
    let mut arr = vec![0; size];

    // Prompt the user to enter data for the array
    println!("Enter {} elements for the array:", size);
    for i in 0..size {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        let num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        arr[i] = num;
    }

    // Display the array data entered by the user
    println!("Array elements:");
    for num in arr.iter() {
        println!("{}", num);
      }
    let num : &[i32] = &arr[0..size];
    println!("{:?}", num);
    
}
