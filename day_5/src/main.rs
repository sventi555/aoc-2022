use utils::read_lines;

fn swap_axes(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut swapped: Vec<Vec<char>> = Vec::new();

    if v.is_empty() {
        return v;
    };

    for _ in 0..v[0].len() {
        swapped.push(Vec::new());
    }

    for row in (0..v.len()).rev() {
        for (col, item) in swapped.iter_mut().enumerate().take(v[0].len()) {
            let c = v[row][col];
            if c != ' ' {
                item.push(v[row][col]);
            }
        }
    }

    swapped
}

fn part_a() {
    let lines: Vec<String> = read_lines("./day_5/input.txt").collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut instr_start_index = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            instr_start_index = index + 1;
            stacks = swap_axes(stacks);
            break;
        }

        stacks.push(Vec::new());
        for (index, c) in line.chars().enumerate() {
            if index % 4 == 1 {
                stacks.last_mut().unwrap().push(c);
            };
        }
    }

    for line in &lines[instr_start_index..] {
        let instr: Vec<usize> = line
            .split(' ')
            .enumerate()
            .filter(|(index, _)| index % 2 == 1)
            .map(|(_, val)| val.parse().unwrap())
            .collect();

        let num_moves = instr[0];
        let source = instr[1] - 1;
        let dest = instr[2] - 1;
        for _ in 0..num_moves {
            let item = stacks[source].pop().unwrap();
            stacks[dest].push(item);
        }
    }

    let top_items: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    println!("{}", top_items);
}

fn part_b() {
    let lines: Vec<String> = read_lines("./day_5/input.txt").collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut instr_start_index = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            instr_start_index = index + 1;
            stacks = swap_axes(stacks);
            break;
        }

        stacks.push(Vec::new());
        for (index, c) in line.chars().enumerate() {
            if index % 4 == 1 {
                stacks.last_mut().unwrap().push(c);
            };
        }
    }

    for line in &lines[instr_start_index..] {
        let instr: Vec<usize> = line
            .split(' ')
            .enumerate()
            .filter(|(index, _)| index % 2 == 1)
            .map(|(_, val)| val.parse().unwrap())
            .collect();

        let num_moves = instr[0];
        let source = instr[1] - 1;
        let dest = instr[2] - 1;

        // could probably use extend_from_slice here instead
        let mut intermediate_stack: Vec<char> = Vec::new();
        for _ in 0..num_moves {
            intermediate_stack.push(stacks[source].pop().unwrap());
        }
        for _ in 0..num_moves {
            stacks[dest as usize].push(intermediate_stack.pop().unwrap());
        }
    }

    let top_items: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    println!("{}", top_items);
}

fn main() {
    part_a();
    part_b();
}
