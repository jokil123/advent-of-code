mod inputs;

fn main() {
    // println!("{:?}", inputs::bingo_boards());

    let mut boards = inputs::bingo_boards()
        .iter()
        .map(|e| BingoBoard {
            board_values: *e,
            board_state: [[false; 5]; 5],
        })
        .collect::<Vec<BingoBoard>>();

    /*
    let mut boards = [BingoBoard {
        board_values: [
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
        ],
        board_state: [[false; 5]; 5],
    }]
    .try_into()
    .unwrap();

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();

    */

    let numbers = inputs::bingo_numbers();

    let mut games = play_games(&mut boards, &numbers);

    games.sort_by(|a, b| a.win_time.cmp(&b.win_time));

    games.iter().enumerate().for_each(|(i, g)| {
        println!(
            "game: {}, score: {}, win: {}, won in: {}",
            i, g.score, g.win, g.win_time
        )
    });
}

fn play_games(boards: &mut Vec<BingoBoard>, numbers: &Vec<i64>) -> Vec<BingoGame> {
    return boards
        .iter_mut()
        .map(|board| play_board(board, &numbers))
        .collect::<Vec<BingoGame>>();
}

fn play_board(board: &mut BingoBoard, numbers: &Vec<i64>) -> BingoGame {
    let mut game = BingoGame {
        win: false,
        score: 0,
        win_time: 0,
    };

    for (i, n) in numbers.iter().enumerate() {
        let result = locate_number(board, n);
        if result.on_board {
            result.positions.iter().for_each(|p| {
                board.board_state[p[0]][p[1]] = true;
            });
            game.win = check_bingo(&*board);

            if game.win {
                game.win_time = i;
                game.score = get_board_score(board, n);
                break;
            }
        }
    }

    return game;
}

fn get_board_score(board: &BingoBoard, last_draw: &i64) -> i64 {
    let mut score = 0;

    board.board_state.iter().enumerate().for_each(|(i, e)| {
        e.iter().enumerate().for_each(|(j, e)| {
            if !*e {
                score += board.board_values[i][j]
            }
        })
    });

    return score * last_draw;
}

fn check_bingo(board: &BingoBoard) -> bool {
    let mut is_winner: bool = false;

    board.board_state.iter().for_each(|state| {
        if state == &[true; 5] {
            is_winner = true;
        }
    });

    transpose_matrix(board.board_state)
        .iter()
        .for_each(|state| {
            if state == &[true; 5] {
                is_winner = true;
            }
        });

    return is_winner;
}

fn transpose_matrix<T: Copy, const X: usize, const Y: usize>(
    in_matrix: [[T; X]; Y],
) -> [[T; Y]; X] {
    // this might bite me in the back later
    let mut out_matrix: [[T; Y]; X] = [[in_matrix[0][0]; Y]; X];

    for i in 0..X {
        for j in 0..Y {
            out_matrix[i][j] = in_matrix[j][i];
        }
    }

    return out_matrix;
}

fn locate_number(board: &BingoBoard, number: &i64) -> NumberLocation {
    let mut location = NumberLocation {
        on_board: false,
        positions: Vec::new(),
    };

    board.board_values.iter().enumerate().for_each(|(i, e)| {
        e.iter().enumerate().for_each(|(j, e)| {
            if e == number {
                location.on_board = true;
                location.positions.push([i, j])
            }
        })
    });

    return location;
}

struct NumberLocation {
    on_board: bool,
    positions: Vec<[usize; 2]>,
}

struct BingoGame {
    win: bool,
    win_time: usize,
    score: i64,
}

struct BingoBoard {
    board_values: [[i64; 5]; 5],
    board_state: [[bool; 5]; 5],
}
