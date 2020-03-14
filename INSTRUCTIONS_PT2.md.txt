## The Problem

The first version of the soccer ranking challenge was a success, but some teams complained that, in
the case of a tie, they were unfairly ordered below other teams based only on the alphabetical order of 
their names and not the number of goals each team scored, or goals they allowed to be scored against them.
All teams agreed that it would be more fair if there were a tie breaker that took the season's goals
scored into account first, so there could only be a tie if both the "league points" and the goals were
the same.

Update the soccer league ranker to implement this tie breaker, which we'll call "goal differential".

The goal differential for each team is defined as the total number of goals scored by that team, minus
the number of goals scored against that team, in all games in the season. Where there is a tie in league
points, break that tie by assigning the higher (eg, lower-ordinal) league rank to the team with the higher
goal differential for the season. If there is still a tie between both league points and goal differential,
then the assigned rank should be the same, and the teams should be sorted alphabetically, as before.

Output the end-of-season goal differential for each team to exactly match the contents of
[expected-output-goal-differential.txt](https://github.com/LeapYear/soccer-ranking-project/blob/master/expected-output-goal-differential.txt).
