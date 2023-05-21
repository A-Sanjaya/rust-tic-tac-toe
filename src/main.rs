struct TicTacToeErr{
    err_note: String,
}

impl TicTacToeErr {
    fn new(err_note: &str) -> TicTacToeErr {
        TicTacToeErr { err_note : String::from(err_note)}
    }
}

struct Player {
    number: Vec<char>
}

impl Player {
    fn new() -> Self {
        Player {number: Vec::new()}
    }
    
    fn turn(&mut self, avaible_number: &mut Vec<char>, number: char) -> Result<(), TicTacToeErr> {
        if avaible_number.contains(&number) {
            self.number.push(number);
            for i in 0..avaible_number.len() {
                if avaible_number[i] == number {
                    avaible_number.remove(i);
                }
            }
            return Ok(())
        }
        Err(TicTacToeErr::new("number is not avaible"))
    }
}

struct TicTacToe {
    avaible_number: Vec<char>,
    player_x: Player,
    player_o: Player,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {avaible_number: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'], player_x: Player::new(), player_o: Player::new()}
    }

    fn player_x_turn(&mut self, number: char) -> Result<(), TicTacToeErr> {
        self.player_x.turn(&mut self.avaible_number, number)
    }
    fn player_o_turn(&mut self, number: char) -> Result<(), TicTacToeErr> {
        self.player_o.turn(&mut self.avaible_number, number)
    }

    fn who_winner(&mut self) {
        let num = 0;

        let is_winner = |player: &mut Player| {
            player.number.sort();

            for i in 0..player.number.len() {
                if player.number[i] <= '3' { //vertical check
                    if player.number.contains(&((player.number[i] as u8 + 3) as char)) && player.number.contains(&((player.number[i] as u8 + 6) as char)) {
                        return true
                    }
                }
                for b in 0..3 {
                    if player.number[i] == (49 + 3*b as u8) as char { //horizontal check
                        if player.number.contains(&((player.number[i] as u8 + 1) as char)) && player.number.contains(&((player.number[i] as u8 + 2)as char)) {
                            return true
                        }
                    }
                }
            }
            if player.number.contains(&'1') {
                let mut n = 0;
                for i in 1..=2 {
                    if player.number.contains(&((49 + 4*i as u8) as char)) {
                        n += 1;
                    } else {
                        break;
                    }
                }
                if n == 2 {
                    return true
                }
            } else if player.number.contains(&'3') {
                let mut n = 0;
                for i in 1..=2 {
                    if player.number.contains(&((51 + 2*i as u8) as char)) {
                        n += 1;
                    } else {
                        break;
                    }
                }
                if n == 2 {
                    return true
                }
            }
            false
        };

        if is_winner(&mut self.player_x) {
            println!("player x is winner");
        }
        else if is_winner(&mut self.player_o) {
            println!("player o is winner");
        }

    }
}

fn main() {
    let mut tic_tac_toe = TicTacToe::new();
    let mut input_number: Vec<u8> = vec![];
    let mut answer_confirmation = String::new();
    let mut error_message: String = String::new();
    let mut number = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    tic_tac_toe.player_o.number = ['3', '5', '7'].to_vec();
    tic_tac_toe.who_winner();
}
