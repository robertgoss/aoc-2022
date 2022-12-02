#[derive(Copy, Clone, Debug)]
enum State{
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone, Debug)]
enum Result{
    Draw,
    Win,
    Loss
}

struct Turn{
    opp : State,
    own : State
}

struct TurnResult{
    opp : State,
    res : Result
}

pub struct Game {
    turns : Vec<Turn>
}

pub struct Guide {
    turns : Vec<TurnResult>
}

impl State {
    fn from_string(string : &str) -> Option<State> {
        match string {
            "A" => Some(State::Rock),
            "B" => Some(State::Paper),
            "C" => Some(State::Scissors),
            "X" => Some(State::Rock),
            "Y" => Some(State::Paper),
            "Z" => Some(State::Scissors),
            _ => None
        }
    }

    fn score(&self) -> usize {
        match self {
            State::Rock => 1,
            State::Paper => 2,
            State::Scissors => 3
        }
    }
}

impl Result {
    fn from_string(string : &str) -> Option<Result> {
        match string {
            "Y" => Some(Result::Draw),
            "Z" => Some(Result::Win),
            "X" => Some(Result::Loss),
            _ => None
        }
    }

    fn score(&self) -> usize {
        match self {
            Result::Draw => 3,
            Result::Win => 6,
            Result::Loss => 0
        }
    }
}

impl Turn {
  fn from_line(line : &str) -> Option<Turn> {
    match line.split_once(" ") {
        Some((opp_str, self_str)) => {
            match (State::from_string(opp_str), State::from_string(self_str)) {
                (Some(opp), Some(self_state)) => Some(
                    Turn {opp : opp, own :self_state}
                ),
                _ => None
            }
        },
        _ => None
    }
  }

  fn score(&self) -> usize {
    self.own.score() + self.outcome().score()
  }

  fn outcome(&self) -> Result {
    match (self.own, self.opp) {
        (State::Rock, State::Rock) => Result::Draw,
        (State::Rock, State::Paper) => Result::Loss,
        (State::Rock, State::Scissors) => Result::Win,
        (State::Paper, State::Rock) => Result::Win,
        (State::Paper, State::Paper) => Result::Draw,
        (State::Paper, State::Scissors) => Result::Loss,
        (State::Scissors, State::Rock) => Result::Loss,
        (State::Scissors, State::Paper) => Result::Win,
        (State::Scissors, State::Scissors) => Result::Draw
    }
  }
}

impl TurnResult {
    fn from_line(line : &str) -> Option<TurnResult> {
      match line.split_once(" ") {
          Some((opp_str, res_str)) => {
              match (State::from_string(opp_str), Result::from_string(res_str)) {
                  (Some(opp), Some(res)) => Some(
                      TurnResult {opp : opp, res :res}
                  ),
                  _ => None
              }
          },
          _ => None
      }
    }

    fn turn(&self) -> Turn {
        let own = match (self.opp, self.res) {
            (State::Rock, Result::Draw) => State::Rock,
            (State::Rock, Result::Win) => State::Paper,
            (State::Rock, Result::Loss) => State::Scissors,
            (State::Paper, Result::Draw) => State::Paper,
            (State::Paper, Result::Win) => State::Scissors,
            (State::Paper, Result::Loss) => State::Rock,
            (State::Scissors, Result::Draw) => State::Scissors,
            (State::Scissors, Result::Win) => State::Rock,
            (State::Scissors, Result::Loss) => State::Paper
        };
        Turn {opp : self.opp, own : own}
      }
}

impl Game {
    pub fn from_lines(lines : &Vec<String>) -> Game {
        let turns = lines.iter().filter_map(
            |line| Turn::from_line(line)
        ).collect();
        Game { turns : turns }
    }

    pub fn score(&self) -> usize {
        self.turns.iter().map(
            |turn| turn.score()
        ).sum()
    }
}

impl Guide {
    pub fn from_lines(lines : &Vec<String>) -> Guide {
        let turns = lines.iter().filter_map(
            |line| TurnResult::from_line(line)
        ).collect();
        Guide { turns : turns }
    }

    pub fn score(&self) -> usize {
        self.game().score()
    }

    fn game(&self) -> Game {
        let turns = self.turns.iter().map(
            |turn_result| turn_result.turn()
        ).collect();
        Game {turns : turns}
    }
}