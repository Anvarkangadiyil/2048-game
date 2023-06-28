use super::game;
use crossterm::event::KeyCode;
use game::Game;

//action when left key is clicked

pub fn left(game: &mut Game) {
    for row in 0..4 {
        let mut merged = [false; 4];
        for col in 1..4 {
            if game.board[row][col] != 0 {
                let mut merge_idx = col;
                while merge_idx > 0 && game.board[row][merge_idx - 1] == 0 {
                    game.board[row][merge_idx - 1] = game.board[row][merge_idx];
                    game.board[row][merge_idx] = 0;
                    merge_idx -= 1;
                }
                if merge_idx > 0
                    && game.board[row][merge_idx - 1] == game.board[row][merge_idx]
                    && !merged[merge_idx - 1]
                {
                    game.board[row][merge_idx - 1] *= 2;
                    game.board[row][merge_idx] = 0;
                    game.score += game.board[row][merge_idx - 1] as i32;
                    merged[merge_idx - 1] = true;
                }
            }
        }
    }

    game.display();
}

//action when right key arrow 

pub fn right(game: &mut Game) {
    for row in 0..4 {
        let mut merged = [false; 4];
        for col in (0..3).rev() {
            if game.board[row][col] != 0 {
                let mut merge_idx = col;
                while merge_idx < 3 && game.board[row][merge_idx + 1] == 0 {
                    game.board[row][merge_idx + 1] = game.board[row][merge_idx];
                    game.board[row][merge_idx] = 0;
                    merge_idx += 1;
                }
                if merge_idx < 3
                    && game.board[row][merge_idx + 1] == game.board[row][merge_idx]
                    && !merged[merge_idx + 1]
                {
                    game.board[row][merge_idx + 1] *= 2;
                    game.board[row][merge_idx] = 0;
                    game.score += game.board[row][merge_idx + 1] as i32;
                    merged[merge_idx + 1] = true;
                }
            }
        }
    }

    game.display();
}

//action when up key is clicked

pub fn up(game: &mut Game) {
    for col in 0..4 {
        let mut merged = [false; 4];
        for row in 1..4 {
            if game.board[row][col] != 0 {
                let mut merge_idx = row;
                while merge_idx > 0 && game.board[merge_idx - 1][col] == 0 {
                    game.board[merge_idx - 1][col] = game.board[merge_idx][col];
                    game.board[merge_idx][col] = 0;
                    merge_idx -= 1;
                }
                if merge_idx > 0
                    && game.board[merge_idx - 1][col] == game.board[merge_idx][col]
                    && !merged[merge_idx - 1]
                {
                    game.board[merge_idx - 1][col] *= 2;
                    game.board[merge_idx][col] = 0;
                    game.score += game.board[merge_idx - 1][col] as i32;
                    merged[merge_idx - 1] = true;
                }
            }
        }
    }

    game.display();
}

//action when down key is clicked
pub fn down(game: &mut Game) {
    for col in 0..4 {
        let mut merged = [false; 4];
        for row in (0..3).rev() {
            if game.board[row][col] != 0 {
                let mut merge_idx = row;
                while merge_idx < 3 && game.board[merge_idx + 1][col] == 0 {
                    game.board[merge_idx + 1][col] = game.board[merge_idx][col];
                    game.board[merge_idx][col] = 0;
                    merge_idx += 1;
                }
                if merge_idx < 3
                    && game.board[merge_idx + 1][col] == game.board[merge_idx][col]
                    && !merged[merge_idx + 1]
                {
                    game.board[merge_idx + 1][col] *= 2;
                    game.board[merge_idx][col] = 0;
                    game.score += game.board[merge_idx + 1][col] as i32;
                    merged[merge_idx + 1] = true;
                }
            }
        }
    }

    game.display();
}

//fuction to check game won or not
pub fn has_won(game: &mut Game) -> bool {
    for row in 0..4 {
        for col in 0..4 {
            if game.board[row][col] == 2048 {
                return true;
            }
        }
    }
    false
}

//function to check game lose or not
pub fn has_defeat(game: &mut Game) -> bool {
    for row in 0..4 {
        for col in 0..4 {
            if game.board[row][col] == 0 {
                return false;
            }
            if row < 3 && game.board[row][col] == game.board[row + 1][col] {
                return false;
            }
            if col < 3 && game.board[row][col] == game.board[row][col + 1] {
                return false;
            }
        }
    }
    true
}

//function that decide which action fuction to be called based on key 
pub fn movement(game: &mut Game, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('w') => up(game),
        KeyCode::Left | KeyCode::Char('a') => left(game),
        KeyCode::Right | KeyCode::Char('d') => right(game),
        KeyCode::Down | KeyCode::Char('s') => down(game),
        _ => println!(""),
    }
}
