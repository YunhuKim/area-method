struct Object{
    width: u32,
    height: u32,
}

impl Object{
    fn area(&self) -> u32{
        self.width * self.height
    }
    
}

impl Object{
    fn show(&self){
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

fn main() {
    let o = Object {
        width: 35,
        height: 55,
    };

    o.show();
}
