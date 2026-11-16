#[derive(Debug)]
struct Rec {
    width: u32,
    height: u32
}

impl Rec {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rec) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let scale = 2;
    let rectl = Rec {
      width: dbg!(30 * scale)  ,
      height: 50,
    };

    let sqr = Rec::square(15);
    dbg!(&rectl);

    println!("area is: {}, can hold: {} ", rectl.area(), rectl.can_hold(&sqr));
}
