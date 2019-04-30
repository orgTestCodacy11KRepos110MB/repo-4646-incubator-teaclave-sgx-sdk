// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! # Intel(R) Software Guard Extensions Sealing and Unsealing Functions
//!
//! The library provides the following functions:
//!
//! * Exposes APIs to create sealed data which is both confidentiality andintegrity protected.
//! * Exposes an API to unseal sealed data inside the enclave.
//! * Provides APIs to authenticate and verify the input data with AES-GMAC.
//!
//! The library also provides APIs to help calculate the sealed data size, encrypt text length, and Message Authentication Code (MAC) text length.
//!
//! # Description
//!
//! When an enclave is instantiated, it provides protections (confidentiality and
//! integrity) to the data by keeping it within the boundary of the enclave. Enclave
//! developers should identify enclave data and/or state that is considered secret
//! and potentially needs preservation across the following enclave destruction
//! events:
//!
//! * Application is done with the enclave and closes it.
//! * Application itself is closed.
//! * The platform is hibernated or shutdown.
//!
//! In general, the secrets provisioned within an enclave are lost when the enclave
//! is closed. However if the secret data needs to be preserved during one of
//! these events for future use within an enclave, it must be stored outside the
//! enclave boundary before closing the enclave. In order to protect and preserve
//! the data, a mechanism is in place which allows enclave software to retrieve a
//! key unique to that enclave. This key can only be generated by that enclave on
//! that particular platform. Enclave software uses that key to encrypt data to the
//! platform or to decrypt data already on the platform. Refer to these encrypt
//! and decrypt operations as sealing and unsealing respectively as the data is
//! cryptographically sealed to the enclave and platform.
//!
//! To provide strong protection against potential key-wear-out attacks, a unique
//! seal key is generated for each data blob encrypted with the seal_data
//! API call. A key ID for each encrypted data blob is stored in clear alongside the
//! encrypted data blob. The key ID is used to re-generate the seal key to decrypt
//! the data blob.
//!
//! AES-GCM (AES – Advanced Encryption Standard) is utilized to encrypt and
//! MAC-protect the payload. To protect against software-based side channel
//! attacks, the crypto implementation of AES-GCM utilizes AES-NI, which is
//! immune to software-based side channel attacks. The Galois/Counter Mode
//! (GCM) is a mode of operation of the AES algorithm. GCM assures authenticity
//! of the confidential data (of up to about 64 GB per invocation) using a universal
//! hash function. GCM can also provide authentication assurance for additional
//! data (of practically unlimited length per invocation) that is not encrypted. GCM
//! can also provide authentication assurance for additional data (of practically
//! unlimited length per invocation) that is not encrypted. If the GCM input contains
//! only data that is not to be encrypted, the resulting specialization of GCM,
//! called GMAC (Galois Message Authentication Code), is simply an authentication
//! mode for the input data. The mac_aadata API call restricts the input to
//! non-confidential data to provide data origin authentication only. The single
//! output of this function is the authentication tag.
//!

#![no_std]

#![feature(alloc)]
#![allow(non_camel_case_types)]
#![allow(unused_assignments)]

#[macro_use]
extern crate alloc;

extern crate sgx_types;
extern crate sgx_trts;
extern crate sgx_tcrypto;
extern crate sgx_tse;

mod seal;
pub use self::seal::{SgxSealedData, SgxUnsealedData};

mod aad;
pub use self::aad::{SgxMacAadata};

mod internal;
