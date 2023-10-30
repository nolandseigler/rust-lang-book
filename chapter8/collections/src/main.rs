use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    println!("{:?}", v);

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    println!("third is {:?}", third);
    match third {
        Some(third) => {
            println!("The third element is {third}");
        }
        None => println!("There is no third element."),
    }
    println!("third is {:?}", third);
    let no_match: Option<&i32> = v.get(999);
    match no_match {
        Some(no_match) => println!("The 999 element is {no_match}"),
        None => println!("There is no 999 element."),
    }

    for i in &v {
        println!("{i}");
    }

    let mut mut_v = vec![100, 32, 57];
    println!("{mut_v:?}");
    for i in &mut mut_v {
        *i += 50;
    }
    println!("{mut_v:?}");

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];
        println!("{v:?}");
        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Begin String

    // let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    println!("{s}");

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{s}");

    let s = String::from("initial contents");
    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{s2}");
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

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
    }

    println!("{:?}", map);
}
