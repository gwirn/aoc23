use std::{fs, i64};
pub fn day5() {
    // read data
    let data = fs::read_to_string("./inputs/day5.txt").expect("couldn't read file");
    // split by map
    let data_split = data.split("\n\n").collect::<Vec<_>>();
    // get seeds as ints from input
    let seeds = &mut data_split[0]
        .split(" ")
        .into_iter()
        .enumerate()
        .filter(|(cx, _)| *cx > 0)
        .map(|(_, x)| x.parse::<i64>().expect("cant parse seeds\n"))
        .collect::<Vec<_>>();
    //PART 2
    let seeds2 = seeds
        .chunks(2)
        .map(|x| x[0]..x[0] + x[1])
        .flatten()
        .collect::<Vec<_>>();
    let mut seeds = seeds2;

    let n_seeds = seeds.len();
    // iterate over all maps and skip seeds from input
    for i in &data_split[1..] {
        // which seeds were already mutated by the current map
        let mut mutated_seeds = vec![false; n_seeds];
        // iterate over all ranges of the map
        for (cj, j) in i.split("\n").into_iter().enumerate() {
            if cj > 0 && j.len() > 0 {
                // get range
                let range_def = j
                    .split(" ")
                    .into_iter()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                // iterate over seeds
                for cs in 0..n_seeds {
                    // if seed is in the current range and was not mutated
                    if seeds[cs] >= range_def[1]
                        && seeds[cs] <= range_def[1] + range_def[2]
                        && !mutated_seeds[cs]
                    {
                        // position of the seed in the range
                        let range_offset = range_def[1] - seeds[cs];
                        // mapping value at the seeds position
                        let new_seed_pos = range_def[0] + range_offset.abs();
                        seeds[cs] = new_seed_pos;
                        mutated_seeds[cs] = true
                    }
                }
            } else {
                println!("MAP: {:?}", j);
            }
        }
    }
    println!("Min value: {}", seeds.iter().min().unwrap());
}
