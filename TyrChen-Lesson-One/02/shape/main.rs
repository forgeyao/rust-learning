// 思考题
// 要构造一个数据结构 Shape，可以是 Rectangle、Circle 或是 Triangle，这三种结构如下代码。请问 Shape
// 类型该用什么数据结构实现？怎么实现？
// 对于上面的三种结构，如果我们要定义一个接口，可以计算周长和面积，怎么计算？

// 解题思路
// 考虑用继承，Shape 做基类，其他继承 Shape。实现时发现 Rust 不支持继承，就放弃这方案了
// 参考评论区解答，发现可以用枚举

enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Cir(c) => c.area(),
            Shape::Tri(t) => t.area(),
        }
    }
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.perimeter(),
            Shape::Cir(c) => c.perimeter(),
            Shape::Tri(t) => t.perimeter(),
        }
    }
}

trait Size {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    a: f64,
    b: f64,
}
impl Size for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.a + self.b)
    }
}

struct Circle {
    r: f64,
}

impl Size for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.r
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Size for Triangle {
    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn main() {
    let _re = Rectangle { a: 1.0, b: 2.0 };
    let _ci = Circle { r: 0.5 };
    let _tr = Triangle {
        a: 3.0,
        b: 2.0,
        c: 3.0,
    };

    let sha1 = Shape::Rec(_re);
    let sha2 = Shape::Cir(_ci);
    let sha3 = Shape::Tri(_tr);

    println!("Rectangle:{}, {}", sha1.area(), sha1.perimeter());
    println!("Circle:{}, {}", sha2.area(), sha2.perimeter());
    println!("Triangle:{}, {}", sha3.area(), sha3.perimeter());
}
