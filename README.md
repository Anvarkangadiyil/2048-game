
# 2048-game
This is a Rust implementation of the [2048 game](https://en.wikipedia.org/wiki/2048_\(video_game\))  which runs in the terminal.  

## Requirements

- [`Cargo`](https://www.rust-lang.org/tools/install)
- [`Rust`](https://www.rust-lang.org/tools/install)

## Supported Systems

This game is designed to run on the following operating systems:

- Windows (7, 8, 10)
- supports all UNIX

## Installation 
 Open command prompt or windows powershell:
```bash
> git clone https://github.com/Anvarkangadiyil/2048-game.git
> cd  2048-game
> cargo build --release
> cargo run --release

```


## How to play  

1. The game board will be displayed in the console, showing a 4x4 grid of numbered tiles.
2. Use the arrow keys (up, down, left, right) or w,s,a,d to move the tiles in the corresponding direction.
3. Tiles with the same number will merge when they collide, doubling their value.
4. After each move, a new tile with a value of 2 or 4 will appear on the board.
5. Your goal is to reach the 2048 tile by merging tiles. The game ends when you cannot make any more moves.


## Screenshot 

 <table background-color="white">
     <tr>
       <th>
             <img src="https://github.com/Anvarkangadiyil/2048-game/blob/main/screenshot/before_lose.png" alt="before lose game">
       </th>
       <th>
             <img src="https://github.com/Anvarkangadiyil/2048-game/blob/main/screenshot/after_lose.png" alt="lose game">
       </td>
     </tr>
   </table>
  


## Contributing
Contributions are welcome! If you would like to contribute to this project, please follow these steps:  

1. Fork the repository.  
2. Create a new branch for your feature or bug fix.  
3. Make your changes and commit them with descriptive commit messages.  
4. Push your changes to your forked repository.  
5. Submit a pull request to the master branch of the original repository.  
6. Please ensure that your code follows the existing code style and includes appropriate tests.



 
