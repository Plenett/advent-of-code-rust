use advent_utils::utils::read_file;


fn main() {

    // Read & Parse
    let sections = parse_input(read_file("input.txt"));

    // First Star
    let mut count_contains = 0;
    sections.iter().for_each(|(s1, s2)|{
        if s1.contains(s2) || s2.contains(s1) {
            count_contains += 1;
        }
    });

    println!("The first answer is : {:?}", count_contains);

    // Second Star
    let mut count_overlap = 0;
    sections.iter().for_each(|(s1, s2)|{
        if s1.overlaps(s2) {
            count_overlap += 1;
        }
    });

    println!("The second answer is : {:?}", count_overlap);
}

fn parse_input(data: String) -> Vec<(Section, Section)> {
    data.lines().map(|s| {
        let mut split = s.split(',');
        (Section::from_str(split.next().unwrap()), Section::from_str(split.next().unwrap()))
    }).collect()
}

#[derive(Debug)]
struct Section {
    start: u8,
    end: u8,
}

impl Section {
    fn from_str(str: &str) -> Section {
        let mut sec = str.split('-');
        Section { start: sec.next().unwrap().parse().unwrap() , end: sec.next().unwrap().parse().unwrap() }
    }

    fn contains(&self, section: &Section) -> bool {
        self.start <= section.start && self.end >= section.end
    }

    fn overlaps(&self, section: &Section) -> bool {
        self.start <= section.end && self.end >= section.start
    }
}