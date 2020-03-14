use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Game<'a> {
    team1: &'a str,
    team1_score: u8,
    team2: &'a str,
    team2_score: u8,
}

pub fn parse_game<'a>(line: &'a str) -> Game<'a> {
    let teams: Vec<&'a str> = line.split(", ").collect();
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

pub fn get_scores<'a>(games: &'a [Game<'a>]) -> HashMap<&'a str, u8> {
    let mut scores = DefaultHashMap::new(0);
    for game in games {
        match get_winner(game) {
            Some(result) => {
                scores[result.winner] += 3;
                scores[result.loser] += 0; // initialize if not already initialized
            },
            None => {
                scores[game.team1] += 1;
                scores[game.team2] += 1;
            }
        }
    }
    scores.into()
}

#[derive(Debug, PartialEq)]
struct GameResult<'a> {
    winner: &'a str,
    loser: &'a str,
}

fn get_winner<'a>(game: &'a Game<'a>) -> Option<GameResult<'a>> {
    if game.team1_score > game.team2_score {
        Some(GameResult { winner: game.team1, loser: game.team2 })
    } else if game.team1_score < game.team2_score {
        Some(GameResult { winner: game.team2, loser: game.team1 })
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct TeamRank<'a> {
    rank: u8,
    name: &'a str,
    score: u8,
}

pub fn get_rankings<'a>(rankings: &HashMap<&'a str, u8>) -> Vec<TeamRank<'a>> {
    rankings
        .iter()
        .sorted_by_key(|(&name, &score)| (-1 * score as i16, name))
        .enumerate()
        .map(|(i, (&name, &score))| TeamRank { rank: i as u8 + 1, name, score })
        .group_by(|team| team.score)
        .into_iter()
        .map(|(_, tied_teams)|
            tied_teams
                .enumerate()
                .map(|(i, team)| TeamRank { rank: team.rank - i as u8, ..team })
        )
        .flatten()
        .collect()
}

pub fn display_rank<'a>(team: &TeamRank<'a>) -> String {
    let label = if team.score == 1 { "pt" } else { "pts" };
    format!("{}. {}, {} {}", team.rank, team.name, team.score, label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_team() {
        assert_eq!(parse_team("Team1 10"), ("Team1", 10));
        assert_eq!(parse_team("Hello world 10"), ("Hello world", 10));
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_game("Team1 10, Hello world 20"),
            Game { team1: "Team1", team1_score: 10, team2: "Hello world", team2_score: 20 }
        );
    }

    #[test]
    fn test_get_winner() {
        let game1 = Game { team1: "A", team1_score: 10, team2: "B", team2_score: 20 };
        assert_eq!(get_winner(&game1), Some(GameResult { winner: "B", loser: "A" }));
        let game2 = Game { team1: "A", team1_score: 10, team2: "B", team2_score: 10 };
        assert_eq!(get_winner(&game2), None);
    }

    #[test]
    fn test_get_scores() {
        let games = vec![
            Game { team1: "A", team1_score: 10, team2: "B", team2_score: 20 },
            Game { team1: "C", team1_score: 10, team2: "D", team2_score: 10 },
            Game { team1: "A", team1_score: 20, team2: "D", team2_score: 10 },
            Game { team1: "B", team1_score: 20, team2: "C", team2_score: 10 },
            Game { team1: "C", team1_score: 20, team2: "E", team2_score: 10 },
        ];
        let scores = get_scores(games.as_slice());

        assert_eq!(scores["A"], 3);
        assert_eq!(scores["B"], 6);
        assert_eq!(scores["C"], 4);
        assert_eq!(scores["D"], 1);
        assert_eq!(scores["E"], 0);
    }

    #[test]
    fn test_get_rankings() {
        let scores = [
            ("D", 10),
            ("A", 10),
            ("B", 20),
            ("C", 15),
            ("E", 0),
        ].iter().cloned().collect();
        assert_eq!(get_rankings(&scores), vec![
            TeamRank { rank: 1, name: "B", score: 20 },
            TeamRank { rank: 2, name: "C", score: 15 },
            TeamRank { rank: 3, name: "A", score: 10 },
            TeamRank { rank: 3, name: "D", score: 10 },
            TeamRank { rank: 5, name: "E", score: 0 },
        ]);
    }

    #[test]
    fn test_display_rank() {
        let rank = TeamRank { rank: 1, name: "Team 1", score: 0 };
        assert_eq!(display_rank(&rank), "1. Team 1, 0 pts");

        let rank = TeamRank { rank: 1, name: "My Team", score: 1 };
        assert_eq!(display_rank(&rank), "1. My Team, 1 pt");

        let rank = TeamRank { rank: 2, name: "My Other Team", score: 2 };
        assert_eq!(display_rank(&rank), "2. My Other Team, 2 pts");
    }
}
