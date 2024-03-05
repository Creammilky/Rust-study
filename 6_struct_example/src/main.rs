#[derive(Debug)] // Allow debug info output
struct Rectangle{
    width: u32,
    height: u32,
    area: u32   // 0 represents not calculated
}

impl Rectangle  {
    fn get_area(&self) -> u32{
        self.area
    }

    fn calc_area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle{
        width: 30,
        height : 50,
        area: 0,
    };
    let rec2 = get_rectangle(20,70);
    println!("{:?}", rec1);
    println!("{:#?}", rec1);

    println!("Area1: {}", get_area_by_calc(&rec1));
    println!("Area2: {}", rec2.calc_area());
}

fn get_rectangle(width: u32, height: u32) -> Rectangle{
    Rectangle{
        width,
        height,
        area: width * height,
    }
}

// Will it cause an overflow?
fn get_area(rec: &Rectangle) -> u32{
    rec.area
}

fn get_area_by_calc(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}