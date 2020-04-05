//! This file should be autogenerated based on the headers created from the .edl file.

use sgx_types::sgx_status_t;
use enclave_ffi_types::{Ctx, EnclaveBuffer, UserSpaceBuffer};

extern "C" {
    pub fn ocall_allocate(retval: *mut UserSpaceBuffer, buffer: *const u8, length: usize) -> sgx_status_t;

    pub fn ocall_read_db(retval: *mut EnclaveBuffer, context: Ctx, key: *const u8, key_len: usize) -> sgx_status_t;

    pub fn ocall_write_db(
        context: Ctx,
        key: *const u8,
        key_len: usize,
        value: *const u8,
        value_len: usize,
    ) -> sgx_status_t;
}
