pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    for p in ['X', 'O'] {
        if diagonals(p, table) || horizontal(p, table) || vertical(p, table) {
            return format!("player {} won", p);
        }
    }

    "tie".to_owned()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }

    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table {
        if row == [player, player, player] {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col_ind in 0..3 {
        let mut hits = 0;
        for row in table.iter() {
            if row[col_ind] == player {
                hits += 1;
            }
        }
        if hits == 3 {
            return true;
        }
    }
    false
}
