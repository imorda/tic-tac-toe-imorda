use std::io::BufRead;
use std::process::exit;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mark {
    X,
    O,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameState {
    Ongoing,
    Winner(Mark),
    Draw,
}

const BOARD_SIZE: usize = 3;
const VERTICAL_SEPARATOR: &str = "│";
const VERTICAL_BOLD_SEPARATOR: &str = "║";
const HORIZONTAL_BOLD_SEPARATOR: &str = "═";
const H_CROSS_BOLD_SEPARATOR: &str = "╪";
const CROSS_BOLD_SEPARATOR: &str = "╬";
const BOLD_T_RIGHT_SEPARATOR: &str = "╡";

#[derive(Clone)]
struct Board {
    data: [[Mark; BOARD_SIZE]; BOARD_SIZE],
}

fn print_repeated_pattern(value: &str, count: usize) {
    for _ in 0..count {
        print!("{value}");
    }
}

fn get_colored_mark(side_to_print: Mark, side_to_compare: Mark) -> String {
    let color_code = (if side_to_compare == Mark::None {
        "\x1b[1m"
    } else if side_to_compare == side_to_print {
        "\x1b[32;1m"
    } else {
        "\x1b[31;1m"
    })
    .to_string();

    match side_to_print {
        Mark::X => color_code + "X\x1b[0m",
        Mark::O => color_code + "0\x1b[0m",
        Mark::None => "\x1b[38;5;8m•\x1b[0m".to_string(),
    }
}

impl Board {
    fn new() -> Board {
        Board {
            data: [[Mark::None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn print_current_position(&self, side: Mark) {
        let cell_width = BOARD_SIZE.to_string().len();

        print_repeated_pattern(" ", cell_width);
        print!("{VERTICAL_BOLD_SEPARATOR}");
        for i in 0..BOARD_SIZE {
            let cur_number = i.to_string();
            print_repeated_pattern(" ", cell_width - cur_number.len());
            print!("{cur_number}{VERTICAL_SEPARATOR}");
        }
        println!();

        print_repeated_pattern(HORIZONTAL_BOLD_SEPARATOR, cell_width);
        print!("{CROSS_BOLD_SEPARATOR}");
        for r in 0..BOARD_SIZE {
            print_repeated_pattern(HORIZONTAL_BOLD_SEPARATOR, cell_width);
            if r < BOARD_SIZE - 1 {
                print!("{H_CROSS_BOLD_SEPARATOR}");
            } else {
                print!("{BOLD_T_RIGHT_SEPARATOR}");
            }
        }
        println!();

        for r in 0..BOARD_SIZE {
            let row_number = r.to_string();
            print_repeated_pattern(" ", cell_width - row_number.len());
            print!("{row_number}{VERTICAL_BOLD_SEPARATOR}");

            for c in 0..BOARD_SIZE {
                print_repeated_pattern(" ", cell_width - 1);

                print!(
                    "{}{VERTICAL_SEPARATOR}",
                    get_colored_mark(self.data[r][c], side)
                );
            }
            println!();
        }
    }

    fn count_equal_row_length(&self, x: usize, y: usize, dx: isize, dy: isize) -> usize {
        let initial = self.data[x][y];
        let mut cur_x = x as isize;
        let mut cur_y = y as isize;
        let mut ans: usize = 0;
        while (0..BOARD_SIZE as isize).contains(&cur_x) && (0..BOARD_SIZE as isize).contains(&cur_y)
        {
            if self.data[cur_x as usize][cur_y as usize] == initial {
                ans += 1;
                cur_x += dx;
                cur_y += dy;
            } else {
                break;
            }
        }
        ans
    }

    fn get_state(&self) -> GameState {
        let mut winner = Mark::None;
        for i in 0..BOARD_SIZE {
            if self.data[i][0] != Mark::None
                && self.count_equal_row_length(i, 0, 0, 1) == BOARD_SIZE
            {
                winner = self.data[i][0];
                break;
            }
            if self.data[0][i] != Mark::None
                && self.count_equal_row_length(0, i, 1, 0) == BOARD_SIZE
            {
                winner = self.data[0][i];
                break;
            }
        }
        if winner == Mark::None {
            if self.count_equal_row_length(0, 0, 1, 1) == BOARD_SIZE {
                winner = self.data[0][0];
            } else if self.count_equal_row_length(BOARD_SIZE - 1, 0, -1, 1) == BOARD_SIZE {
                winner = self.data[BOARD_SIZE - 1][0];
            } else if !self.data.iter().flatten().any(|&value| value == Mark::None) {
                return GameState::Draw;
            }
        }

        if winner == Mark::None {
            GameState::Ongoing
        } else {
            GameState::Winner(winner)
        }
    }

    fn make_turn_checked(&mut self, x: usize, y: usize, side: Mark) -> bool {
        if !((0..BOARD_SIZE).contains(&x) && (0..BOARD_SIZE).contains(&y)) {
            return false;
        }
        if self.data[x][y] != Mark::None {
            return false;
        }
        self.data[x][y] = side;
        true
    }
}

trait Player {
    fn turn(&self, board: &Board) -> Option<(usize, usize)>;
    fn get_mark(&self) -> Mark;
}

struct HumanPlayer {
    mark: Mark,
}

impl HumanPlayer {
    fn parse_pos(s: &str) -> Option<(usize, usize)> {
        let mut parts = s.split(' ');
        let row = parts.next()?.trim().parse().ok()?;
        let col = parts.next()?.trim().parse().ok()?;
        Some((row, col))
    }

    fn new(mark: Mark) -> Self {
        Self { mark }
    }
}

impl Player for HumanPlayer {
    fn turn(&self, board: &Board) -> Option<(usize, usize)> {
        loop {
            println!("Current board state:");
            board.print_current_position(self.mark);
            println!("Enter the next mark's position (row, column) space-separated");
            println!("Press ENTER to surrender");

            let mut stdin = std::io::stdin().lock();
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();

            if line.trim().is_empty() {
                return None;
            }

            if let Some((x, y)) = HumanPlayer::parse_pos(&line) {
                if (0..BOARD_SIZE).contains(&x)
                    && (0..BOARD_SIZE).contains(&y)
                    && board.data[x][y] == Mark::None
                {
                    return Some((x, y));
                } else {
                    println!("Entered position is invalid!");
                }
            }
        }
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameOutcome {
    Winning,
    Losing,
    Draw,
}

struct AIPlayer {
    mark: Mark,
}

impl AIPlayer {
    fn new(mark: Mark) -> Self {
        Self { mark }
    }

    fn find_best_move(board: &mut Board, side: Mark) -> ((usize, usize), GameOutcome) {
        let mut best_move: (usize, usize) = (0, 0);
        let mut best_outcome = GameOutcome::Losing;

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if board.data[i][j] == Mark::None {
                    board.data[i][j] = side;

                    match board.get_state() {
                        GameState::Winner(mark) => {
                            if mark == side {
                                board.data[i][j] = Mark::None;
                                return ((i, j), GameOutcome::Winning);
                            } else {
                            }
                        }
                        GameState::Draw => {
                            best_move = (i, j);
                            best_outcome = GameOutcome::Draw;
                        }
                        GameState::Ongoing => {
                            let (_, result) = Self::find_best_move(
                                board,
                                if side == Mark::X { Mark::O } else { Mark::X },
                            );

                            match result {
                                GameOutcome::Losing => {
                                    board.data[i][j] = Mark::None;
                                    return ((i, j), GameOutcome::Winning);
                                }
                                GameOutcome::Draw => {
                                    best_move = (i, j);
                                    best_outcome = GameOutcome::Draw;
                                }
                                GameOutcome::Winning => {}
                            }
                        }
                    }
                    board.data[i][j] = Mark::None;
                }
            }
        }
        (best_move, best_outcome)
    }
}

impl Player for AIPlayer {
    fn turn(&self, board: &Board) -> Option<(usize, usize)> {
        let mut own_board = board.clone();
        let (coords, outcome) = AIPlayer::find_best_move(&mut own_board, self.mark);
        if outcome == GameOutcome::Losing {
            None
        } else {
            Some(coords)
        }
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}

fn get_human_side() -> Mark {
    let mut stdin = std::io::stdin().lock();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        match line.trim() {
            "0" => return Mark::O,
            "O" => return Mark::O,
            "X" => return Mark::X,
            _ => {
                println!("Wrong input format! Try again");
            }
        }
    }
}

fn main() {
    let mut board = Board::new();

    println!("Choose your side (write 'X' or '0')");
    let human_side = get_human_side();

    let human = HumanPlayer::new(human_side);
    let ai = AIPlayer::new(if human_side == Mark::X {
        Mark::O
    } else {
        Mark::X
    });

    let mut cur_side: &dyn Player = if human_side == Mark::X { &human } else { &ai };

    while board.get_state() == GameState::Ongoing {
        println!("Current board state:");
        board.print_current_position(Mark::None);
        println!(
            "Now it is {}'s turn",
            get_colored_mark(cur_side.get_mark(), human_side)
        );

        match cur_side.turn(&board) {
            None => {
                println!(
                    "{} was unable to make a turn, assuming it has surrendered!",
                    get_colored_mark(cur_side.get_mark(), human_side)
                );
                exit(0);
            }
            Some((x, y)) => {
                if !board.make_turn_checked(x, y, cur_side.get_mark()) {
                    println!(
                        "{} just made an illegal move! BANNED!",
                        get_colored_mark(cur_side.get_mark(), human_side)
                    );
                    exit(0);
                }

                if cur_side.get_mark() == human.get_mark() {
                    cur_side = &ai;
                } else {
                    cur_side = &human;
                }
            }
        }
    }
    println!(
        "GAME OVER! The result is {}!",
        match board.get_state() {
            GameState::Ongoing => {
                "interrupted".to_string()
            }
            GameState::Winner(mark) => {
                format!("{:?} is a winner", mark)
            }
            GameState::Draw => {
                "Draw".to_string()
            }
        }
    );
    println!("Final board:");
    board.print_current_position(Mark::None);
}
