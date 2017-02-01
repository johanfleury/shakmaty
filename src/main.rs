#![feature(associated_consts)]
#![feature(cfg_target_feature)]
#![feature(asm)]
#![feature(field_init_shorthand)]

#![allow(dead_code)]  // Remove

mod square;
mod bitboard;
mod attacks;
mod board;

use board::Board;
use board::Move;
use attacks::Precomp;

extern crate rayon;
use rayon::prelude::*;

fn perft_inner(board: &Board, depth: i8, precomp: &Precomp) -> usize {
    if depth < 1 {
        1
    } else {
        let mut moves: Vec<Move> = Vec::new();
        board.legal_moves(&mut moves, precomp);

        moves.iter().map(|m| {
            let mut child = board.clone();
            child.do_move(m);
            perft_inner(&child, depth - 1, precomp)
        }).sum()
    }
}

fn perft(board: &Board, depth: i8, precomp: &Precomp) -> usize {
    if depth < 1 {
        1
    } else {
        let mut moves: Vec<Move> = Vec::new();
        board.legal_moves(&mut moves, precomp);

        moves.par_iter().map(|m| {
            let mut child = board.clone();
            child.do_move(m);
            let p = perft_inner(&child, depth - 1, precomp);
            println!("{} {}", m, p);
            p
        }).sum()
    }
}

fn main() {
    let precomp = attacks::Precomp::new();
    let mut board = Board::new();

    //board.do_move(&Move::from_uci("e2e4").unwrap());
    //board.do_move(&Move::from_uci("e7e5").unwrap());
    //board.do_move(&Move::from_uci("f1b5").unwrap());

    //assert_eq!(perft(&board, 1, &precomp), 20);
    //assert_eq!(perft(&board, 2, &precomp), 400);
    //assert_eq!(perft(&board, 3, &precomp), 8902);
    assert_eq!(perft(&board, 4, &precomp), 197281);
}
