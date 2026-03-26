use serial_test::{local_parallel_core, local_serial_core};

#[test]
fn test_empty_serial_call() {
    local_serial_core(vec!["beta"], None, || {
        println!("Bar");
    });
}

// Verify that serial_test is compatible with Miri's strict provenance mode.
//
// Run with:
//   MIRIFLAGS="-Zmiri-strict-provenance" cargo +nightly miri test -p serial_test --no-default-features --test tests test_miri_strict_provenance
#[test]
fn test_miri_strict_provenance() {
    local_serial_core(vec!["miri_provenance_test"], None, || {});
    local_parallel_core(vec!["miri_provenance_test"], None, || {});
}
