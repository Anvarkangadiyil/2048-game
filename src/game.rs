use std::io;

use crossterm::{
    cursor::{Hide, Show, self, },
    execute,
    style::{Color as TermColor, Print, ResetColor, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use rand::{Rng, seq::SliceRandom};

use super::display;



#[derive(Clone, Copy)]
 pub struct Game{
   pub  board:[[u32;4];4],
   pub score:i32,  
}
impl Game {

    //function to initiate Game 
    pub fn new() -> Game {
        Game {
            board:[[0;4];4],
            score:0,
        }
    }

   


    pub fn display(&mut self) {
       
       
        self.random_number_generator();


        // Define the cell size
        let cell_width = 10;
        

        // Initialize the terminal and hide the cursor
        let mut stdout = io::stdout();
        execute!(stdout, Hide).unwrap();


        // to Clear the terminal
        execute!(stdout, Clear(ClearType::All)).unwrap();

        //to place cusor in top  of terminal
        execute!(stdout, cursor::MoveTo(0,0)).unwrap();
            
        display::to_display_score(self.score);

        // Iterate over each row in the game board
        for row in &self.board {
          
        
            // Iterate over each number in the row
            for number in row {
                // Set the background color based on the number
                let background_color = match number {
                    0 =>TermColor::DarkGrey,
                    2 =>TermColor::DarkRed,
                    4 =>TermColor::Green,
                    8 =>TermColor::DarkBlue,
                    16=>TermColor::DarkCyan,
                    32=>TermColor::Magenta,
                    64=>TermColor::DarkYellow,
                    128=>TermColor::DarkRed,
                    256=>TermColor::DarkGreen,
                    2048=>TermColor::Cyan,
                    _ => TermColor::DarkMagenta,
                };


                // Set the background color and print the number
                execute!(
                    stdout,
                    SetBackgroundColor(background_color),
                    Print(format!("{:^1$}", number, cell_width - 2))
                )
                .unwrap();

              

                // Reset the background color
                execute!(stdout, ResetColor).unwrap();
            }
            
            // Move the cursor to the next line
            execute!(stdout, Print("\n\r")).unwrap();
        }
         
        display::footer();
       
        // Show the cursor
        execute!(stdout, Show).unwrap();
    }
    
    //to generate random number in matrix
   pub  fn random_number_generator(self: &mut Self) {
        let mut rng = rand::thread_rng();
     
        let mut empty_positions = vec![];
    
        // Find all empty positions in the matrix
        for row in 0..4 {
            for col in 0..4 {
                if self.board[row][col]==0{
                    empty_positions.push((row, col));
                }
            }
        }
    
        // Generate a random index from the empty positions
        if let Some((row, col)) = empty_positions.choose(&mut rng) {
            let number = if rng.gen_ratio(1, 2) { 2 } else { 4 };
            self.board[*row][*col] = number;
        }
        
    }
   

   

}