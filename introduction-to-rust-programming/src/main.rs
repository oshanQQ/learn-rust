fn main() {
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }
}

struct Iter {
    current: usize,
    max: usize,
}

// Iterator型を実装するために、2つのことを行う必要がある
impl Iterator for Iter {
    type Item = usize; // 出力する型の紐づけ

    // next()メソッドの実装
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}
