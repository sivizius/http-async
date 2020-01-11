//! Simple Server, who speaks the Hyper Text Transfer Protocol, with async-std.
//! # Usage
//! Include this:
//! ```
//!   [dependencies]
//!   http-async = "0.1"
//! ```
//! in you project `Cargo.toml`-file.
//! # Licence

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

/// Protocol Methods.
pub mod method;
/// Path and Queries to Resources.
pub mod path;
/// Requests to Server.
pub mod request;
/// Responses to Client.
pub mod response;
/// Status Codes
pub mod status;
/// Protocol Versions.
pub mod version;

use
{
  crate::
  {
    request::
    {
      Request,
    },
    response::
    {
      Response,
    },
  },
  async_std::
  {
    io::
    {
      Error,
    },
    net::
    {
      TcpListener,
      ToSocketAddrs,
    },
    prelude::*,
    sync::
    {
      Arc,
    },
    task,
  },
  std::
  {
    fmt::
    {
      Display,
    },
  },
};

/// Simple Key-Value-Pair. E.g. for header-fields, Queries, etc.
#[derive(Debug)]
pub struct    KeyValuePair
{
  /// Key.
  pub key:                              String,
  /// Value.
  pub value:                            String,
}

/// Creates a `Future` to start a Hyper Text Transfer Protocol Server.
///
/// # Arguments
/// * `address`                         – server binds to this address,
/// * `this`                            – some data, that will be passed to `handler` everytime, someone connects,
/// * `handler`                         – handler for Hyper Text Transfer Protocol Requests.
pub async fn  server
<
  Address,
  Data,
  Handler,
>
(
  address:                              Address,
  this:                                 Arc < Data    >,
  handler:                              Arc < Handler >,
)
->  Result
    <
      (),
      Error,
    >
where
  Address:                              ToSocketAddrs + Display + Send + Sync + Clone + 'static,
  Data:                                 Send + Sync + 'static,
  Handler:                              Send + Sync + 'static +
    Fn
    (
      Request,
      Arc < Data  >,
    )
    ->  Response,
{
  let     socket
  =   TcpListener::bind ( &address   )
        .await
        .expect ( "Failed to bind"  );
  println!("Waiting for connections");
  loop
  {
    let     this                        =   this.clone    ( );
    let     handler                     =   handler.clone ( );
    let     address                     =   address.clone ( );
    let
    (
      mut tcpStream,
      client
    )
    =   socket
          .accept()
          .await
          .unwrap();
    task::spawn
    (
      async move
      {
        match Request().parse
              (
                &mut tcpStream,
              ).await
        {
          Ok  ( request )
          =>  match match tcpStream
                            .write
                            (
                              handler
                              (
                                request,
                                this,
                              )
                                .into_vector  ( )
                                .as_slice     ( )
                            )
                            .await
                    {
                      Ok    ( _     )
                      =>  tcpStream
                            .flush().await,
                      Err   ( error )
                      =>  Err ( error ),
                    }
              {
                Ok  ( _       )
                =>  println!
                    (
                      "Success! {} -> {}",
                      address,
                      client,
                    ),
                Err ( error )
                =>  eprintln!
                    (
                      "Send Fail: {}",
                      error,
                    ),
              },
          Err ( message )
          =>  eprintln! ( "Input Fail: {}", message ),
        }
      }
    );
  }
}
