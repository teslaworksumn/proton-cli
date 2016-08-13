
#![allow(dead_code)]

#[derive(Clone)]
pub enum TestSeqSec {
    Good1of1,
    Good1of2,
    Good2of2,
    Good1of3,
    Good2of3,
    Good3of3,
}

/// Retrieves a pre-generated sequence section for testing
/// One second file, 50ms frames, 20 frames per row
/// 3 channels
pub fn get_test_seq_sec(key: TestSeqSec) -> Vec<Vec<u8>> {
    match key {
        TestSeqSec::Good1of1 => vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
            vec![1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8],
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 122, 100, 222, 67, 34, 101, 135]
        ],
        TestSeqSec::Good1of2 => vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 4, 8, 16, 32, 64, 128, 1, 2],
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
        ],
        TestSeqSec::Good2of2 => vec![
            vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
            vec![4, 8, 16, 32, 64, 128, 1, 2, 4, 8],
            vec![89, 144, 233, 122, 100, 222, 67, 34, 101, 135]
        ],
        TestSeqSec::Good1of3 => vec![
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![1, 2, 4, 8, 16, 32, 64],
            vec![1, 1, 2, 3, 5, 8, 13]
        ],
        TestSeqSec::Good2of3 => vec![
            vec![7, 8, 9, 10, 11, 12, 13],
            vec![128, 1, 2, 4, 8, 16, 32],
            vec![21, 34, 55, 89, 144, 233, 122]
        ],
        TestSeqSec::Good3of3 => vec![
            vec![14, 15, 16, 17, 18, 19],
            vec![64, 128, 1, 2, 4, 8],
            vec![100, 222, 67, 34, 101, 135]
        ],
    }
}
