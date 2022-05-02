struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    a: Point<T>,
    b: Point<T>,

}

fn generics() {
    let a = Point { x:0, y: 4};
    let b = Point { x:4, y: 10};

    let line = Line { a, b };
}

fn main() {
    generics();
}
