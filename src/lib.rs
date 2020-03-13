#[derive(Debug)]
pub struct Game<'a> {
    team1: &'a str,
    team1_score: u8,
    team2: &'a str,
    team2_score: u8,
}

pub fn parse_game(line: &str) -> Game {
    let teams: Vec<&str> = line.split(", ").collect();
    let (team1, team1_score) = parse_team(teams[0]);
    let (team2, team2_score) = parse_team(teams[1]);
    Game { team1: team1, team1_score: team1_score, team2: team2, team2_score: team2_score }
}

fn parse_team(s: &str) -> (&str, u8) {
    let result: Vec<&str> = s.rsplitn(2, ' ').collect();
    let score = result[0].parse().unwrap();
    let name = result[1];
    (name, score)
}
