pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0;
    }

    let map: Vec<&[u8]> = lines.iter().map(|line| line.as_bytes()).collect();
    let map = &map[..];

    (0..map.len())
        .map(|y| {
            (0..map[y].len())
                .filter(|&x| map[y][x] == b'+')
                .map(|x| {
                    ((y + 1)..map.len())
                        .map(|height| {
                            ((x + 1)..map[height].len())
                                .filter(|&width| {
                                    map[height][width] == b'+'
                                        && is_connected(map, (x, y), (width, y))
                                        && is_connected(map, (x, height), (width, height))
                                        && is_connected(map, (x, y), (x, height))
                                        && is_connected(map, (width, y), (width, height))
                                })
                                .count() as u32
                        })
                        .sum::<u32>()
                })
                .sum::<u32>()
        })
        .sum()
}

fn is_connected(map: &[&[u8]], (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
    let on_x_axis = y1 == y2;
    (if on_x_axis { x1..=x2 } else { y1..=y2 }).all(|i| {
        if on_x_axis {
            map[y1][i] == b'+' || map[y1][i] == b'-'
        } else {
            map[i][x1] == b'+' || map[i][x1] == b'|'
        }
    })
}
