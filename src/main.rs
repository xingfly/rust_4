fn main() {
    第一题枚举信号灯();
    第二题整数求和();
    第三题计算面积();
}

fn 第一题枚举信号灯() {
    println!("第一题枚举信号灯");
    println!("------------");
    let red = SignalLight::RED(20);
    let green = SignalLight::GREEN(25);
    let yellow = SignalLight::YELLOW(5);
    println!("红灯:{}秒", red.time());
    println!("绿灯:{}秒", green.time());
    println!("黄灯:{}秒", yellow.time());
    println!("------------");
    enum SignalLight {
        RED(u32),
        GREEN(u32),
        YELLOW(u32),
    }

    trait LightTime {
        fn time(&self) -> u32;
    }

    impl LightTime for SignalLight {
        fn time(&self) -> u32 {
            match self {
                &Self::RED(v) => v,
                &Self::GREEN(v) => v,
                &Self::YELLOW(v) => v,
            }
        }
    }
}

fn 第二题整数求和() {
    println!("第二题整数求和");
    println!("------------");
    let v = vec![1, 2, 3, 5];
    if let Some(value) = sum(&v) {
        println!("{}", value);
    } else {
        println!("结果溢出了")
    }
    println!("------------");
    fn sum(v: &[u32]) -> Option<u32> {
        let mut result: u32 = 0;
        for &item in v.iter() {
            let value = match result.checked_add(item) {
                None => return None,
                Some(v) => v,
            };
            result = value;
        }
        Some(result)
    }
}

fn 第三题计算面积() {
    println!("第三题计算面积");
    println!("------------");
    let c = Circle {
        radius: 1.0,
        name: String::from("圆形"),
    };
    let t = Triangle {
        base: 2.0,
        height: 3.0,
        name: String::from("三角形"),
    };
    let s = Square {
        length: 2.0,
        name: String::from("正方形"),
    };
    let calculator = Calculator { area: c };
    calculator.print_area();
    print_and_calculate(&s);
    print_and_calculate(&t);
    println!("------------");

    fn print_and_calculate<T: Area>(area: &T) {
        println!("{} - 面积是:{}", area.name(), area.area())
    }

    struct Calculator<T>
    where
        T: Area,
    {
        area: T,
    }

    impl<T> Calculator<T>
    where
        T: Area,
    {
        fn instance(&self) -> &T {
            &self.area
        }

        fn print_area(&self) {
            println!(
                "{} - 面积是:{}",
                self.instance().name(),
                self.instance().area()
            );
        }
    }

    trait Area {
        fn area(&self) -> f64;
        fn name(&self) -> &str;
    }

    struct Triangle {
        name: String,
        base: f64,
        height: f64,
    }

    struct Circle {
        name: String,
        radius: f64,
    }

    struct Square {
        name: String,
        length: f64,
    }

    impl Area for Triangle {
        fn area(&self) -> f64 {
            (self.base * self.height) / 2.0
        }
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl Area for Circle {
        fn area(&self) -> f64 {
            self.radius * self.radius * 3.14
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    impl Area for Square {
        fn area(&self) -> f64 {
            self.length * self.length
        }

        fn name(&self) -> &str {
            &self.name
        }
    }
}
