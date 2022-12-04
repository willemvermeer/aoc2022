fn main() {
    let lines = include_str!("../input.txt").lines();

    let mut total = 0;

    for line in lines {
        let x = line.split_once(" ");
        let i0 = x.unwrap().0.trim();
        let i1 = x.unwrap().1.trim();
        println!("{} {}", i0, i1);
        let result = match i0 {
            "A" => match i1 {
                "X" => 1 + 3, // Z
                "Y" => 2 + 6, //
                "Z" => 3 + 0,
                _ => 0,
            },
            "B" => match i1 {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => 0,
            },
            "C" => match i1 {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                _ => 0,
            },
            _ => 0,
        };
        total = total + result;
        println!("{}", result);
    }
    println!("Het antwoord is {}", total);

    total = 0;
    let lines2 = include_str!("../input.txt").lines();
    for line in lines2 {
        let x = line.split_once(" ");
        let i0 = x.unwrap().0.trim();
        let i1 = x.unwrap().1.trim();
        println!("{} {}", i0, i1);
        let result = match i0 {
            "A" => match i1 {
                "X" => 3 + 0, // Z, verlies
                "Y" => 1 + 3, // X, draw
                "Z" => 2 + 6, // Y, winst
                _ => 0,
            },
            "B" => match i1 {
                "X" => 1 + 0, // X, verlies
                "Y" => 2 + 3, // Y, draw
                "Z" => 3 + 6, // Z, winst
                _ => 0,
            },
            "C" => match i1 {
                "X" => 2 + 0, // Y, verlies
                "Y" => 3 + 3, // Z, draw,
                "Z" => 1 + 6, // X, winst
                _ => 0,
            },
            _ => 0,
        };
        total = total + result;
        println!("{}", result);
    }
    println!("Het tweede antwoord is {}", total);
}



// if line.is_empty() {
// _index = _index + 1;
// book_reviews.insert(_index, vec!());
// } else {
// let v = line.parse::<i32>().unwrap();
// println!("{}", v);
// let mut vec: Vec<i32> = book_reviews.get(&_index).unwrap().to_vec();
// book_reviews.remove(&_index);
// vec.push(v);
// book_reviews.insert(_index, vec);
// println!("{}", line);
// }
