// I don't think I need to say this, just run it

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

const COUNT_RUNS: usize = 2;
const WIDTH_CHECK_LENGTH: isize = 3;
const HEIGHT_CHECK_LENGTH: isize = 3;
const COUNT_NEED_FOR_LIVE: usize = 3;
const COUNT_NEED_FOR_DIE: usize = 5;

fn main() {
    let mut map = [false;WIDTH*HEIGHT];
    if let Ok(data) = std::fs::read("./input") {
        for (ind, id) in data.iter().filter(|data| **data == 49 || **data == 48).enumerate() {
            map[ind] = *id == 49;
        }
    }

    for _ in 0..COUNT_RUNS {
        let old_map = map.clone();
        for (pos, is_live) in old_map.iter().enumerate() {
            let (x, y) = (pos%WIDTH, pos/HEIGHT);
            let mut count_lived_neighboring_cells = (0..HEIGHT_CHECK_LENGTH+1).map(|i| {check_ceils_line(&old_map, (x as isize-WIDTH_CHECK_LENGTH.div_euclid(2), y as isize-HEIGHT_CHECK_LENGTH.div_euclid(2)+i))}).sum::<usize>();
            if *is_live == true {count_lived_neighboring_cells-=1}
            map[pos] = count_lived_neighboring_cells >= COUNT_NEED_FOR_LIVE && COUNT_NEED_FOR_DIE >= count_lived_neighboring_cells;
        }
    }

    let mut out = "".to_string();
    for chunk in map.chunks(WIDTH) {
        out+=&format!("{:?}\n", chunk)
    }
    std::fs::write("./out", out).unwrap();
}

fn check_ceils_line(map: &[bool], pos: (isize, isize)) -> usize {
    let mut out = 0;
    for ind in 0..WIDTH_CHECK_LENGTH {
        if (ind+pos.0).abs()/WIDTH as isize > 0 {continue;}
        if let Ok(o) = TryInto::<usize>::try_into(ind+pos.0+pos.1*HEIGHT as isize) {
            if let Some(ceil) = map.get(o) {
                if *ceil == true {out+=1}
            }
        }
    }
    out
}
