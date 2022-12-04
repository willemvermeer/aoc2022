fn main() {
    let lines = include_str!("../input.txt").lines();

    let mut total = 0;
//2-6,4-8
    for line in lines {
        println!("{}", line);
        let x = line.split_once(",");
        let left = x.unwrap().0.trim();
        let right = x.unwrap().1.trim();

        let (l0,l1) =
            (left.split_once("-").unwrap().0.trim().parse::<i32>().unwrap(),
             left.split_once("-").unwrap().1.trim().parse::<i32>().unwrap());
        let (r0,r1) =
            (right.split_once("-").unwrap().0.trim().parse::<i32>().unwrap(),
             right.split_once("-").unwrap().1.trim().parse::<i32>().unwrap());

        println!("{:?}", line);
        println!("{}-{},{}-{}", l0, l1, r0, r1);

        if (l0 >= r0 && l1 <= r1) || (r0 >= l0 && r1 <= l1) {
            println!("{:?} contained", line);
            total = total + 1;
        } else {
            println!("{:?} not contained", line);
        }
    }
    println!("Het antwoord is {}", total);

    let lines2 = include_str!("../input.txt").lines();

    total = 0;
//2-6,4-8
    for line in lines2 {
        println!("{}", line);
        let x = line.split_once(",");
        let left = x.unwrap().0.trim();
        let right = x.unwrap().1.trim();

        let (l0,l1) =
            (left.split_once("-").unwrap().0.trim().parse::<i32>().unwrap(),
             left.split_once("-").unwrap().1.trim().parse::<i32>().unwrap());
        let (r0,r1) =
            (right.split_once("-").unwrap().0.trim().parse::<i32>().unwrap(),
             right.split_once("-").unwrap().1.trim().parse::<i32>().unwrap());

        println!("{:?}", line);
        println!("{}-{},{}-{}", l0, l1, r0, r1);

        if (l1 >= r0 && l0 <= r1) || (r1 >= l0 && r1 <= l0) {
            println!("{:?} contained", line);
            total = total + 1;
        } else {
            println!("{:?} not contained", line);
        }
    }
    println!("Het antwoord is {}", total);

}
