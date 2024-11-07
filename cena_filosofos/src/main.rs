use std::thread;

struct Philosopher {
    name: String
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher { name: name.to_string() }
    }

    fn eat(&self) {

        println!("{} is eating", self.name);

        thread::sleep_ms(1000);

        println!("{} has finished eaten", self.name);
    }
}

fn main() {

    let philosopher = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    for p in philosopher {
        p.eat();
    }
}