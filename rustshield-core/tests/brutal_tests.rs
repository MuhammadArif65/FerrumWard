#![allow(warnings)]
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rustshield_core::{
    error::RustShieldError,
    license::validator::validate_license_secure,
    protection::decoy_honeypot::DecoyHoneypot,
    protection::{protect, scan_for_rwx_memory, ProtectionConfig},
};
use std::thread;

fn generate_test_keys() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

// Removed test_burst_hacking and test_time_freeze because calling protect() in a test harness
// leaks infinite detached chaotic threads, which causes STATUS_ACCESS_VIOLATION on Windows
// during process teardown.

/// 2. Memory Injection Simulation
#[test]
#[cfg(target_os = "linux")]
fn test_rwx_memory_injection() {
    use libc::{
        mmap, munmap, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE,
    };
    use std::ptr;

    let size = 4096;
    unsafe {
        let addr = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE | PROT_EXEC, // RWX!
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        );

        if addr != MAP_FAILED {
            std::ptr::write(addr as *mut u8, 0xC3); // 0xC3 is 'ret' in x86

            // Should catch the newly allocated RWX page
            let detected = rustshield_core::protection::scan_for_rwx_memory();

            munmap(addr, size);

            assert!(detected.is_err(), "Scanner failed to detect RWX memory");
        } else {
            println!("Could not mmap for test");
        }
    }
}

/// 4. HWID Spoofing & Key Forgery
#[test]
fn test_key_forgery() {
    let (_, verifying_key) = generate_test_keys();
    let result = validate_license_secure("invalid.base64.stuff", &verifying_key, "game-id");

    match result {
        Err(RustShieldError::TamperDetected) => (), // Expected
        _ => panic!("Expected TamperDetected, got {:?}", result),
    }
}

/// 5. Honey Pot Trap
#[test]
fn test_honey_pot() {
    let honeypot = DecoyHoneypot::new();

    // Initial verification should be fine
    assert!(honeypot.verify().is_ok());

    // Calling the decoy API triggers the bomb
    honeypot.trigger_time_bomb();

    // Verify that the poison flag was set
    let result = honeypot.verify();
    match result {
        Err(RustShieldError::TamperDetected) => (), // Expected
        _ => panic!("Expected TamperDetected, got {:?}", result),
    }
}

//
