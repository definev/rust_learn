#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[derive(Clone, Copy, Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 40,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 39,
                style: String::from("sandal"),
            },
            Shoe {
                size: 40,
                style: String::from("boots"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 40);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 40,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 40,
                    style: String::from("boots"),
                }
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let counter = Counter::new();
        let mut counter_iter = counter.into_iter();

        for count in counter.into_iter() {
            println!("{}", count);
        }

        assert_eq!(counter_iter.next(), Some(1));
        assert_eq!(counter_iter.next(), Some(2));
        assert_eq!(counter_iter.next(), Some(3));
        assert_eq!(counter_iter.next(), Some(4));
        assert_eq!(counter_iter.next(), Some(5));
        assert_eq!(counter_iter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum = Counter::new()
            .zip(Counter::new().map(|x| x * x))
            .map(|(a, b)| a * b)
            .collect::<Vec<u32>>();

        println!("{:?}", sum);
    }
}

/*
Filter { 
    iter: Map { 
        iter: Zip { 
            a: Counter { count: 0 }, 
            b: Skip { iter: Counter { count: 0 }, n: 1 } 
        } 
    } 
}
*/
