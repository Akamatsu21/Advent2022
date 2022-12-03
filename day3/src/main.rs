use std::io::Read;
use std::fs::File;
use std::path::Path;

fn char_to_int(c: char) -> u64
{
    match c
    {
        'a'..='z' => c as u64 - 'a' as u64 + 1,
        'A'..='Z' => c as u64 - 'A' as u64 + 27,
        _ => 0
    }
}

fn single_score(contents: &String) -> u64
{
    let mut score: u64 = 0;
    for line in contents.lines()
    {
        let (first, second) = line.split_at(line.len() / 2);
        for c in first.chars()
        {
            if second.contains(c)
            {
                score += char_to_int(c);
                break;
            }
        }
    }

    score
}

fn group_score(contents: &String) -> u64
{
    let mut score: u64 = 0;
    let mut lines = contents.lines();
    let mut group: [&str; 3] = ["", "", ""];
    loop
    {
        for i in 0..group.len()
        {
            if let Some(line) = lines.next()
            {
                group[i] = line;
            }
            else
            {
                return score;
            }
        }

        for c in group[0].chars()
        {
            if group[1].contains(c) && group[2].contains(c)
            {
                score += char_to_int(c);
                break;
            }
        }
    }
}

fn main()
{
    let file_path = Path::new("input.txt");
    let mut file = File::open(&file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let score1: u64 = single_score(&contents);
    let score2: u64 = group_score(&contents);

    println!("{score1}");
    println!("{score2}");
}
