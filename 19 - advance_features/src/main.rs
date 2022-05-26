mod multiple_implement_method;
mod multiple_static_method;
mod super_trait;
mod unsafe_rust;
mod advance_types;
mod closures;
pub mod macros;

fn main() {}

#[test]
fn example_split_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = (&mut v).split_at_mut(3);

    assert_eq!(a, vec![1, 2, 3]);
    assert_eq!(b, vec![4, 5, 6]);
}
