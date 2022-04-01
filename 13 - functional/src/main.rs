fn closure_test() {
    let cacher = Cacher::new(|x: usize| x * 2);
    let mut cache_cacher = Cacher::new(move |_| cacher);

    assert_eq!(cache_cacher.value(cacher).value(30), 60);

    cache_cacher.recache(cacher).recache(300);
    assert_eq!(cache_cacher.value.unwrap().value.unwrap(), 600);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let v0_iter = v1.iter();
    for ele in v0_iter {
        println!("Print ele: {}", ele);
    }

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v2: Vec<_> = v1
        .iter()
        .map_while(|x| if x % 2 == 0 { None } else { Some(x) })
        .collect();
    println!("{:?}", v2);
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2 = v1.iter().map(|x| x + 1).collect::<Vec<i32>>();

    println!("{}", v2.iter().sum::<i32>());
}

#[derive(Clone, Copy)]
struct Cacher<T, V>
where
    T: Fn(V) -> V,
    V: Copy,
{
    calculation: T,
    value: Option<V>,
}

impl<T, V> Cacher<T, V>
where
    T: Fn(V) -> V,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, V> {
        return Cacher {
            calculation,
            value: None,
        };
    }

    fn value(&mut self, value: V) -> V {
        match self.value {
            Some(v) => v,
            None => {
                let value = (&self.calculation)(value);
                self.value = Some(value);
                value
            }
        }
    }

    fn recache(&mut self, value: V) -> V {
        self.value = Some((&self.calculation)(value));
        value
    }
}
