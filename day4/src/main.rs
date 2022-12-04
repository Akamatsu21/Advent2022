use std::io::Read;
use std::fs::File;
use std::path::Path;

fn main()
{
    let file_path = Path::new("input.txt");
    let mut file = File::open(&file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut overlapped: u64 = 0;
    let mut contained: u64 = 0;
    for line in contents.lines()
    {
        let sections_str: Vec<&str> = line.split(&[',', '-']).collect();
        let mut sections: Vec<u64> = vec![];
        for s in sections_str.iter()
        {
            sections.push(s.parse::<u64>().unwrap());
        }

        if sections[0] <= sections[3] && sections[2] <= sections[1]
        {
            overlapped += 1;

            if (sections[0] <= sections[2] && sections[1] >= sections[3])
            || (sections[2] <= sections[0] && sections[3] >= sections[1])
            {
                contained += 1;
            }
        }
    }

    println!("{contained}");
    println!("{overlapped}");
}
