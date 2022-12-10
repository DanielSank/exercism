use std::collections::HashMap;
use std::cmp::Ordering;


pub struct Stats{
    wins: u32,
    losses: u32,
    draws: u32,
    name: String,
}

impl Stats{
    fn score(&self) -> u32 {
        self.wins * 3 + self.draws
    }
    pub fn new(wins: u32, losses: u32, draws: u32, name: String) -> Stats {
        Stats{wins, losses, draws, name}
    }
}

impl Ord for Stats{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score().cmp(&other.score()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match self.name.cmp(&other.name) {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering:: Greater,
                Ordering::Equal => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Stats{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Stats{
    fn eq(&self, other: &Self) -> bool {
        if self.score() == other.score() && self.name == other.name {
            return true
        }
        else {
            return false
        }
    }
}

impl Eq for Stats {}


pub fn tally(match_results: &str) -> String {
    let mut results: HashMap<&str, Stats> = HashMap::new();
    for line in match_results.lines() {
        let mut maybe_result = line.split(";");
        let (first, second, outcome) = (
            maybe_result.next().unwrap(),
            maybe_result.next().unwrap(),
            maybe_result.next().unwrap(),
        );
        match results.get(first) {
            Some(_r) => {},
            None => {
                let r = Stats {
                    wins: 0,
                    losses: 0,
                    draws: 0,
                    name: first.to_string(),
                };
                results.insert(first, r);
            }
        };
        match results.get(second) {
            Some(_r) => {},
            None => {
                let r = Stats {
                    wins: 0,
                    losses: 0,
                    draws: 0,
                    name: second.to_string(),
                };
                results.insert(second, r);
            }
        };
        match outcome {
            "win" => {
                results.entry(first).and_modify(|v| v.wins+=1);
                results.entry(second).and_modify(|v| v.losses += 1);
            },
            "loss" => {
                results.entry(second).and_modify(|v| v.wins += 1);
                results.entry(first).and_modify(|v| v.losses += 1);
            },
            "draw" => {
                results.entry(first).and_modify(|v| v.draws += 1);
                results.entry(second).and_modify(|v| v.draws += 1);
            },
            _ => panic!()
        };
    }
    let mut myvec: Vec<&Stats> = results.values().collect();
    myvec.sort();

    let length = 55;
    let length_for_table = 25;
    let length_for_team_name = length - length_for_table + 1;
    let mut strvec: Vec<String> = Vec::new();
    strvec.push("Team                           | MP |  W |  D |  L |  P".to_string());
    for result in myvec.iter().rev() {
        let s = format!("{: <len$}", result.name, len = length_for_team_name);
        let (wins, losses, draws) = (result.wins, result.losses, result.draws);
        let matches = wins + losses + draws;
        let table_row = format!("|  {} |  {} |  {} |  {} |  {}", matches, wins, draws, losses, result.score());
        strvec.push(s + &table_row)
    }
    strvec.join("\n").to_string()
}
