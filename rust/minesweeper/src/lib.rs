pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, &c)| {
                    if c == b'*' {
                        return '*';
                    }
                    let mine_count = [
                        (1, -1),
                        (1, 0),
                        (1, 1),
                        (0, -1),
                        (0, 1),
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                    ]
                    .iter()
                    .filter_map(|(di, dj)| match minefield.get(i.wrapping_add_signed(*di)) {
                        Some(row) => row.as_bytes().get(j.wrapping_add_signed(*dj)).copied(),
                        None => None,
                    })
                    .filter(|&c| c == b'*')
                    .count();

                    match mine_count {
                        0 => ' ',
                        _ => char::from_digit(mine_count as u32, 10).unwrap(),
                    }
                })
                .collect()
        })
        .collect()
}
