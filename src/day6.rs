use std::fs;
fn dist_travel(hold_time: i64, race_time: i64) -> i64 {
    let time_remainder = race_time - hold_time;
    time_remainder * hold_time
}
pub fn day6() {
    let data = fs::read_to_string("./inputs/day6.txt").expect("couldn't read file");
    /*
    // START PART 1
    // split data into times and record distances
    let split_data: Vec<Vec<i64>> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|y| {
            let yy = y.split_whitespace();
            yy.filter(|w| !w.contains(":"))
                .map(|z| z.parse::<i64>().expect("cant convert to i64"))
                .collect::<Vec<_>>()
        })
        .collect();
    // END PART 1
    */

    // START PART 2
    // split data into time and record distance
    // only a Vec<Vec<i64>> to be easier compatible with Part 1
    let split_data: Vec<Vec<i64>> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|y| {
            y.split(":")
                .enumerate()
                .filter(|(cw, _)| *cw > 0)
                .map(|(_, w)| {
                    w.replace(" ", "")
                        .parse::<i64>()
                        .expect("cant parse to i64")
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // END PART 2
    let mut n_wins = 1;
    for i in 0..split_data[0].len() {
        let time = split_data[0][i];
        let record_dist = split_data[1][i];
        let mut times: Vec<i64> = vec![];
        // the time to currently test
        let mut time_while = 1;
        // previous max dist with which you won
        let mut prev_dist = 0;
        // whether your best win is already reached - are kind of bell shaped
        let mut reached_peak = false;
        loop {
            // the distance with the current setting
            let d = dist_travel(time_while, time);
            // is this a winning dist
            if d > record_dist {
                times.push(d);
            }
            // did we already find max performance
            if prev_dist > d {
                reached_peak = true;
            }
            // stop if we don't find winning solution anymore
            if reached_peak && d < record_dist {
                break;
            }
            prev_dist = d;
            time_while += 1;
        }
        // count wins
        n_wins = n_wins * times.len();
    }
    println!("{}", n_wins);
}
