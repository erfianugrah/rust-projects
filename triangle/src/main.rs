fn main() {
    let x = [0, 3, 6];
    let y = [0, 3, 0];
    let area = calculate_triangle_area(&x, &y);
    println!("Area of triangle is {}", area);
}

fn calculate_triangle_area(x: &[u32], y: &[u32]) -> u32 {
    let area = x[0] * y[1] + x[1] * y[2] + x[2] * y[0] - x[1] * y[0] - x[2] * y[1] - x[0] * y[2];
    area / 2
}
