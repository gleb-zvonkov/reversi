//Gleb Zvonkov
//October 4, 2024
//ECE1724

use std::io;
const DIM: usize = 8; //the demension of the board
                      //make 8 a constant

//Intializes the board as requird in the assignment specifications
//no input parameters
//returns an 8x8 2D array of characters
fn initialize_board() -> [[char; DIM]; DIM] {
    let mut board = [['.'; DIM]; DIM]; // Create a mutable DIMxDIM board, cells are intialized with "."
    board[3][3] = 'W'; // Set initial pieces on the board
    board[3][4] = 'B';
    board[4][3] = 'B';
    board[4][4] = 'W';
    board //return the board
}

//Print the board
//input paramets is DIMxDIM character array
//returns nothing
fn print_board(board: [[char; DIM]; DIM]) {
    println!("  abcdefgh"); // Print column labels
    for row in 0..DIM {
        print!("{} ", (b'a' + row as u8) as char); //print row labels
        for col in 0..DIM {
            //get each cell from each row
            print!("{}", board[row][col]);
        }
        println!(); // print a new line, so next row starts on new line
    }
}

//allow the user to input row/column name
//parameters are two characters
//returns tuple containing row and column number
fn get_move_input() -> (usize, usize, bool) {
    let mut input = String::new(); // Create a mutable string variable to store the player's input.
    io::stdin() // Read a line from standard input
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim(); // Remove leading and trailing whitespace from input string
    if trimmed.len() != 2 {
        return (0, 0, false); // Return false for invalid input format
    }
    let row_char = trimmed.chars().nth(0).unwrap(); // Extract the first character of the input string
    let col_char = trimmed.chars().nth(1).unwrap(); // Extract the second character of the input string
    if !row_char.is_alphabetic()    //check if it is a character
        || !col_char.is_alphabetic()
        || row_char < 'a'  //if the character is smaller than a
        || row_char > 'h'  //if the character is greater than h
        || col_char < 'a'
        || col_char > 'h'
    {
        return (0, 0, false); // Return None for invalid characters
    }
    let row_idx = row_char as u8 - b'a'; // Convert row character to index
    let col_idx = col_char as u8 - b'a'; // Convert column character to index
    (row_idx as usize, col_idx as usize, true) // Return valid indices and true for success
}

//attempt the inputted move
//from the selected cell
//the first loop look into all possible direction
//the second loop check if there is an opponent piece followed by the players peice in that direction
//parameters are the mutable board, the row and column of the move, and the player character
//output is true if the move is succesful
fn attempt_move(
    board: [[char; DIM]; DIM],
    row: usize,
    col: usize,
    player: char,
) -> (bool, Vec<(usize, usize)>) {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]; //all directions we need to check, all combinations of 0,1,-1
    let opponent = if player == 'B' { 'W' } else { 'B' }; // determine the opponent
    let mut valid_move_found = false; //intialize valid move to false
    let mut final_flip_cells: Vec<(usize, usize)> = Vec::new(); //the cells we need to flip
    if board[row][col] != '.' {
        //if cells not empty return false
        return (false, Vec::new());
    }

    for (row_offset, col_offset) in directions.iter() {
        //search all possible directions
        let mut r = row as isize; //convert them into signed integers because may be -1
        let mut c = col as isize;
        let mut found_opponent_piece = false; // flag to track if we found an opponent's piece
        let mut temp_flip_cells = Vec::new(); //keep track of the cells we would flip if its a valid move
                                              // Peices we would flip if that move was valid
        loop {
            //move in the offset direction
            r += row_offset; //add the row offset to where player is attempting to play move
            c += col_offset;
            if r < 0 || r as usize >= DIM || c < 0 || c as usize >= DIM {
                //If its out of bounds, wont be a valid move in that direction
                break; // break out of checking that offset direciton
            }
            match board[r as usize][c as usize] {
                //use a match to deterime if that cell is players or opponents
                p if p == opponent => {
                    //if its an opponents piece
                    found_opponent_piece = true; //set the flag to true
                    temp_flip_cells.push((r as usize, c as usize)); // add it to the temporary list of opponent pieces to flip
                                                                    //dont want to break here because we need to find a player cell in this same direction
                }
                p if p == player => {
                    // if we found the players peice
                    if found_opponent_piece {
                        //we only need to treat the case where there was an opponent piece before
                        final_flip_cells.extend(&temp_flip_cells); //add the opponent cells that are supposed to be flipped
                        final_flip_cells.push((row, col)); //also push the selected cell
                        valid_move_found = true; // set the flag to true
                                                 //dont want to return here because there might be others cells we still have to flip
                    }
                    break; // dont have to keep looking in that direction
                }
                _ => break, // if you find anything else (an empty cell) break out of checking that offset direciton
            } //end match
        } // end infinite loop
    } //end for loop
    (valid_move_found, final_flip_cells) //return if a valid move has been completed and all the cells that need to be flipped
}

//flip a vector list of pieces on the board
//paramaters are a mutible board and a vector of peices to flip and what character to flip them to
//return is nothing
fn flip_pieces(board: &mut [[char; DIM]; DIM], pieces_to_flip: Vec<(usize, usize)>, player: char) {
    for (flip_row, flip_col) in pieces_to_flip {
        // Go through all the pieces to flip
        board[flip_row][flip_col] = player; // Flip them
    }
}

//check if the player has any valid moves
//input parameters are the board and the player
//return is a boolean true if player has valid moves
fn has_valid_moves(board: [[char; DIM]; DIM], player: char) -> bool {
    for row in 0..DIM {
        //go throught all the rows
        for col in 0..DIM {
            // go throught each cell
            if attempt_move(board, row, col, player).0 {
                // attempt a move in that cell
                //note that we pass it a clone of board so the move is not actaully executed
                return true; // a move is possible
            }
        }
    }
    false // a move is not possible
}

// count the number of pieces at the end of the game
// input parameters are the board
// return is a tuple containing the two scores
fn count_pieces(board: &[[char; DIM]; DIM]) -> (u32, u32) {
    let mut black_count = 0; //blacks score count
    let mut white_count = 0; //white score count
    for row in board.iter() {
        //go throught all the rows
        for cell in row.iter() {
            //go throught all the cells
            match cell {
                //match player with corresponding score
                'B' => black_count += 1,
                'W' => white_count += 1,
                _ => {}
            }
        }
    }
    (black_count, white_count) //retrun the two scores
}

fn main() {
    let mut board = initialize_board(); //intialize the board as specified in the assignment
    print_board(board); //print the board
    let mut player = 'B'; //set the first players to black

    loop {
        //infinite loop that allows for moves to go on
        if !has_valid_moves(board, player) {
            // check if the current player does not have any valid moves
            println!("{} player has no valid move.", player); //print that the player has no valid moves
            let opponent = if player == 'B' { 'W' } else { 'B' }; //deterime the opponent
            if !has_valid_moves(board, opponent) {
                //check if opponent has any valid movies
                println!("{} player has no valid move.", opponent); //print that the player has no valid movies, ie both player have no valid moves
                let (black_score, white_score) = count_pieces(&board); //count the peices
                if black_score > white_score {
                    //print who won and the final scores
                    println!("Black wins by {} points!", black_score - white_score);
                } else if white_score > black_score {
                    println!("White wins by {} points!", white_score - black_score);
                } else {
                    println!("It's a tie!");
                }
                break; // End the game
            }
        }

        print!("Enter move for colour {} (RowCol): ", player); //Prompt the user for a move
        let (row_idx, col_idx, valid_input) = get_move_input(); // get the move
        if valid_input {
            //if its valid
            let (valid_move, cells_to_flip) = attempt_move(board, row_idx, col_idx, player); //attempt the move
            if valid_move {
                //if its valid input and the move was succesfully completed
                flip_pieces(&mut board, cells_to_flip, player); // flip the pieces from the valid move
                print_board(board);
                player = if player == 'B' { 'W' } else { 'B' }; //swtich the current player
            } else {
                println!("Invalid move. Try again."); // Handle invalid move
                print_board(board);
            }
        } else {
            println!("Invalid move. Try again."); // Handle invalid move
            print_board(board);
        }
    } //end loop
} //end main
