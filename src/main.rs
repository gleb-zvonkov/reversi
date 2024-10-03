use std::io;

fn initialize_board() -> [[char; 8]; 8] {
    //returns an 8x8 2D array of characters
    let mut board = [['.'; 8]; 8]; // Create a mutable 8x8 board where every cell is initially filled with a '.'

    board[3][3] = 'W'; // Set the initial pieces on the board
    board[3][4] = 'B';
    board[4][3] = 'B';
    board[4][4] = 'W';

    board //return the board
}

fn print_board(board: [[char; 8]; 8]) {
    //intakes a the board (8x8 2D array of characters)
    println!("  abcdefgh"); // Print the column labels
    for (i, row) in board.iter().enumerate() {
        //board.iter() returns an iterator over the rows of the 2D array board
        //enumerate()  returns a new iterator that produces pairs
        //i will be the index of the row (an integer)
        //row will be the actual reference to that row
        print!("{} ", (b'a' + i as u8) as char); // Print row labels

        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

fn get_move_input() -> (char, char) {
    //returns a tuple containing two characters (representing row and column)
    let mut input = String::new(); // Create a mutable string variable 'input' to store the player's input.
                                   // Print a prompt asking the player to enter their move

    io::stdin() // Read a line of input from standard input (the player)
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim(); // Remove any leading and trailing whitespace from the input string

    if trimmed.len() != 2 {
        // If the length of the trimmed input is not exactly 2 characters
        panic!("Invalid input format. Please use the format 'RowCol'.");
    }

    let row = trimmed.chars().nth(0).unwrap(); // Extract the first character of the input string
    let col = trimmed.chars().nth(1).unwrap(); // Extract the second character of the input string

    (row, col)
}

fn is_valid_move(board: &mut [[char; 8]; 8], row: usize, col: usize, player: char) -> bool {
    // Directions for checking: (row_offset, col_offset)
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1), // Up-left, Up, Up-right
        (0, -1),
        (0, 1), // Left, Right
        (1, -1),
        (1, 0),
        (1, 1), // Down-left, Down, Down-right
    ];

    // The cell must be empty
    if board[row][col] != '.' {
        return false;
    }

    let opponent = if player == 'B' { 'W' } else { 'B' };
    let mut valid_move_found = false;

    // Check each direction for a valid move
    for (row_offset, col_offset) in directions.iter() {
        let mut r = row as isize;
        let mut c = col as isize;
        let mut found_opponent_piece = false; // Flag to track if we found an opponent's piece
        let mut pieces_to_flip = Vec::new(); // Reset this for every direction

        // Move in the current direction
        loop {
            r += row_offset;
            c += col_offset;

            // Check if out of bounds
            if r < 0 || r >= 8 || c < 0 || c >= 8 {
                break;
            }

            match board[r as usize][c as usize] {
                // Found an opponent's piece, so we might flip it
                p if p == opponent => {
                    found_opponent_piece = true;
                    pieces_to_flip.push((r as usize, c as usize)); // Track opponent piece to flip
                }
                // Found the player's piece and it's valid only if we found opponent pieces before
                p if p == player => {
                    if found_opponent_piece {
                        // Flip the pieces
                        //println!("Flipping pieces: {:?}", pieces_to_flip);
                        for (flip_row, flip_col) in pieces_to_flip {
                            board[flip_row][flip_col] = player;
                        }
                        valid_move_found = true;
                    }
                    break;
                }
                // Found an empty cell or something else
                _ => break,
            }
        }
    }

    valid_move_found
}

fn main() {
    let mut board = initialize_board();
    print_board(board); //arrays implement the copy trait
    let mut current_player = 'B';

    loop {
        print!("Enter move for colour {} (RowCol): ", current_player);
        let (row_char, col_char) = get_move_input();
        let row_idx = (row_char as u8 - b'a') as usize;
        let col_idx = (col_char as u8 - b'a') as usize;

        if is_valid_move(&mut board, row_idx, col_idx, current_player) {
            board[row_idx][col_idx] = current_player;
            print_board(board);
            current_player = if current_player == 'B' { 'W' } else { 'B' };
        } else {
            println!("Invalid move. Try again.");
            print_board(board);
        }
    }
}
