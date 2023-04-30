use std::fs;

#[derive(Debug)]
struct SquareMatrix {
    width: usize,
    data: Vec<Option<u32>>,
}

impl SquareMatrix {
    fn new(width: usize) -> Self {
        let data: Vec<Option<u32>> = vec![None; width * width];
        return SquareMatrix { width, data };
    }

    fn size(&self) -> usize {
        return self.width * self.width;
    }

    fn get(&self, i: usize, j: usize) -> Option<u32> {
        return self.data[i * self.width + j];
    }

    fn set(&mut self, i: usize, j: usize, value: u32) {
        self.data[i * self.width + j] = Some(value);
    }

    fn fill_diagonal(&mut self, value: u32) {
        for i in 0..self.width {
            self.set(i, i, value);
        }
    }

    fn show(&self) {
        for i in 0..self.width {
            for j in 0..self.width {
                print!(
                    "{} ",
                    self.get(i, j).map_or("-".to_owned(), |c| c.to_string())
                );
            }
            println!();
        }
    }

    fn parse(string: &str) -> SquareMatrix {
        let mut lines = string.lines();
        let mut ret = SquareMatrix::new(lines.next().unwrap().parse().unwrap());

        // Ignore edges count
        lines.next();

        for line in lines {
            let content:Vec<u32> = line.split(" ").map(|c| c.parse().unwrap()).collect();

            ret.set(content[0] as usize - 1, content[1] as usize - 1, content[2]);
        }

        ret.fill_diagonal(0);
        return ret;
    }
}

fn main() {
    let content = fs::read_to_string("data/reseau3.dat").expect("File not found");
    let matrix = SquareMatrix::parse(content.as_str());
    matrix.show();
}
