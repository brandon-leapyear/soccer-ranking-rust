use defaultmap::DefaultHashMap;
use itertools::Itertools;
use std::cmp::Reverse;
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

#[derive(Clone, Debug, PartialEq)]
pub struct TeamScores {
    league_score: u8,
    goal_diff: i32,
}

pub fn get_scores<'a>(games: &'a [Game<'a>]) -> HashMap<&'a str, TeamScores> {
    let mut scores = DefaultHashMap::new(TeamScores { league_score: 0, goal_diff: 0 });

    for game in games {
        match get_winner(game) {
            Some(result) => {
                let winner = &mut scores[result.winner];
                winner.league_score += 3;
                winner.goal_diff += result.goal_diff;

                let loser = &mut scores[result.loser];
                loser.goal_diff -= result.goal_diff;
            },
            None => {
                scores[game.team1].league_score += 1;
                scores[game.team2].league_score += 1;
            }
        }
    }

    scores.into()
}

#[derive(Debug, PartialEq)]
struct GameResult<'a> {
    winner: &'a str,
    loser: &'a str,
    goal_diff: i32,
}

fn get_winner<'a>(game: &'a Game<'a>) -> Option<GameResult<'a>> {
    let goal_diff = game.team1_score as i32 - game.team2_score as i32;
    if goal_diff > 0 {
        Some(GameResult { winner: game.team1, loser: game.team2, goal_diff })
    } else if goal_diff < 0 {
        Some(GameResult { winner: game.team2, loser: game.team1, goal_diff: -goal_diff })
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub struct TeamRank<'a> {
    rank: u8,
    name: &'a str,
    score: u8,
    goal_diff: i32,
}

pub enum RankStrategy {
    // Rank by score
    Score,
    // Rank by score, then goal differential
    ScoreThenGoalDiff,
}

pub fn get_rankings<'a>(rankings: &HashMap<&'a str, TeamScores>, rank_strategy: RankStrategy) -> Vec<TeamRank<'a>> {
    let get_score = |scores: &TeamScores| {
        match rank_strategy {
            RankStrategy::Score => (scores.league_score, 0),
            RankStrategy::ScoreThenGoalDiff => (scores.league_score, scores.goal_diff),
        }
    };

    let sorted_rankings = rankings
        .iter()
        .sorted_by_key(|(&name, scores)| (Reverse(get_score(scores)), name));

    let mut length = 0;
    let mut result = vec![];

    for (_, tied_teams) in sorted_rankings.group_by(|(_, scores)| get_score(scores)).into_iter() {
        for (i, team) in tied_teams.enumerate() {
            let (&name, scores) = team;
            result.push(TeamRank {
                rank: (length - i + 1) as u8,
                name,
                score: scores.league_score,
                goal_diff: scores.goal_diff,
            });
            length += 1;
        }
    }

    result
}

pub fn display_rank_pt1<'a>(team: &TeamRank<'a>) -> String {
    let label = if team.score == 1 { "pt" } else { "pts" };
    format!("{}. {}, {} {}", team.rank, team.name, team.score, label)
}

pub fn display_rank_pt2<'a>(team: &TeamRank<'a>) -> String {
    let label = if team.score == 1 { "pt" } else { "pts" };
    format!("{}. {}, {} {}, gd: {}", team.rank, team.name, team.score, label, team.goal_diff)
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
        assert_eq!(get_winner(&game1), Some(GameResult { winner: "B", loser: "A", goal_diff: 10 }));
        let game1 = Game { team1: "A", team1_score: 20, team2: "B", team2_score: 0 };
        assert_eq!(get_winner(&game1), Some(GameResult { winner: "A", loser: "B", goal_diff: 20 }));
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

        assert_eq!(scores["A"], TeamScores { league_score: 3, goal_diff: 0 });
        assert_eq!(scores["B"], TeamScores { league_score: 6, goal_diff: 20 });
        assert_eq!(scores["C"], TeamScores { league_score: 4, goal_diff: 0 });
        assert_eq!(scores["D"], TeamScores { league_score: 1, goal_diff: -10 });
        assert_eq!(scores["E"], TeamScores { league_score: 0, goal_diff: -10 });
    }

    #[test]
    fn test_get_rankings_pt1() {
        let scores = [
            ("D", TeamScores { league_score: 10, goal_diff: 0 }),
            ("A", TeamScores { league_score: 10, goal_diff: 0 }),
            ("B", TeamScores { league_score: 20, goal_diff: 0 }),
            ("C", TeamScores { league_score: 15, goal_diff: 0 }),
            ("E", TeamScores { league_score: 0, goal_diff: 0 }),
        ].iter().cloned().collect();

        assert_eq!(get_rankings(&scores, RankStrategy::Score), vec![
            TeamRank { rank: 1, name: "B", score: 20, goal_diff: 0 },
            TeamRank { rank: 2, name: "C", score: 15, goal_diff: 0 },
            TeamRank { rank: 3, name: "A", score: 10, goal_diff: 0 },
            TeamRank { rank: 3, name: "D", score: 10, goal_diff: 0 },
            TeamRank { rank: 5, name: "E", score: 0, goal_diff: 0 },
        ]);
    }

    #[test]
    fn test_get_rankings_pt2() {
        let scores = [
            ("D", TeamScores { league_score: 10, goal_diff: 0 }),
            ("A", TeamScores { league_score: 10, goal_diff: 0 }),
            ("B", TeamScores { league_score: 10, goal_diff: 10 }),
            ("C", TeamScores { league_score: 15, goal_diff: 0 }),
            ("E", TeamScores { league_score: 0, goal_diff: 0 }),
        ].iter().cloned().collect();

        assert_eq!(get_rankings(&scores, RankStrategy::ScoreThenGoalDiff), vec![
            TeamRank { rank: 1, name: "C", score: 15, goal_diff: 0 },
            TeamRank { rank: 2, name: "B", score: 10, goal_diff: 10 },
            TeamRank { rank: 3, name: "A", score: 10, goal_diff: 0 },
            TeamRank { rank: 3, name: "D", score: 10, goal_diff: 0 },
            TeamRank { rank: 5, name: "E", score: 0, goal_diff: 0 },
        ]);
    }

    #[test]
    fn test_display_rank_pt1() {
        let rank = TeamRank { rank: 1, name: "Team 1", score: 0, goal_diff: 0 };
        assert_eq!(display_rank_pt1(&rank), "1. Team 1, 0 pts");

        let rank = TeamRank { rank: 1, name: "My Team", score: 1, goal_diff: 0 };
        assert_eq!(display_rank_pt1(&rank), "1. My Team, 1 pt");

        let rank = TeamRank { rank: 2, name: "My Other Team", score: 2, goal_diff: 0 };
        assert_eq!(display_rank_pt1(&rank), "2. My Other Team, 2 pts");
    }
}
