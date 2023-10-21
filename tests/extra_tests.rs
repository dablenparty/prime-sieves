use rust_primes::{map::MapPrimesIter, pq::QueuePrimesIter};

/// Skips the first 100 primes and takes the next 100. This is more
/// of a sanity check to make sure that the sieves aren't too slow.
const NEXT_100_PRIMES: &[u64] = &[
    547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653,
    659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787,
    797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919,
    929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031, 1033,
    1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151,
    1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223,
];

#[test]
fn test_next_100_primes_queue() {
    let actual: Vec<_> = QueuePrimesIter::default().skip(100).take(100).collect();
    assert_eq!(NEXT_100_PRIMES, actual);
}

#[test]
fn test_next_100_primes_map() {
    let actual: Vec<_> = MapPrimesIter::default().skip(100).take(100).collect();
    assert_eq!(NEXT_100_PRIMES, actual);
}
