pub fn main() {
    let mut mul_3: Vec<i32> = Vec::new();
    let mut mul_5: Vec<i32> = Vec::new();
    
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            mul_3.push(i);
        }
        if i % 5 == 0 {
            mul_5.push(i)
        }   
    }

    let sum_3: i32 = mul_3.iter().sum();
    let sum_5: i32 = mul_5.iter().sum();
    println!("Sum of multiples of 3 or 5 below 1000: {}", sum_3 + sum_5);
}