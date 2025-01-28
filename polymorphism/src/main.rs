use std::any::Any;

trait Shape: Any {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn as_any(&self) -> &dyn Any;
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

struct Circle {
    radius: f64,
}
impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
fn main() {
    let c1 = Circle::new(1.0);
    let r1 = Rectangle::new(1.0, 2.0);
    let shapes: Vec<&dyn Shape> = vec![&c1, &r1];
    for shape in shapes {
        println!("Shape: {}", {
            if shape.as_any().downcast_ref::<Circle>().is_some() { // Similar to virtualcasting in CPP which is still a bad practice for OOP in any language.
                "Circle"
            } else if shape.as_any().downcast_ref::<Rectangle>().is_some() {
                "Rectangle"
            } else {
                "unkown"
            }
        }
        );
        println!("Area: {}", shape.area());
        println!("Perimeter: {}", shape.perimeter());
    }
}
