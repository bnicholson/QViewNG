
This project follows Rust industry standards for maintaining and organizing tests.

The following standards are to be followed:


- UNIT TESTS are to be added in-line at the bottom of the file. Here is an example:

// src/my_module.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}


- INTEGRATION TESTS are to be located (and therefore placed) in this file's parent directory only ({root}/backend/tests).
    - Each file under {root}/backend/tests is its own crate and is run as such when 'cargo test' is run.
    - These tests test QView API endpoints from the vantage point of an external project. This gives a high-level view of 
        what the API's actual behavior is compared to its expected/designed behavior.
    - Each file (and therfore crate) is to house only one service's endpoints. For instance, the "tournament.rs' service
        is entirely housed under {root}/backend/tests/tournament_test.rs. No other service's endpoints should exist in 
        this file.


