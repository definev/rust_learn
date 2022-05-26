/// Closure and function in rust is different from other languages.
/// - Closure can take a variable from outside scope to manipulate.
/// - Function just manipulate variable in function scope.
///
/// -> Instead of using function, we can use closure take it as argument of function if we want manipulate something.
///
///
/// Closure has three trait `Fn`, `FnOnce`, `FnMut`
/// - `Fn`: Call and can not mutate.
/// - `FnOnce`: Call once time and can mutate.
/// - `FnMut`: Call multiple time and can mutate.
#[test]
fn advance_types() {
    // alias name
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    #[allow(unused)]
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    #[allow(unused)]
    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }

    #[allow(unused)]
    fn never_type_never_return() -> ! {
        panic!();
    }

    let returns_long_type_closures = || f();
    returns_long_type_closures();
}
