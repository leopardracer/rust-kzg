use std::{ffi::CStr, fs::File, io::Read, os::fd::IntoRawFd, path::PathBuf, ptr::null_mut};

use kzg::eip_4844::{
    load_trusted_setup_string, Blob, CKZGSettings, KZGCommitment, BYTES_PER_COMMITMENT,
    BYTES_PER_FIELD_ELEMENT, BYTES_PER_G1, BYTES_PER_G2, C_KZG_RET, C_KZG_RET_BADARGS,
    C_KZG_RET_OK,
};
use libc::FILE;

use crate::tests::{
    eip_4844::generate_random_blob_bytes,
    utils::{get_manifest_dir, get_trusted_setup_path},
};

fn get_trusted_setup_fixture_path(fixture: &str) -> String {
    let filename = if cfg!(feature = "minimal-spec") {
        "trusted_setup_4_fixture.txt"
    } else {
        "trusted_setup_fixture.txt"
    };

    PathBuf::from(get_manifest_dir())
        .join("src/tests/fixtures")
        .join(fixture)
        .join(filename)
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn blob_to_kzg_commitment_invalid_blob_test(
    load_trusted_setup: unsafe extern "C" fn(
        *mut CKZGSettings,
        *const u8,
        usize,
        *const u8,
        usize,
    ) -> C_KZG_RET,
    blob_to_kzg_commitment: unsafe extern "C" fn(
        out: *mut KZGCommitment,
        blob: *const Blob,
        s: &CKZGSettings,
    ) -> C_KZG_RET,
) {
    let mut file = File::open(get_trusted_setup_path())
        .map_err(|_| {})
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (g1_bytes, g2_bytes) = load_trusted_setup_string(&contents).unwrap();

    let mut c_settings = CKZGSettings {
        g1_values: null_mut(),
        g2_values: null_mut(),
        max_width: 0,
        roots_of_unity: null_mut(),
    };

    let status = unsafe {
        load_trusted_setup(
            &mut c_settings,
            g1_bytes.as_ptr(),
            g1_bytes.len() / BYTES_PER_G1,
            g2_bytes.as_ptr(),
            g2_bytes.len() / BYTES_PER_G2,
        )
    };
    assert_eq!(status, C_KZG_RET_OK);

    let mut rng = rand::thread_rng();
    let mut blob_bytes = generate_random_blob_bytes(&mut rng);

    let bls_modulus: [u8; BYTES_PER_FIELD_ELEMENT] = [
        0x73, 0xED, 0xA7, 0x53, 0x29, 0x9D, 0x7D, 0x48, 0x33, 0x39, 0xD8, 0x08, 0x09, 0xA1, 0xD8,
        0x05, 0x53, 0xBD, 0xA4, 0x02, 0xFF, 0xFE, 0x5B, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
        0x00, 0x01,
    ];
    // Make first field element equal to BLS_MODULUS
    blob_bytes[0..BYTES_PER_FIELD_ELEMENT].copy_from_slice(&bls_modulus);

    let blob = Blob { bytes: blob_bytes };
    let mut commitment = KZGCommitment {
        bytes: [0; BYTES_PER_COMMITMENT],
    };

    let output = unsafe { blob_to_kzg_commitment(&mut commitment, &blob, &c_settings) };

    assert_eq!(output, C_KZG_RET_BADARGS)
}

pub fn load_trusted_setup_invalid_g1_byte_length_test(
    load_trusted_setup: unsafe extern "C" fn(
        *mut CKZGSettings,
        *const u8,
        usize,
        *const u8,
        usize,
    ) -> C_KZG_RET,
) {
    let mut file = File::open(get_trusted_setup_path()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (mut g1_bytes, g2_bytes) = load_trusted_setup_string(&contents).unwrap();
    // Add one more point
    let additional = [0; BYTES_PER_G1];
    g1_bytes.extend_from_slice(&additional);

    let mut loaded_settings = CKZGSettings {
        g1_values: null_mut(),
        g2_values: null_mut(),
        max_width: 0,
        roots_of_unity: null_mut(),
    };

    let status = unsafe {
        load_trusted_setup(
            &mut loaded_settings,
            g1_bytes.as_ptr(),
            g1_bytes.len() / BYTES_PER_G1,
            g2_bytes.as_ptr(),
            g2_bytes.len() / BYTES_PER_G2,
        )
    };

    assert_eq!(status, C_KZG_RET_BADARGS)
}

pub fn load_trusted_setup_invalid_g1_point_test(
    load_trusted_setup: unsafe extern "C" fn(
        *mut CKZGSettings,
        *const u8,
        usize,
        *const u8,
        usize,
    ) -> C_KZG_RET,
) {
    let mut file = File::open(get_trusted_setup_path()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (mut g1_bytes, g2_bytes) = load_trusted_setup_string(&contents).unwrap();
    // Break first G1 point
    g1_bytes[0] = 0;

    let mut loaded_settings = CKZGSettings {
        g1_values: null_mut(),
        g2_values: null_mut(),
        max_width: 0,
        roots_of_unity: null_mut(),
    };

    let status = unsafe {
        load_trusted_setup(
            &mut loaded_settings,
            g1_bytes.as_ptr(),
            g1_bytes.len() / BYTES_PER_G1,
            g2_bytes.as_ptr(),
            g2_bytes.len() / BYTES_PER_G2,
        )
    };

    assert_eq!(status, C_KZG_RET_BADARGS)
}

pub fn load_trusted_setup_invalid_g2_byte_length_test(
    load_trusted_setup: unsafe extern "C" fn(
        *mut CKZGSettings,
        *const u8,
        usize,
        *const u8,
        usize,
    ) -> C_KZG_RET,
) {
    let mut file = File::open(get_trusted_setup_path()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (g1_bytes, mut g2_bytes) = load_trusted_setup_string(&contents).unwrap();
    // Add one more point
    let additional = [0; BYTES_PER_G2];
    g2_bytes.extend_from_slice(&additional);

    let mut loaded_settings = CKZGSettings {
        g1_values: null_mut(),
        g2_values: null_mut(),
        max_width: 0,
        roots_of_unity: null_mut(),
    };

    let status = unsafe {
        load_trusted_setup(
            &mut loaded_settings,
            g1_bytes.as_ptr(),
            g1_bytes.len() / BYTES_PER_G1,
            g2_bytes.as_ptr(),
            g2_bytes.len() / BYTES_PER_G2,
        )
    };

    assert_eq!(status, C_KZG_RET_BADARGS)
}

pub fn load_trusted_setup_invalid_g2_point_test(
    load_trusted_setup: unsafe extern "C" fn(
        *mut CKZGSettings,
        *const u8,
        usize,
        *const u8,
        usize,
    ) -> C_KZG_RET,
) {
    let mut file = File::open(get_trusted_setup_path()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (g1_bytes, mut g2_bytes) = load_trusted_setup_string(&contents).unwrap();
    // Break first G2 point
    g2_bytes[0] = 0;

    let mut loaded_settings = CKZGSettings {
        g1_values: null_mut(),
        g2_values: null_mut(),
        max_width: 0,
        roots_of_unity: null_mut(),
    };

    let status = unsafe {
        load_trusted_setup(
            &mut loaded_settings,
            g1_bytes.as_ptr(),
            g1_bytes.len() / BYTES_PER_G1,
            g2_bytes.as_ptr(),
            g2_bytes.len() / BYTES_PER_G2,
        )
    };

    assert_eq!(status, C_KZG_RET_BADARGS)
}

pub fn load_trusted_setup_invalid_form_test(
    load_trusted_setup: unsafe extern "C" fn(
        *mut CKZGSettings,
        *const u8,
        usize,
        *const u8,
        usize,
    ) -> C_KZG_RET,
) {
    let mut file = File::open(get_trusted_setup_fixture_path("old")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let (g1_bytes, g2_bytes) = load_trusted_setup_string(&contents).unwrap();

    let mut loaded_settings = CKZGSettings {
        g1_values: null_mut(),
        g2_values: null_mut(),
        max_width: 0,
        roots_of_unity: null_mut(),
    };

    let status = unsafe {
        load_trusted_setup(
            &mut loaded_settings,
            g1_bytes.as_ptr(),
            g1_bytes.len() / BYTES_PER_G1,
            g2_bytes.as_ptr(),
            g2_bytes.len() / BYTES_PER_G2,
        )
    };

    assert_eq!(status, C_KZG_RET_BADARGS)
}

pub fn load_trusted_setup_file_invalid_format_test(
    load_trusted_setup_file: unsafe extern "C" fn(
        out: *mut CKZGSettings,
        in_: *mut FILE,
    ) -> C_KZG_RET,
) {
    struct Fixture {
        name: String,
        message: String,
    }

    let fixtures = [
        Fixture {
            name: "old".to_string(),
            message: "Invalid format because trusted setup is old, i.e. is not in Lagrange form".to_string(),
        },
        Fixture {
            name: "invalid_g1_point_count".to_string(),
            message: "Invalid format because G1 point count in trusted setup file does not match expected".to_string(),
        },
        Fixture {
            name: "invalid_g2_point_count".to_string(),
            message: "Invalid format because G2 point count in trusted setup file does not match expected".to_string(),
        },
        Fixture {
            name: "missing_g1_point_count".to_string(),
            message: "Invalid format because G1 point count is was not found in trusted setup file".to_string(),
        },
        Fixture {
            name: "missing_g2_point_count".to_string(),
            message: "Invalid format because G2 point count is was not found in trusted setup file".to_string(),
        },
        Fixture {
            name: "insufficient_g1_points".to_string(),
            message: "Invalid format because failed to read specified amount of G1 points"
                .to_string(),
        },
        Fixture {
            name: "insufficient_g2_points".to_string(),
            message: "Invalid format because failed to read specified amount of G2 points"
                .to_string(),
        },
        Fixture {
            name: "invalid_chars".to_string(),
            message: "Invalid format because incorrect characters encountered".to_string(),
        },
        Fixture {
            name: "not_a_number".to_string(),
            message: "Invalid format because file starts with not a number".to_string(),
        },
    ];

    for fixture in fixtures {
        let file_path = get_trusted_setup_fixture_path(&fixture.name);
        let file = File::open(file_path.clone()).unwrap();
        let c_file = unsafe {
            libc::fdopen(
                file.into_raw_fd(),
                CStr::from_bytes_with_nul_unchecked(b"r\0").as_ptr(),
            )
        };

        assert!(!c_file.is_null());

        let mut loaded_settings = CKZGSettings {
            g1_values: null_mut(),
            g2_values: null_mut(),
            max_width: 0,
            roots_of_unity: null_mut(),
        };

        let output = unsafe { load_trusted_setup_file(&mut loaded_settings, c_file) };

        assert!(
            output == C_KZG_RET_BADARGS,
            "{}, fixture: {file_path}",
            fixture.message
        );
    }
}