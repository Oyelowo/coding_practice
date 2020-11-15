fn main() {
    iterator_adaptors();
}


// Creating Our Own Iterators with the Iterator Trait
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self{
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self)-> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else{
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods(){
    let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a,b)| a*b)
    .filter(|x| x % 3 == 0)
    .sum();

    assert_eq!(18, sum);
}

#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32)-> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);



        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}





fn iterator_adaptors() {
    let v1 = vec![1,2,3];
    // v1.iter().map(|x| x + 1); // This is lazy and wont get called until collected with the collect method
    let v2 :Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2,3,4]);

}

fn consuming_adaptors() {
    /* 
    Methods that call next are called consuming adaptors, because calling them uses up the iterator. One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete.
    */
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total :i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
fn iterator_traits() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    for val in v1_iter {
        println!("Got: {}", val);
    }
}