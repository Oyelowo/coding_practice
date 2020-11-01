use std::collections::HashMap;

fn main() {
    // some1();
    // some2();
    // someEnum();
    // string_utf8_tut()
    hashmap_tut()
}

fn hashmap_tut() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("onbe, {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}{}", field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
        println!("{:?}", count);
    }

    println!("{:?}", map);
}

fn string_utf8_tut() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");

    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let j = String::from("otipo");
    let p = &j[..];

    let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    let answer = &hello.as_bytes();
    println!("byte shit {:?}", answer.len());

    let kk = "नमस्ते";
    let aa = kk.as_bytes();
    println!("Devanagari ,{:?}", &kk.as_bytes());
    println!("Devanagari ,{:?}", &kk.escape_unicode());

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn someEnum() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(32.3),
    ];
}

fn some2() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}

fn some1() {
    let mut v: Vec<i32> = Vec::new();
    v.push(39);
    v.push(39);

    let first = &v[0];

    println!("the first ele is: {:?}", first);

    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];
    // v2.push(3);
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("third ele is {}", third),
        None => println!("No third ele"),
    }

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
