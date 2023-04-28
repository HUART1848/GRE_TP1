#[derive(Debug)]
struct SquareMatrix {
    width: usize,
    data: Vec<u32>
}

impl SquareMatrix {
    fn new(width: usize) -> Self {
        let data:Vec<u32> = vec![0; width * width];
        return SquareMatrix { width, data };
    }

    fn size(&self) -> usize {
        return self.width * self.width;
    }

    fn get(&self, i: usize, j: usize) -> u32 {
        return self.data[i * self.width + j];
    }

    fn set(&mut self, i: usize, j: usize, value: u32) {
        self.data[i * self.width + j] = value;
    }

    fn show(&self) {
        for i in 0..self.width {
            for j in 0..self.width {
                print!("{} ", self.get(i, j));
            }
            println!();
        }
    }
}

fn main() {
    let mut m = SquareMatrix::new(4);
    m.set(0, 3, 12);
    m.set(2, 1, 440);
    m.show();
}
