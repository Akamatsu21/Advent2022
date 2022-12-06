use std::collections::HashSet;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn find_marker(contents: &String, length: usize) -> usize
{
    let mut marker: usize = contents.len();
    for i in (length - 1)..contents.len()
    {
        let mut last_four = HashSet::<char>::new();
        let range_start: usize = i - (length - 1);

        for c in contents[range_start..=i].chars()
        {
            last_four.insert(c);
        }

        if last_four.len() == length
        {
            marker = i + 1;
            break;
        }
    }

    marker
}

fn main()
{
    let file_path = Path::new("input.txt");
    let mut file = File::open(&file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let marker1: usize = find_marker(&contents, 4);
    let marker2: usize = find_marker(&contents, 14);

    println!("{marker1}");
    println!("{marker2}");
}
