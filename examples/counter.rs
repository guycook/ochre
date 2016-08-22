#[macro_use]
extern crate ochre;

struct CounterData {
    count: i32,
}

impl ochre::New for CounterData {
    fn new() -> CounterData {
        CounterData { count: 0 }
    }
}

ochre_app!(Counter);

fn main() {
    let mut app = Counter::new();
    app.run();
}
