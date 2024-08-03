fn get_mine_count(minefield: &[&str], x: usize, y: usize) -> char {
    if minefield[y].chars().nth(x).unwrap() == '*' {
        return '*';
    }

    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let new_x = x as i32 + i;
            let new_y = y as i32 + j;

            if new_x < 0 || new_y < 0 {
                continue;
            }

            // TODO: max 조건 추가
            
            if let Some(row) = minefield.get(new_y as usize) {
                if let Some(cell) = row.chars().nth(new_x as usize) {
                    if cell == '*' {
                        count += 1;
                    }
                }
            }
        }
    }

    if count == 0 {
        ' '
    } else {
        char::from_u32(count).unwrap()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut field = vec![];

    for y in 0..minefield.len() {
        let mut temp = String::new();
        for x in 0..minefield.first().unwrap_or(&"").len() {
            temp.push(get_mine_count(minefield, x, y));
        }
        field.push(temp);
    }

    field
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

#[cfg(test)]
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
