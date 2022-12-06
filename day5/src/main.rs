use std::io::Read;
use std::fs::File;
use std::path::Path;

fn get_top_crates(contents: &String, model: u64) -> String
{
    let mut stacks: Vec<Vec<char>> = vec![];

    match contents.lines().next()
    {
        Some(line) =>
        {
            let stacks_count: usize = (line.len() + 1) / 4;
            for _ in 0..stacks_count
            {
                stacks.push(vec![]);
            }
        },
        None => panic!("Invalid input.")
    }

    let mut stack_finished: bool = false;
    for line in contents.lines()
    {
        if line == ""
        {
            stack_finished = true;
        }
        else if stack_finished
        {
            let line: String = line.replace("move ", "").replace(" from ", "_").replace(" to ", "_");
            let params: Vec<&str> = line.split('_').collect();
            let amount = params[0].parse::<usize>().unwrap();
            let from = params[1].parse::<usize>().unwrap();
            let to = params[2].parse::<usize>().unwrap();

            match model
            {
                9000 =>
                {
                    for _ in 0..amount
                    {
                        match stacks[from - 1].pop()
                        {
                            Some(val) => stacks[to - 1].push(val),
                            None => ()
                        }
                    }
                },
                9001 =>
                {
                    let from_length: usize = stacks[from - 1].len();
                    let mut moved_crates: Vec<char> = stacks[from - 1].split_off(from_length - amount);
                    stacks[to - 1].append(&mut moved_crates);
                },
                _ => panic!("Invalid model.")
            }
        }
        else
        {
            for (i, c) in line.chars().enumerate()
            {
                if c == '['
                {
                    let next_char = line.as_bytes()[i + 1] as char;
                    stacks[i / 4].insert(0, next_char);
                }
            }
        }
    }

    let mut top_crates = String::new();
    for stack in stacks.iter()
    {
        match stack.get(stack.len() - 1)
        {
            Some(val) => top_crates.push(*val),
            None => top_crates.push(' ')
        }
    }

    top_crates
}

fn main()
{
    let file_path = Path::new("input.txt");
    let mut file = File::open(&file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let crates1: String = get_top_crates(&contents, 9000);
    let crates2: String = get_top_crates(&contents, 9001);

    println!("{crates1}");
    println!("{crates2}");
}