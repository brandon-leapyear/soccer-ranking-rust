use defaultmap::DefaultHashMap;

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

pub fn get_rankings<'a>(games: &'a [Game<'a>]) -> DefaultHashMap<&'a str, u8> {
    let mut rankings = DefaultHashMap::new(0);
    for game in games {
        match get_winner(game) {
            Some(winner) => {
                rankings[winner] += 3;
            },
            None => {
                rankings[game.team1] += 1;
                rankings[game.team2] += 1;
            }
        }
    }
    rankings
}

fn get_winner<'a>(game: &'a Game) -> Option<&'a str> {
    if game.team1_score > game.team2_score {
        Some(game.team1)
    } else if game.team1_score < game.team2_score {
        Some(game.team2)
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
        assert_eq!(get_winner(&game1), Some("B"));
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
        ];
        let rankings = get_rankings(games.as_slice());

        assert_eq!(rankings["A"], 3);
        assert_eq!(rankings["B"], 6);
        assert_eq!(rankings["C"], 1);
        assert_eq!(rankings["D"], 1);
    }
}
