use std::{thread, time::Duration, collections::HashMap};


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

     // generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout_with_caching(simulated_user_specified_value, simulated_random_number);
}

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

/* fn xample_clusure_defs() {
    fn  add_one_v1   (x: u32) -> u32 { x + 1 } // Normal function
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

    let add_one_v0 = |x| x + 1;
    let k = add_one_v0(3);

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // won't work. String already inferre
} */

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32,u32>,
}

impl <T> Cacher<T>
where T: Fn(u32)->u32
{
    fn new(calculation: T)-> Cacher<T> {
        Cacher{
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32{
  
        match self.value.get(&arg) {
            Some(v)=> *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

struct Cacher_old<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher_old<T>
where T: Fn(u32)->u32
{
    fn new(calculation: T)-> Cacher_old<T> {
        Cacher_old{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) ->u32{
        match self.value {
            Some(v)=>v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

fn generate_workout_with_caching(intensity: u32, random_number: u32) {
    //let expensive_result = simulate_expensive_calculation(intensity);
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(5));
        num
    });
    


    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(66));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(66));
        println!("Next, do {} situps!", expensive_closure.value(66));
        println!("Next, do {} situps!", expensive_closure.value(66));
        println!("Next, do {} situps!", expensive_closure.value(66));
        println!("Next, do {} situps!", expensive_closure.value(88));
        println!("Next, do {} situps!", expensive_closure.value(89));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_closure.value(intensity));
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_result = simulate_expensive_calculation(intensity);
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    


    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity));
        }
    }
}
