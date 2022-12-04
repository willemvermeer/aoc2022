fn main() {
    let lines = include_str!("../input.txt").lines();

    let mut total = 0;

    for line in lines {
        println!("{}", line);
        let z = line.trim().len();
        let x = &line[0..(z / 2)];
        let y = &line[(z / 2)..z];

        println!("{} {} {}", x, y, z);
        let inboth: Vec<char> = x.chars().map(|c| if y.contains(&c.to_string()) { c } else { '-' }).collect::<Vec<char>>();
        println!("asasdas{:?}", inboth);

        let zz: &char = inboth.iter().filter(|c| *c != &'-').collect::<Vec<&char>>()[0];

        let zz3: char = *zz;
        let zzval = zz3 as u32;
        let bruh = if zz3.is_lowercase() {
            1 + zzval - ('a' as u32)
        } else {
            1 + 26 + zzval - ('A' as u32)
        };
        println!("{}", zz3 as u32);
        println!("{}", bruh);

        println!("{}", "abc".contains('d'));

        total = total + bruh;
    }
    println!("Het antwoord is {}", total);

    let lines2 = include_str!("../input.txt").lines();
    total = 0;
    let mut l0 = "";
    let mut l1 = "";
    let mut l2 = "";

    let mut linecounter = 0;
    for line in lines2 {
        println!("{}", line);
        let z = line.trim().len();

        if linecounter % 3 == 0 {
            l0 = line;
            linecounter = linecounter + 1;
        } else if linecounter % 3 == 1 {
            l1 = line;
            linecounter = linecounter + 1;
        } else {
            l2 = line;
            linecounter = linecounter + 1;
            let inboth: Vec<char> = l0.chars().map(|c| if l1.contains(&c.to_string()) { c } else { '-' }).collect::<Vec<char>>();
            println!("{:?}",inboth);
            let in3: Vec<char> = l2.chars().map(|c| if inboth.contains(&c) { c } else { '-' }).collect::<Vec<char>>();
            println!("{:?}",in3);

            let zz: &char = in3.iter().filter(|c| *c != &'-').collect::<Vec<&char>>()[0];
            let zz3: char = *zz;
            let zzval = zz3 as u32;
            let bruh = if zz3.is_lowercase() {
                1 + zzval - ('a' as u32)
            } else {
                1 + 26 + zzval - ('A' as u32)
            };
            total = total + bruh;
        }
    }
    println!("Het antwoord op vraag is {}", total);
}
