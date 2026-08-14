// use std::io::{self, Write};

/// Calculates the signed shortest dial spin from `from` digit to `to` digit.
/// Returns a positive number for clockwise spins and negative for counter-clockwise spins.
pub fn shortest_spin(from: u32, to: u32) -> i32 {
    let mut diff = to as i32 - from as i32;
    if diff > 5 {
        diff -= 10;
    } else if diff < -5 {
        diff += 10;
    }
    diff
}

/// Calculates total circular rotations required to align all lock digits to target `k`.
fn total_dial_distance(digits: &[u32], target: u32) -> u32 {
    digits
        .iter()
        .map(|&d| shortest_spin(d, target).unsigned_abs())
        .sum()
}

/// Algorithm 1: Naive Arithmetic Average
// fn naive_avg(digits: &[u32]) -> String {
//     let sum: u32 = digits.iter().sum();
//     let avg = sum / digits.len() as u32;
//     avg.to_string().repeat(digits.len())
// }

/// Algorithm 2: Original Pairwise Distance Approach
// fn original_distance(digits: &[u32]) -> String {
//     let n = digits.len();
//     let mut result_digits = Vec::with_capacity(n);

//     for i in 0..n {
//         let curr = digits[i] as i32;
//         let mut avg_dist: i32 = 0;

//         for j in 0..n {
//             if i == j {
//                 continue;
//             }
//             let other = digits[j] as i32;
//             let d1 = other - curr;
//             let d2 = (other + 10) - curr;
//             let d3 = other - (curr + 10);

//             if d1.abs() <= 5 {
//                 avg_dist += d1;
//             } else if d2.abs() <= 5 {
//                 avg_dist += d2;
//             } else if d3.abs() <= 5 {
//                 avg_dist += d3;
//             }
//         }

//         avg_dist /= (n - 1) as i32;
//         let mut res_digit = avg_dist + curr;
//         if res_digit < 0 {
//             res_digit += 10;
//         } else if res_digit > 9 {
//             res_digit -= 10;
//         }

//         result_digits.push(res_digit as u32);
//     }

//     naive_avg(&result_digits)
// }

/// Algorithm 3: Globally Optimal Minimum Circular Distance O(N)
pub fn optimal_distance(digits: &[u32]) -> (u32, u32) {
    (0..=9)
        .map(|k| (k, total_dial_distance(digits, k)))
        .min_by_key(|&(_, cost)| cost)
        .expect("Combination cannot be empty")
}

// fn main() {
//     println!("Input your combination:");
//     print!("==> ");
//     io::stdout().flush().unwrap();

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     let combo = input.trim();

//     if combo.is_empty() {
//         println!("Error: Combination cannot be empty.");
//         return;
//     }

//     // Convert string inputs safely into a Vec<u32>
//     let digits: Vec<u32> = match combo.chars().map(|c| c.to_digit(10)).collect() {
//         Some(vec) => vec,
//         None => {
//             println!("Error: Your combination must contain only integer digits (0-9).");
//             return;
//         }
//     };

//     if digits.len() == 1 {
//         println!("Your single-digit combination is \"{}\"", combo);
//         return;
//     }

//     println!("\n================ RESULTS COMPARISON ================");

//     // 1. Naive Average
//     let naive_res = naive_avg(&digits);
//     let naive_target = naive_res.chars().next().unwrap().to_digit(10).unwrap();
//     let naive_cost = total_dial_distance(&digits, naive_target);
//     println!(
//         "1. Naive Average       : \"{}\"  (Total rotations: {})",
//         naive_res, naive_cost
//     );

//     // 2. Original Distance
//     let orig_res = original_distance(&digits);
//     let orig_target = orig_res.chars().next().unwrap().to_digit(10).unwrap();
//     let orig_cost = total_dial_distance(&digits, orig_target);
//     println!(
//         "2. Original Distance   : \"{}\"  (Total rotations: {})",
//         orig_res, orig_cost
//     );

//     // 3. Optimal Search
//     let (opt_digit, opt_cost) = optimal_distance(&digits);
//     let opt_res = opt_digit.to_string().repeat(digits.len());
//     println!(
//         "3. Optimal Min-Distance: \"{}\"  (Total rotations: {})",
//         opt_res, opt_cost
//     );
//     println!("====================================================");

//     // Rotation breakdown table for Optimal Min-Distance
//     println!("=====================");
//     println!("Original | Spin | New");
//     for &digit in &digits {
//         let spin = shortest_spin(digit, opt_digit);
//         println!("{:<8} | {:<4} | {}", digit, spin, opt_digit);
//     }
//     println!("=====================");
// }
