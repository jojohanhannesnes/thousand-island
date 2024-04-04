const FIRST: usize = 0;
const SECOND: usize = 1;

pub struct Fibonacci {
    pub values: Vec<usize>,
    pub total: usize,
}

impl Fibonacci {
    pub fn new(limit: usize) -> Self {
        let val = fibonacci(limit);
        Fibonacci {
            values: val.clone().collect(),
            total: val.clone().sum(),
        }
    }

    pub fn even_fib(&mut self) {
        let iterators = self.values.clone().into_iter().filter(|x| *x % 2 == 0);

        self.values = iterators.clone().collect();
        self.total = iterators.clone().sum();
    }
}

fn fibonacci(limit: usize) -> impl Iterator<Item = usize> + Clone {
    std::iter::successors(Some((SECOND, FIRST)), move |&(a, b)| {
        if a + b < limit {
            Some((b, a + b))
        } else {
            None
        }
    })
    .map(|(_, b)| b)
}
