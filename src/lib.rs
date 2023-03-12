#![allow(dead_code)]
#![deny(warnings)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
pub mod _cp;
pub mod a_star;
pub mod aa_tree;
pub mod ab_tree;
pub mod abs_diff;
pub mod accounting_method;
pub mod ackermann;
pub mod activation;
pub mod adaptive_heap_sort;
pub mod addition_theorem;
pub mod addressable_heap;
pub mod af_heap;
pub mod algorithm;
pub mod bdd;
pub mod berlekamp_massey;
pub mod bf;
pub mod binary_decision_diagram;
pub mod bm;
pub mod boyer_moore;
pub mod boyer_moore_horspool;
pub mod boyer_moore_majority_vote;
pub mod branching_program;
pub mod cht;
pub mod circle_hough_transform;
pub mod clamp;
pub mod convex_hull;
pub mod convex_hull_trick;
pub mod data_structure;
pub mod deterministic_finite_acceptor;
pub mod deterministic_finite_automaton;
pub mod deterministic_fsa;
pub mod deterministic_fsm;
pub mod dfa;
pub mod dfsa;
pub mod dfsm;
pub mod disjoint_set;
pub mod disjoint_set_union;
pub mod dsu;
pub mod dsu_by_rank;
pub mod dsu_by_size;
pub mod dsu_groups;
pub mod dsu_labels;
pub mod dsu_optim;
pub mod dsu_path_compression;
pub mod dsu_same;
pub mod dsu_trait;
pub mod exponentiation;
pub mod finite_state_automaton;
pub mod finite_state_machine;
pub mod frank_wolfe;
pub mod fsa;
pub mod fsm;
pub mod fw;
pub mod kmp;
pub mod knuth_morris_pratt;
pub mod lfsr;
pub mod line;
pub mod line2d;
pub mod line3d;
pub mod line_container;
pub mod linear_feedback_shift_register;
pub mod partially_persistent_dsu;
pub mod persistent_dsu;
pub mod point;
pub mod point1d;
pub mod point2d;
pub mod point3d;
pub mod potentialized_dsu;
pub mod pow;
pub mod power;
pub mod prelude;
pub mod proof_of_work;
pub mod rabin_karp;
pub mod retain;
pub mod rk;
pub mod rollback_dsu;
pub mod size_trait;
pub mod string_matching;
pub mod string_searching;
pub mod truncated_division;
pub mod uf;
pub mod union_find;
pub mod vec_dedup;
pub mod vec_unique;
pub mod weighted_dsu;
pub mod zhu_takaoka;
pub mod zobrist_hashing;
pub mod zolotarev_lemma;
