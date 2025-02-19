#![no_std]

struct ThreatMap<const N: usize> {
    map: [[i32; N]; N]
}

impl<const N: usize> ThreatMap<N> {
    fn new(read_scan_at: &impl Fn(i8, i8) -> char) -> Self {
        let mut map = [[i32::MAX; N]; N];
        ThreatMap { map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
