use square;
use square::Square;
use bitboard::Bitboard;

fn sliding_attack(sq: Square, occupied: Bitboard, deltas: &[i8]) -> Bitboard {
    let mut attack = Bitboard(0);

    for delta in deltas {
        let Square(mut s) = sq;

        loop {
            s += *delta;
            if s < 0 || s >= 64 || square::distance(Square(s), Square(s - delta)) > 2 {
                break;
            }

            attack.add(Square(s));

            if occupied.contains(Square(s)) {
                break;
            }
        }
    }

    attack
}

const ROOK_DELTAS: [i8; 4] = [8, 1, -8, -1];

const ROOK_INDEXES: [usize; 64] = [
    0, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 20480, 22528, 23552, 24576, 25600, 26624,
    27648, 28672, 30720, 32768, 33792, 34816, 35840, 36864, 37888, 38912, 40960, 43008, 44032,
    45056, 46080, 47104, 48128, 49152, 51200, 53248, 54272, 55296, 56320, 57344, 58368, 59392,
    61440, 63488, 64512, 65536, 66560, 67584, 68608, 69632, 71680, 73728, 74752, 75776, 76800,
    77824, 78848, 79872, 81920, 86016, 88064, 90112, 92160, 94208, 96256, 98304];

pub struct Table {
    rook_masks: [Bitboard; 64],
    rook_table: [Bitboard; 0x19000],
}

impl Table {
    pub fn new() -> Table {
        let mut table = Table { rook_masks: [Bitboard(0); 64], rook_table: [Bitboard(0); 0x19000] };

        for sq in Bitboard::all() {
            let edges = ((Bitboard::rank(0) | Bitboard::rank(7)) & !Bitboard::rank(sq.rank())) |
                        ((Bitboard::file(0) | Bitboard::file(7)) & !Bitboard::file(sq.file()));

            let mask = sliding_attack(sq, Bitboard(0), &ROOK_DELTAS) & !edges;
            table.rook_masks[sq.0 as usize] = mask;

            for subset in mask.subsets() {
                let attacks = sliding_attack(sq, subset, &ROOK_DELTAS);
                let index = magic_index(&ROOK_INDEXES, &table.rook_masks, sq, subset);
                table.rook_table[index] = attacks;
            }
        }

        table
    }

    pub fn rook_attacks(self, sq: Square, occupied: Bitboard) -> Bitboard {
        self.rook_table[magic_index(&ROOK_INDEXES, &self.rook_masks, sq, occupied)]
    }
}

fn magic_index(indexes: &[usize], masks: &[Bitboard], Square(sq): Square, occupied: Bitboard) -> usize {
    indexes[sq as usize] + occupied.pext(masks[sq as usize]) as usize
}

mod test {
    use magics::ROOK_DELTAS;
    use magics::sliding_attack;
    use magics::Table;
    use bitboard::Bitboard;
    use square;

    #[test]
    fn test_sliding_rook_attacks() {
        let attack = sliding_attack(square::D6, Bitboard(0x3f7f28802826f5b9), &ROOK_DELTAS);
        assert_eq!(attack, Bitboard(0x8370808000000));
    }

    #[test]
    fn test_rook_attacks() {
        let table = Table::new();
        assert_eq!(table.rook_attacks(square::D6, Bitboard(0x3f7f28802826f5b9)), Bitboard(0x8370808000000));
    }
}