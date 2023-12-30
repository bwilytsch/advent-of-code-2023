use std::collections::HashMap;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Invalid spring"),
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    fn new(springs: Vec<Spring>, groups: Vec<usize>) -> Self {
        Record { springs, groups }
    }

    fn parse_line(line: &str, mul: usize) -> Result<Self, &str> {
        let mut result = line.split_whitespace();

        let mut springs = result
            .next()
            .ok_or("No springs")
            .unwrap()
            .chars()
            .map(|c| c.into())
            .collect::<Vec<_>>();

        let len = springs.len();

        let groups = result
            .next()
            .ok_or("No groups")
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>()
            .repeat(mul);

        // Expanding
        springs = springs
            .iter()
            .cloned()
            .chain([Spring::Unknown].iter().cloned())
            .cycle()
            .take(len * 5 + 4)
            .collect();

        Ok(Record::new(springs, groups))
    }
}

type Cache = HashMap<Record, usize>;

fn combos(record: &Record, memo: &mut Cache) -> usize {
    if let Some(&v) = memo.get(&record) {
        return v;
    }

    if record.groups.is_empty() {
        let v = match record.springs.iter().any(|s| *s == Spring::Damaged) {
            true => 0,
            false => 1,
        };

        memo.insert(record.clone(), v);
        return v;
    }

    if record.springs.len() < record.groups.iter().sum::<usize>() + record.groups.len() - 1 {
        memo.insert(record.clone(), 0);
        return 0;
    }

    if record.springs[0] == Spring::Operational {
        let solutions = combos(
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
            memo,
        );
        memo.insert(record.clone(), solutions);
        return solutions;
    }

    let mut solutions = 0;
    let cur = record.groups[0];
    let all_non_operational = record.springs[0..cur]
        .iter()
        .all(|s| *s != Spring::Operational);
    let end = (cur + 1).min(record.springs.len());
    if all_non_operational
        && ((record.springs.len() > cur && record.springs[cur] != Spring::Damaged)
            || record.springs.len() <= cur)
    {
        solutions = combos(
            &Record::new(record.springs[end..].to_vec(), record.groups[1..].to_vec()),
            memo,
        );
    }

    if record.springs[0] == Spring::Unknown {
        solutions += combos(
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
            memo,
        );
    }

    memo.insert(record.clone(), solutions);
    solutions
}

fn process(input: &str) -> Result<usize, String> {
    let mut cache = HashMap::new();

    let sum = input
        .lines()
        .into_iter()
        .map(|l| {
            let record = Record::parse_line(l, 5).unwrap();
            combos(&record, &mut cache)
        })
        .sum::<usize>();

    Ok(sum)
}

fn main() {
    let input = include_str!("./input.txt");
    match process(input) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_case_single() {
        let input = "???.### 1,1,3";
        assert_eq!(process(input).unwrap(), 1);
    }

    #[test]
    fn base_case_all() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(process(input).unwrap(), 525152);
    }
}
