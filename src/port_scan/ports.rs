use std::u16::MAX;

pub fn ports() -> Vec<u16> {
    let ports: Vec<u16> = (0..MAX).collect();
    ports
}