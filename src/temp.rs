// fn main() {
//     // let graph = vec![(0, 1, 1), (0, 2, 3), (1, 3, 2), (2, 3, 1), (3, 4, 4)];
//     // let start = 0;
//     // let end = 4;

//     let input = "[(0, 1, 1), (0, 2, 3), (1, 3, 2), (2, 3, 1), (3, 4, 4)], 0, 4";

//     let start_index = input.find('[').expect("Square brackets not found");
//     let end_index = input.rfind(']').expect("Square brackets not found");

//     let data = &input[start_index..=end_index];

//     let start = input[end_index+3..=end_index+3].parse::<usize>().unwrap();
//     let end = input[end_index+6..=end_index+6].parse::<usize>().unwrap();

//     println!("data: {}", data);

//     println!("start: {}", start);
//     println!("end: {}", end);

//     println!("{:?}", find_bottlenecks(data, start, end));

// }

// fn main() {
//     let input = "[(0, 1, 1), (1, 2, 1), (2, 3, 1)], 0, 3";

//     // Find the start and end indices of square brackets
//     let start_index = input.find('[').expect("Square brackets not found");
//     let end_index = input.rfind(']').expect("Square brackets not found");

//     // Extract the data inside the square brackets
//     let data = &input[start_index..=end_index];

//     let start = &input[end_index+3..=end_index+3];
//     let data = &input[end_index+6..=end_index+6];

//     println!("{}", data);
// }

// use std::io;

// fn main() {
//     // Read input from the user
//     println!("Enter tuples in the form [(a, b, c), (d, e, f), ...]:");
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     let start_index = input.find('[').expect("Square brackets not found");
//     let end_index = input.rfind(']').expect("Square brackets not found");

//     let data_str = &input[start_index..=end_index];

//     let start = &input[end_index+3..=end_index+3];
//     let end = &input[end_index+6..=end_index+6];

//     // Extract tuples from input
//     println!("data_str: {}",  data_str);
//     println!("start: {}",  start);
//     println!("end: {}",  end);


//     let mut data: Vec<(i32, i32, i32)> = Vec::new();

//     let mut buffer = String::new();
//     let mut inside_brackets = false;

//     for c in data_str.chars() {
//         match c {
//             '[' => inside_brackets = true,
//             ']' => inside_brackets = false,
//             '(' => buffer.clear(),
//             ')' => {
//                 let tuple: Vec<i32> = buffer
//                     .split(',')
//                     .map(|s| s.trim().parse().unwrap())
//                     .collect();
//                 data.push((tuple[0], tuple[1], tuple[2]));
//             }
//             _ => {
//                 if inside_brackets {
//                     buffer.push(c);
//                 }
//             }
//         }
//     }

//     println!("{:?}", data);
// }



// fn main() {
//     let data_str = "[(0, 1, 1), (0, 2, 3), (1, 3, 2), (2, 3, 1), (3, 4, 4)]";
//     let mut data: Vec<(i32, i32, i32)> = Vec::new();

//     let mut buffer = String::new();
//     let mut inside_brackets = false;

//     for c in data_str.chars() {
//         match c {
//             '[' => inside_brackets = true,
//             ']' => inside_brackets = false,
//             '(' => buffer.clear(),
//             ')' => {
//                 let tuple: Vec<i32> = buffer
//                     .split(',')
//                     .map(|s| s.trim().parse().unwrap())
//                     .collect();
//                 data.push((tuple[0], tuple[1], tuple[2]));
//             }
//             _ => {
//                 if inside_brackets {
//                     buffer.push(c);
//                 }
//             }
//         }
//     }

//     println!("{:?}", data);
// }