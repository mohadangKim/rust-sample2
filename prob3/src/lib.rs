pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for (row_idx, row_content) in minefield.iter().enumerate() {
        let mut s = String::new();
        for (col_idx, col_content) in row_content.chars().enumerate() {
            if col_content == '*' {
                s.push('*');
            } else {
                let mut count = 0;
                for i in row_idx.saturating_sub(1)..=row_idx + 1 {
                    for j in col_idx.saturating_sub(1)..=col_idx + 1 {
                        if i < minefield.len()
                            && j < row_content.len()
                            && minefield[i].chars().nth(j) == Some('*')
                        {
                            count += 1;
                        }
                    }
                }

                if count == 0 {
                    s.push(' ');
                } else {
                    s.push((count as u8 + b'0') as char);
                }
            }
        }
        vec.push(s);
    }
    vec
}

pub fn annotate_use_map(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_idx, row_content)| {
            row_content
                .chars()
                .enumerate()
                .map(|(col_idx, col_content)| {
                    if col_content == '*' {
                        '*'
                    } else {
                        let mut count = 0;

                        for i in row_idx.saturating_sub(1)..=row_idx + 1 {
                            for j in col_idx.saturating_sub(1)..=col_idx + 1 {
                                if i < minefield.len()
                                    && j < row_content.len()
                                    && minefield[i].chars().nth(j) == Some('*')
                                {
                                    count += 1;
                                }
                            }
                        }

                        if count == 0 {
                            ' '
                        } else {
                            (count as u8 + b'0') as char
                        }
                    }
                })
                .collect()
        })
        .collect()
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