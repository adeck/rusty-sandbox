
#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn square(width: u16) -> Self {
        Rectangle {
            width,
            height: width
        }
    }

    fn fits_within(&self, other: &Self) -> bool {
        self.width < other.width && self.height < other.height
    }
}


fn main() {
    let r = Rectangle::square(5);
    let r2 = Rectangle::square(6);
    println!("r: {:?}", r);
    println!("r2: {:?}", r2);
    println!("r < r2: {}", r.fits_within(&r2));
    println!("r2 < r: {}", r2.fits_within(&r));
}



