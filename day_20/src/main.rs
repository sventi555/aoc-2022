use utils::read_lines;

fn part_a() {
    let original: Vec<i32> = read_lines("./day_20/input.txt")
        .map(|line| line.parse().unwrap())
        .collect();

    let mut nums = original.clone();

    let len = nums.len();

    let mut cur_index = 0;
    let mut index_in_nums = 0;
    while cur_index < len {
        let item = original[cur_index];
        while nums[index_in_nums] != item {
            index_in_nums += 1;
        }

        let index_i32 = index_in_nums as i32;
        let modulo = len as i32 - 1;

        let new_index = ((((item + index_i32) % modulo) + modulo) % modulo) as usize;

        nums.remove(index_in_nums);
        nums.insert(new_index, item);

        cur_index += 1;
    }

    let zero_index = nums.iter().position(|num| *num == 0).unwrap();
    let coordinate_sum = nums[(zero_index + 1000) % len]
        + nums[(zero_index + 2000) % len]
        + nums[(zero_index + 3000) % len];

    println!("{coordinate_sum}");
}

#[derive(Clone)]
struct Num {
    val: i64,
    id: usize,
}

impl Num {
    fn new(val: i64, id: usize) -> Self {
        Num { val, id }
    }
}

fn part_b() {
    let original: Vec<Num> = read_lines("./day_20/input.txt")
        .enumerate()
        .map(|(index, line)| Num::new(line.parse::<i64>().unwrap() * 811589153, index))
        .collect();

    let mut nums = original.clone();

    let len = nums.len();

    for _ in 0..10 {
        let mut cur_index = 0;
        let mut index_in_nums = 0;
        while cur_index < len {
            let num = &original[cur_index];
            // need to add an id to uniquely identify duplicate values
            while nums[index_in_nums].id != num.id {
                index_in_nums = (index_in_nums + 1) % len;
            }

            let item = num.val;

            let index_i64 = index_in_nums as i64;
            let modulo = len as i64 - 1;

            let new_index = ((((item + index_i64) % modulo) + modulo) % modulo) as usize;

            nums.remove(index_in_nums);
            nums.insert(new_index, num.clone());

            cur_index += 1;
        }
    }

    let zero_index = nums.iter().position(|num| num.val == 0).unwrap();
    let coordinate_sum = nums[(zero_index + 1000) % len].val
        + nums[(zero_index + 2000) % len].val
        + nums[(zero_index + 3000) % len].val;

    println!("{coordinate_sum}");
}

fn main() {
    part_a();
    part_b();
}
