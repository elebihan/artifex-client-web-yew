//
// Copyright (C) 2023 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

pub mod rpc {
    pub use artifex_rpc::*;
    use tonic_web_wasm_client::Client;

    /// Create a client for gRPC server.
    pub fn create_client(url: &str) -> artifex_client::ArtifexClient<Client> {
        let client = Client::new(url.to_string());
        artifex_client::ArtifexClient::new(client)
    }
}

pub mod components;
pub mod contexts;
