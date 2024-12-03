/*
 * This software is available under the Apache-2.0 license.
 * See https://www.apache.org/licenses/LICENSE-2.0.txt for full text.
 *
 * Copyright (c) 2024, Gluu, Inc.
 */
use std::sync::{Arc, Mutex};
use std::thread;

//#[cfg(not(target_arch = "wasm32"))]
//use reqwest::blocking::Client;
use reqwest::Response;
//#[cfg(target_arch = "wasm32")]
use reqwest::Client;
//use tokio::runtime::Runtime;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};
use std::{thread::sleep, time::Duration};

/// A wrapper providing HTTP request functionality with retry logic.
///
/// The `HttpClient` struct allows for sending GET requests with a retry mechanism
/// that attempts to fetch the requested resource up to a maximum number of times
/// if an error occurs.
#[derive(Debug)]
pub struct HttpClient {
    //#[cfg(not(target_arch = "wasm32"))]
    //client: reqwest::blocking::Client,
    //#[cfg(target_arch = "wasm32")]
    client: reqwest::Client,
    max_retries: u32,
    retry_delay: Duration,
}

impl HttpClient {
    /// Constructs a new `HttpClient` instance.
    ///
    /// On native platforms, initializes a `reqwest` client. On WebAssembly, no initialization is required.
    pub fn new(max_retries: u32, retry_delay: Duration) -> Result<Self, HttpClientError> {
        let client = Client::builder()
            .build()
            .map_err(HttpClientError::Initialization)?;

        Ok(Self {
            client,
            max_retries,
            retry_delay,
        })
    }

    /// Sends a GET request to the specified URI with retry logic.
    ///
    /// This method will attempt to fetch the resource up to `max_retries` times, with a delay
    /// between each attempt if it fails.
    pub fn get(&self, uri: &str) -> Result<Response, HttpClientError> {
        //  #[cfg(not(target_arch = "wasm32"))]
        //  {
        //      let mut attempts = 0;
        //      let response = loop {
        //          match self.client.get(uri).send() {
        //              Ok(response) => break response,
        //              Err(e) if attempts < self.max_retries => {
        //                  attempts += 1;
        //                  eprintln!(
        //                      "Request failed (attempt {} of {}): {}. Retrying...",
        //                      attempts, self.max_retries, e
        //                  );
        //                  sleep(self.retry_delay * attempts);
        //              }
        //              Err(e) => return Err(HttpClientError::MaxHttpRetriesReached(e)),
        //          }
        //      };

        //      return response
        //          .error_for_status()
        //          .map_err(HttpClientError::HttpStatus);
        //  }

        //#[cfg(target_arch = "wasm32")]

        // Fetch the JWKS from the jwks_uri
        let mut attempts = 0;
        let runtime = RuntimeBuilder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime"); //Runtime::new().expect("Failed to create Tokio runtime");
        let rt = Arc::new(Mutex::new(runtime));

        // Blocking execution of an async task
        let result = rt.lock().unwrap().block_on(async {
            let client_tmp = Arc::new(Mutex::new(self.client.clone()));
            let client_mut = client_tmp.lock().unwrap();
            Ok(match client_mut.get(uri).send().await {
                Ok(response) => response, //.text().await.map_err(|e| HttpClientError::MaxHttpRetriesReached(e)),
                Err(err) => return Err(HttpClientError::MaxHttpRetriesReached(err)),
            })
        });
        result
    }
}

/// Error type for the HttpClient
#[derive(thiserror::Error, Debug)]
pub enum HttpClientError {
    /// Indicates failure to initialize the HTTP client.
    #[error("Failed to initilize HTTP client: {0}")]
    Initialization(#[source] reqwest::Error),
    /// Indicates an HTTP error response received from an endpoint.
    #[error("Received error HTTP status: {0}")]
    HttpStatus(#[source] reqwest::Error),

    /// Indicates a failure to reach the endpoint after 3 attempts.
    #[error("Could not reach endpoint after trying 3 times: {0}")]
    MaxHttpRetriesReached(#[source] reqwest::Error),
    #[error("Error in Http Request: {0}")]
    ErrorInHTTPRequest(String),
}
