// 定义一个 trait，表示可以计算面积的类型
trait Area {
    fn calculate_area(&self) -> f64;
}

// 定义圆形结构体
struct Circle {
    radius: f64,
}

// 为 Circle 实现 Area trait
impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义三角形结构体
struct Triangle {
    base: f64,
    height: f64,
}

// 为 Triangle 实现 Area trait
impl Area for Triangle {
    fn calculate_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 定义正方形结构体
struct Square {
    side_length: f64,
}

// 为 Square 实现 Area trait
impl Area for Square {
    fn calculate_area(&self) -> f64 {
        self.side_length * self.side_length
    }
}

// 定义打印面积的函数，接收实现了 Area trait 的类型
fn print_area<T: Area>(shape: T) {
    println!("The area is: {}", shape.calculate_area());
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_u32_collection() {
        let circle = Circle { radius: 5.0 };
        let triangle = Triangle { base: 4.0, height: 3.0 };
        let square = Square { side_length: 6.0 };

        // 打印不同图形的面积
        print_area(circle);
        print_area(triangle);
        print_area(square);
    }
}

