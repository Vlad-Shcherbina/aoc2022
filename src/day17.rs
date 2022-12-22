pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut wind_iter = input.trim_end().chars().cycle();
    let mut well: Vec<u32> = vec![BOTTOM];
    let mut cur_piece_idx = 0;
    for _ in 0..2022 {
        let cur_piece = PIECES[cur_piece_idx];
        let mut x = 3;
        let mut y = well.len() + 3;
        assert!(fits(&well, cur_piece, x, y));
        loop {
            let x1 = match wind_iter.next().unwrap() {
                '<' => x - 1,
                '>' => x + 1,
                _ => panic!(),
            };
            if fits(&well, cur_piece, x1, y) {
                x = x1;
            }
            if !fits(&well, cur_piece, x, y - 1) {
                break;
            }
            y -= 1;
        }
        place(&mut well, cur_piece, x, y);

        cur_piece_idx += 1;
        if cur_piece_idx == PIECES.len() {
            cur_piece_idx = 0;
        }
    }
    out((well.len() - 1).to_string());
}

fn fits(well: &[u32], piece: &[u32], x: usize, y: usize) -> bool {
    for (i, &row) in piece.iter().enumerate() {
        if well.get(y + i).copied().unwrap_or(WALL) & (row << x) != 0 {
            return false;
        }
    }
    true
}

fn place(well: &mut Vec<u32>, piece: &[u32], x: usize, y: usize) {
    if well.len() < y + piece.len() {
        well.resize(y + piece.len(), WALL);
    }
    for (i, &row) in piece.iter().enumerate() {
        assert_eq!(well[y + i] & (row << x), 0);
        well[y + i] |= row << x;
    }
}

const WALL: u32 =   0b100000001;
const BOTTOM: u32 = 0b111111111;

// LSB is left, so it's flipped horizontally.
// y is up, so it's also flipped vertically.
const PIECES: &[&[u32]] = &[
    &[
        0b1111,
    ], &[
        0b010,
        0b111,
        0b010,
    ],
    &[
        0b111,
        0b100,
        0b100,
    ], &[
        0b1,
        0b1,
        0b1,
        0b1,
    ], &[
        0b11,
        0b11,
    ]
];
