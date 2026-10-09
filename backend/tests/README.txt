
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


- INTEGRATION TESTS are to be located (and therefore placed) in {root}/backend/tests/it/.
    - They all compile into ONE test binary, `it`: tests/it/main.rs declares each `*_test.rs` file as a
        module (plus the shared `common` and `fixtures` modules). One binary links once per change instead
        of once per file, which is most of the build time. Declare every new test file in main.rs.
    - These tests test QView API endpoints from the vantage point of an external project. This gives a high-level view of 
        what the API's actual behavior is compared to its expected/designed behavior.
    - Each file (and therefore module) is to house only one service's endpoints. For instance, the "tournament.rs' service
        is entirely housed under {root}/backend/tests/it/tournament_test.rs. No other service's endpoints should exist in 
        this file.
    - Every test gets its own empty, migrated database from `common::test_db()` (see tests/it/common/mod.rs), so tests
        never share data and can run in parallel. The database server is the RAM-backed Postgres in
        {root}/compose.test.yaml, which {root}/backend/run-tests.sh starts.
    - Run one file's tests by its module name, e.g. `cargo test --test it tournament_test::`.
