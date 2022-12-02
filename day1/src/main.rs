use std::io::Read;
use std::fs::File;
use std::path::Path;

fn main()
{
    let file_path = Path::new("input.txt");
    let mut file = File::open(&file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut elves: Vec<u64> = vec![];
    let mut current_total: u64 = 0;
    let mut max: [u64; 3] = [0, 0, 0];
    for line in contents.lines()
    {
        match line
        {
            "" => 
            {
                elves.push(current_total);
                for i in 0..max.len()
                {
                    if current_total > max[i]
                    {
                        let buffer = max[i];
                        max[i] = current_total;
                        current_total = buffer;
                    }
                }
                current_total = 0;
            },
            _ =>
            {
                current_total +=
                    match line.parse::<u64>()
                    {
                        Ok(x) => x,
                        Err(e) =>
                        {
                            println!("{e}");
                            0
                        }
                    };
            }
        }
    }

    for i in 0..max.len()
    {
        println!("{}", max[i]);
    }

    println!("{}", max[0] + max[1] + max[2]);
}