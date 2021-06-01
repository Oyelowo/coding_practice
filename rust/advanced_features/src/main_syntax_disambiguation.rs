fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());

    /* Won't work cos Animal impl for Dog has no self. It is
    associated function, not associated method
    println!("A baby dog is called a {}", Animal::baby_name()); */
    // Solution: using fully qualified syntax (<Type as Trait>::function(receiver_of_method, next_arg, ...);)
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up.")
    }
}

impl Human {
    fn fly(&self) {
        println!("waving arms furiously.")
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
