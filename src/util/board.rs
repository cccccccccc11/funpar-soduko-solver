pub(crate) fn format_board(board: &str) -> String {
    let mut result = String::new();
    let rows = "ABCDEFGHI".chars().collect::<Vec<_>>();
    let cols = "123456789".chars().collect::<Vec<_>>();

    for (i, ch) in board.chars().enumerate() {
        let col = i % 9;
        result.push(if ch == '.' { '.' } else { ch });
        if col == 8 {
            result.push('\n');
        } else {
            result.push(' ');
            if (col + 1) % 3 == 0 {
                result.push('|');
            }
        }
        if (i + 1) % 27 == 0 && i != 80 {
            result.push_str("------+-------+------\n");
        }
    }
    result
}

pub(crate) fn print_board(board: &str) {
    let formatted_board = format_board(board);
    println!("{}", formatted_board);
}
