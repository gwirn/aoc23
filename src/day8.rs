use std::collections::HashMap;
use std::fs;
// greatest common divisor
fn gcd(a: i128, b: i128) -> i128 {
    let mut aa = a.clone();
    let mut bb = b.clone();
    while bb > 0 {
        let temp = bb;
        bb = aa % bb;
        aa = temp;
    }
    aa
}
// least common multiple
fn lcm(n1: i128, n2: i128) -> i128 {
    let n = (n1 * n2 / gcd(n1, n2)) as f64;
    let nn = n.floor() as i128;
    nn
}
pub fn day8() {
    // read data
    let raw_data = fs::read_to_string("./inputs/day8.txt").expect("cant read file");
    let data = raw_data.split("\n").collect::<Vec<_>>();
    //  convert instructions
    let instructions = data[0]
        .trim()
        .split("")
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>();
    // convert direction instructions to index
    let dir_to_idx = HashMap::from([("L", 0 as usize), ("R", 1 as usize)]);
    // parse the mapping (input)
    let mapping = data[1..]
        .iter()
        .filter(|x| x.len() > 1)
        .map(|x| {
            let x_bind = x.replace(" ", "");
            let x_nspace = x_bind.split("=").collect::<Vec<_>>();
            let lr_bind = x_nspace[1].replace("(", "").replace(")", "");
            let lr = lr_bind.split(",").collect::<Vec<_>>();
            let mkey = x_nspace[0].to_string();
            let directions: [String; 2] = [lr[0].to_string(), lr[1].to_string()];
            (mkey, directions)
        })
        .collect::<HashMap<_, _>>();
    /*
    // PART 1
    // Starting positions
    let start = "AAA".to_string();
    let end = "ZZZ".to_string();
    // number of steps needed
    let mut n_steps = 0;
    // which direction instruction to use
    let mut dir_idx = 0;
    //  current position after mapping (after step)
    let mut cur_pos = start;
    // number of direction instructions
    let n_instruct = instructions.len();
    loop {
        // restart instruction index if reached end
        if dir_idx >= n_instruct {
            dir_idx = 0
        }
        //  get current position from mapping
        cur_pos = mapping.get(&cur_pos).expect("Step not in mapping")[*dir_to_idx
            .get(instructions[dir_idx])
            .expect("Dir not in dir map")]
        .clone();
        n_steps += 1;
        dir_idx += 1;
        if cur_pos == end {
            break;
        }
    }
    println!("{:?}", n_steps);
    */
    // PART 2
    // collect all starting positions
    let mut cur_pos = mapping
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    let n_instruct = instructions.len();
    // number of steps each starting position needs until ..Z is reached
    let mut all_steps: Vec<usize> = vec![];
    for ci in 0..cur_pos.len() {
        println!("Start {}", ci);
        let mut n_steps = 0;
        let mut dir_idx = 0;
        // see PART 1
        loop {
            if dir_idx >= n_instruct {
                dir_idx = 0
            }
            cur_pos[ci] = mapping.get(&cur_pos[ci]).expect("Step not in mapping")[*dir_to_idx
                .get(instructions[dir_idx])
                .expect("Dir not in dir map")]
            .clone();
            n_steps += 1;
            dir_idx += 1;
            if cur_pos[ci].ends_with("Z") {
                all_steps.push(n_steps.clone());
                break;
            }
        }
    }
    //  calculate the least common multiple to get where all starting positions meet
    let mut inter_lcm = lcm(all_steps[0] as i128, all_steps[1] as i128);
    for i in 2..all_steps.len() {
        inter_lcm = lcm(inter_lcm, all_steps[i] as i128);
    }
    println!("Meeting cycle: {}", inter_lcm);
}
