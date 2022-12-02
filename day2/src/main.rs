use std::io::Read;
use std::fs::File;
use std::path::Path;

fn strategy1(contents: &String) -> u64
{
    let mut score: u64 = 0;
    for line in contents.lines()
    {
        let items = line.split(" ").collect::<Vec<&str>>();

        match items[1]
        {
            "X" =>
            {
                score +=1;
                match items[0]
                {
                    "A" => score += 3,
                    "B" => (),
                    "C" => score += 6,
                    _ => panic!("Opponent string incorrect.")
                }
            },
            "Y" =>
            {
                score += 2;
                match items[0]
                {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => (),
                    _ => panic!("Opponent string incorrect.")
                }
            },
            "Z" =>
            {
                score += 3;
                match items[0]
                {
                    "A" => (),
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => panic!("Opponent string incorrect.")
                }
            },
            _ => panic!("Player string incorrect.")
        }
    }

    score
}

fn strategy2(contents: &String) -> u64
{
    let mut score: u64 = 0;
    for line in contents.lines()
    {
        let items = line.split(" ").collect::<Vec<&str>>();

        match items[1]
        {
            "X" =>
            {
                match items[0]
                {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => panic!("Opponent string incorrect.")
                }
            },
            "Y" =>
            {
                score += 3;
                match items[0]
                {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => panic!("Opponent string incorrect.")
                }
            },
            "Z" =>
            {
                score += 6;
                match items[0]
                {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => panic!("Opponent string incorrect.")
                }
            },
            _ => panic!("Player string incorrect.")
        }
    }

    score
}

fn main()
{
    let file_path = Path::new("input.txt");
    let mut file = File::open(&file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let score1: u64 = strategy1(&contents);
    let score2: u64 = strategy2(&contents);

    println!("{score1}");
    println!("{score2}");
}
