use std::io;
 
fn main() {
    let mut user_input = String::new();
    let mut cmd : char;
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];
    let mut player: i32 = -1;

    while check_if_win(board) == false {
        player = player * (-1);
        if player == 1{
            println!("Player 1 (X), write number (1-9):");
        }
        else{
            println!("Player 2 (Y), write number (1-9):");
        }

        user_input.clear();
        let _ = io::stdin().read_line(&mut user_input); 
        cmd = user_input.chars().nth(0).unwrap();

        let mut number = cmd.to_digit(10).unwrap();

        if check_if_update_possible(board, number) == false {
            println!("Wrong!");
            break
        }

        update_board(&mut board,&mut number, player);
        print_board(board);
    }
    println!("Player {:?} wins!", player)
}

fn update_board(board : &mut [[char; 3]; 3], number : &mut u32, player : i32) {


    *number = *number -1;
    let row = (*number / 3) as usize;
    let col = (*number % 3) as usize;

    if player == 1 {
        board[row][col] = 'X';
    }
    else {
        board[row][col] = 'Y';
    }
}

fn check_if_update_possible(board: [[char; 3]; 3], number: u32) -> bool {
    if number > 0 && number < 10 {
        let index = number - 1;
        let row = (index / 3) as usize;
        let col = (index % 3) as usize;

        if board[row][col] == ' ' {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn check_if_win(board: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return true; 
        }
        if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return true; 
        }
    }
    
    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return true; 
    }
    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return true; 
    }
    false
}

fn print_board(board: [[char; 3]; 3]) {
    for row in board.iter() {
        println!("{:?}", row);
    }
}