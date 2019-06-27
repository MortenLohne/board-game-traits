# Overview

A collection of minimal traits for abstractly representing 2-player board games. The traits can be implemented for any sequential, deterministic, perfect-information game. This includes many popular games such as chess, go, xiangqi, othello, connect four and tic-tac-toe.

No implementations are provided in this crate.

# Usage examples

Count the total number of moves possible at n moves deep. Commonly called a [perf test](https://www.chessprogramming.org/Perft) and used to check the correctness of move generation: 

````
pub fn perft<B: Board>(board: &mut B, depth: u16) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut moves = vec![];
        board.generate_moves(&mut moves);
        moves
            .into_iter()
            .map(|mv| {
                let reverse_move = board.do_move(mv);
                let num_moves = perft(board, depth - 1);
                board.reverse_move(reverse_move);
                num_moves
            })
            .sum()
    }
}
````

The [Minimax algorithm](https://en.wikipedia.org/wiki/Minimax), running n plies ahead:

````
fn minimax<B: EvalBoard>(board: &mut B, depth: u16) -> f32 {
    match board.game_result() {
        Some(GameResult::WhiteWin) => return 100.0,
        Some(GameResult::BlackWin) => return -100.0,
        Some(GameResult::Draw) => return 0.0,
        None => (),
    }
    if depth == 0 {
        board.static_eval()
    } else {
        let side_to_move = board.side_to_move();
        let mut moves = vec![];
        board.generate_moves(&mut moves);
        let child_evaluations = moves.into_iter().map(|mv| {
            let reverse_move = board.do_move(mv);
            let eval = minimax(board, depth - 1);
            board.reverse_move(reverse_move);
            eval
        });
        match side_to_move {
            Color::White => child_evaluations
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            Color::Black => child_evaluations
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
        }
    }
}

````