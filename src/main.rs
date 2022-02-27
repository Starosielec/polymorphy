

#[allow(non_camel_case_types)]
trait poly {
    type Output;
    fn poly(self) -> Self::Output;
}

impl poly for (i32, &f64) {
    type Output = u8;
    fn poly(self) -> Self::Output {
        let (integer, float) = self;
        println!("I am polymorphic fn poly(integer: i32={}, float: &f64={}) -> u8", integer, float);
        8u8
    }
}

impl poly for (&str, char) {
    type Output = ();
    fn poly(self) -> Self::Output {
        let (string, character) = self;
        println!("I am polymorphic fn poly(string: &str={}, character: char={}) -> u8", string, character);
    }
}

impl poly for (&mut u8, ()) {
    type Output = u8;
    fn poly(self) -> Self::Output {
        let (target, _) = self;
        println!("I am polymorphic fn poly(target: &mut u8={}) -> u8", target);
        *target = *target * 2;
        *target
    }
}

#[inline(always)]
fn poly<O, A, B>(a: A, b: B) -> O
    where (A, B): poly<Output=O> {
        <(A, B) as poly>::poly((a, b))
}

fn poly_1<O, A>(a: A) -> O
    where (A, ()): poly<Output=O> {
        <(A, ()) as poly>::poly((a, ()))
}

fn main() {
    println!("Hello, polymorphic world!");
    let u = poly("string", 'c');
    println!("poly() -> {:?}", u);
    let mut u = poly(32i32, &64.0f64);
    println!("poly() -> {:?}", u);
    let w = poly_1(&mut u);
    println!("poly() -> {:?}, u={:?}", w, u);
}
