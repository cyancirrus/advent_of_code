#![allow(dead_code, unused)]
use std::cmp::Ordering;
use std::mem;
use std::time::Instant;
use std::{error::Error, fs};

const NUMBER_SHAPES: usize = 6;
const N: usize = 3;

pub fn fill_rotate_0(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[(i + x) * n + j + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(i + x) * n + j + y] = shape[i * N + j];
        }
    }
    true
}

pub fn fill_rotate_1(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    // j decrease i increase
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[(j + x) * n + N - i - 1 + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(j + x) * n + N - i - 1 + y] = shape[i * N + j];
        }
    }
    true
}

pub fn fill_rotate_2(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[(N - i - 1 + x) * n + N - j - 1 + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(N - i - 1 + x) * n + N - j - 1 + y] = shape[i * N + j];
        }
    }
    true
}

pub fn fill_rotate_3(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    // j decrease i increase
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if shape[i * N + j] && space[(N - j + x - 1) * n + i + y] {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(N - j + x - 1) * n + i + y] = shape[i * N + j];
        }
    }
    true
}

pub fn unfill_rotate_0(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if !(shape[i * N + j] && space[(i + x) * n + j + y]) {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(i + x) * n + j + y] = !shape[i * N + j];
        }
    }
    true
}

pub fn unfill_rotate_1(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    // j decrease i increase
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if !(shape[i * N + j] && space[(j + x) * n + N - i - 1 + y]) {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(j + x) * n + N - i - 1 + y] = !shape[i * N + j];
        }
    }
    true
}

pub fn unfill_rotate_2(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if !(shape[i * N + j] && space[(N - i - 1 + x) * n + N - j - 1 + y]) {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(N - i - 1 + x) * n + N - j - 1 + y] = !shape[i * N + j];
        }
    }
    true
}

pub fn unfill_rotate_3(
    space: &mut [bool],
    shape: &[bool],
    m: usize,
    n: usize,
    x: usize,
    y: usize,
) -> bool {
    // j decrease i increase
    if x + N > m || y + N > n {
        return false;
    }
    for i in 0..N {
        for j in 0..N {
            if !(shape[i * N + j] && space[(N - j + x - 1) * n + i + y]) {
                return false;
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            space[(N - j + x - 1) * n + i + y] = !shape[i * N + j];
        }
    }
    true
}
