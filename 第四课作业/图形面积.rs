// 定义一个可以计算面积的trait
pub trait HasArea {
    fn area(&self) -> f64;
}

// 为圆形实现这个trait
pub struct Circle {
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 为正方形实现这个trait
pub struct Square {
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 定义一个打印面积的函数，它接受一个实现了HasArea trait的类型
pub fn print_area<T: HasArea>(shape: &T) {
    println!("The area is: {}", shape.area());
}

// 在main函数中测试
fn main() {
    let circle = Circle { radius: 5.0 };
    print_area(&circle);

    let square = Square { side: 4.0 };
    print_area(&square);
}
