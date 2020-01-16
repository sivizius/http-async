//! Simple Server, who speaks the Hyper Text Transfer Protocol, with async-std.
//! # Usage
//! Include this:
//! ```
//!   [dependencies]
//!   http-async = "0.1"
//! ```
//! in you project `Cargo.toml`-file.
//! # Licence
//! MIT

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

/// Header of Requests and Responses.
pub mod header;
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
    status::
    {
      Status,
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
      SocketAddr,
      TcpListener,
      ToSocketAddrs,
    },
    prelude::*,
    sync::
    {
      Arc,
    },
    task::
    {
      spawn,
    },
  },
  std::
  {
    fmt::
    {
      Display,
    },
  },
};

/// Configuration of the HTTP Server.
pub struct    Configuration
{
  /// Address, on which the Server should listen to.
  pub address:                          SocketAddr,
  /// Maximum Bytes of Request Content.
  /// Prevents DOS by Allocation a large Buffer (Header ›Content-Length‹ could contain any decimal value) without ever filling it.
  pub maxContent:                       usize,
  /// Maximum Numbers of Headers.
  /// Prevents Slow Lorris Attacks:
  ///   Client might slowly send Header by Header for ever,
  ///     but because neither the Connection times out nor the Request every ends,
  ///       the Server keeps reading the Stream.
  pub maxHeaderEntries:                 usize,
  /// Maximum Length of a Header.
  pub maxHeaderLength:                  usize,
  /// Maximum Length of Path.
  /// Prevents Slow Lorris Attacks.
  pub maxPathLength:                    usize,
  /// Maximum Length of Query String.
  /// Prevents Slow Lorris Attacks.
  pub maxQueryLength:                   usize,
}

/// Just send this content successfully.
#[macro_export]
macro_rules!  content
{
  (
    $Type:expr,
    $Path:expr
  )
  =>  {
        http_async::Content
        (
          http_async::status::Status::Ok,
          $Type,
          include_bytes!  ( $Path )
            .to_vec(),
        )
      };
}

/// Content of a Hyper Text Transfer Protocol Response.
pub struct    Content
{
  statusCode:                           Status,
  contentType:                          &'static str,
  contentBody:                          Vec < u8  >,
}

/// Constructor for `Content`.
///
/// # Arguments
/// * `` –
pub fn        Content
(
  statusCode:                           Status,
  contentType:                          &'static str,
  contentBody:                          Vec < u8  >,
)
->  Content
{
  Content
  {
    statusCode,
    contentType,
    contentBody,
  }
}

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
  println!
  (
    "Waiting for connections on {}",
    address,
  );
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
    spawn
    (
      async move
      {
        let mut counter                 =   0;
        loop
        {
          match Request()
                  .parse
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
                                  this.clone(),
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
                        "Success! {} -> {} #{}",
                        address,
                        client,
                        counter,
                      ),
                  Err ( error )
                  =>  {
                        eprintln!
                        (
                          "Send Fail: {}",
                          error,
                        );
                        break;
                      },
                },
            Err ( error )
            =>  {
                  eprintln!
                  (
                    "Input Fail: {}",
                    error,
                  );
                  break;
                }
          }
          counter                       +=  1;
        }
      }
    );
  }
}
