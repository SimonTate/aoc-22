use std::fs;
use std::env;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    let file = fs::File::open(filepath)
        .expect("FAILED TO READ FILE");
    let reader = BufReader::new(file);

    let mut calorie_count = 0;
    let mut elves_calories = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        // println!("line: {l}");
        if l == "" {
            // println!("calorie_count: {calorie_count}");
            elves_calories.push(calorie_count);
            calorie_count = 0;
        } else {
            let l_int : i32 = l.parse().unwrap();
            calorie_count += l_int;
        }
    }

    // println!("calorie_count: {calorie_count}");
    elves_calories.push(calorie_count);


    // Now get biggest <three> values.
    
    // sort them
    // println!("{:?}", elves_calories);
    elves_calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // println!("{:?}", elves_calories);
    elves_calories.truncate(3);

    let mut sum = 0;
    for c in &elves_calories {
        sum += c;
    }

    println!("{sum}");
}
