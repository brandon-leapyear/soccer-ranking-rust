use defaultmap::DefaultHashMap;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
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

pub fn get_rankings<'a>(games: &'a [Game<'a>]) -> HashMap<&'a str, u8> {
    let mut rankings = DefaultHashMap::new(0);
    for game in games {
        match get_winner(game) {
            Some(result) => {
                rankings[result.winner] += 3;
                rankings[result.loser] += 0; // initialize if not already initialized
            },
            None => {
                rankings[game.team1] += 1;
                rankings[game.team2] += 1;
            }
        }
    }
    rankings.into()
}

#[derive(Debug)]
#[derive(PartialEq)]
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
    fn test_get_rankings() {
        let games = vec![
            Game { team1: "A", team1_score: 10, team2: "B", team2_score: 20 },
            Game { team1: "C", team1_score: 10, team2: "D", team2_score: 10 },
            Game { team1: "A", team1_score: 20, team2: "D", team2_score: 10 },
            Game { team1: "B", team1_score: 20, team2: "C", team2_score: 10 },
            Game { team1: "C", team1_score: 20, team2: "E", team2_score: 10 },
        ];
        let rankings = get_rankings(games.as_slice());

        assert_eq!(rankings["A"], 3);
        assert_eq!(rankings["B"], 6);
        assert_eq!(rankings["C"], 4);
        assert_eq!(rankings["D"], 1);
        assert_eq!(rankings["E"], 0);
    }
}
