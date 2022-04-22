
fn main() {
    let r = Rectangle::new(2, 5);
    let s1 = Rectangle::square(4);
    let s2 = Rectangle::square(7);
    for cur in [&r, &s1, &s2] {
        println!("cur: {:?}", cur);
    }
    println!("r < s1 (false): {}", r.fits_within(&s1)); 
    println!("s1 < r (false): {}", s1.fits_within(&r)); 
    println!("r < s2 (true): {}", r.fits_within(&s2)); 
    println!("s2 < r (false): {}", s2.fits_within(&r)); 
}

#[derive(Debug)]
struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {
    fn new(width: u8, height: u8) -> Self {
        Rectangle {
            width,
            height,
        }
    }

    fn square(width: u8) -> Self {
        Rectangle {
            width,
            height: width,
        }
    }

    fn fits_within(&self, other: &Self) -> bool {
        self.width < other.width && self.height < other.height
    }
}

