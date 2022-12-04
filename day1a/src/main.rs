use std::collections::HashMap;

pub fn main() {

    let mut _index = 0;
    let mut book_reviews: HashMap<i32, Vec<i32>> = HashMap::new();
    book_reviews.insert(0, vec!());
    let lines = include_str!("../input.txt").lines();

    for line in lines {
        if line.is_empty() {
            _index = _index + 1;
            book_reviews.insert(_index, vec!());
        } else {
            let v = line.parse::<i32>().unwrap();
            println!("{}", v);
            let mut vec: Vec<i32> = book_reviews.get(&_index).unwrap().to_vec();
            book_reviews.remove(&_index);
            vec.push(v);
            book_reviews.insert(_index, vec);
            println!("{}", line);
        }
    }
    println!("{:?}", book_reviews);

    // let result = book_reviews.iter()
    //     .map(|(_k,v)| {println!("{:?}",v.iter().count());v});

    let mut maxi = 0;
    let mut sums:Vec<i32> = Vec::new();
    for (key, value)in book_reviews {
        let sum = value.iter().fold(0, |acc,x| acc + x);
        println!("{}", sum);
        if sum > maxi { maxi = sum; }
        sums.push(sum);
    }

    println!("Het antwoord is {:?}", maxi);

    println!("{:?}", sums);

    sums.sort();

    println!("{:?}", sums);

    sums.reverse();

    println!("{:?}", sums);
    // sums.sort()

    let r2 = sums.get(0).unwrap() + sums.get(1).unwrap() + sums.get(2).unwrap();
    println!("Het tweede antwoord is {}", r2);
        // .group_by(|l| l.isEm/**/pty());
        // .map(|l| l.parse::<i32>())
    // let lines = include_str!("../example.txt")
    //     .lines()
    //     .map(|l| l.parse::<i32>())
    //     .fold(0, |acc, x| { acc + x});
        // .map(|l| l.split_once(" ").unwrap());

    // println!("{:?}", lines);


    // let f = vec!("a", "b", "c");
    // let d = f.iter().count();
    //
    // let g = vec!(1,2,3);
    // let g2: Vec<i32> = g.iter().map(|g3| g3 + 1).collect();
    // println!("{:?}", g2);
    //
    // let w = g2.iter().fold(0, |acc, q| acc + q);
    // println!("{}", w);
    //
    // let r: Vec<String> = f.iter().map(|x| format!("{} {}", x, x)).collect();

    // let qq = "1".parse::<i32>().unwrap();
    // println!("{}", qq);
    // let (f, d) = include_str!("../input.txt")
    //     .lines()
    //     .map(|l| l.split_once(" ").unwrap())
    //     .fold((0, 0), |(f, d), (k, v)| {
    //         match (k, v.parse::<i32>().unwrap()) {
    //             ("forward", v) => (f + v, d),
    //             ("down", v) => (f, d + v),
    //             ("up", v) => (f, d - v),
    //             _ => unreachable!(),
    //         }
    //     });

    // println!("{}", f * d);
    // println!("{:?}", f);
    // println!("{:?}", r);
    // println!("{}", d);
}