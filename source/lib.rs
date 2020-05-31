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
#![warn(future_incompatible)]
#![warn(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(unused_results)]

#![allow(clippy::suspicious_else_formatting)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

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
      parser::
      {
        Parser,
      },
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
    borrow::
    {
      Cow,
    },
    time::
    {
      Duration,
      Instant,
    },
  },
};

/// Configuration of the HTTP Server.
#[
  derive
  (
    Clone,
    Copy,
    Debug,
  )
]
pub struct    Configuration
{
  /// Address, on which the Server should listen to.
  pub address:                          SocketAddr,
  /// Split Key-Value-Pairs of the Query at this character.
  pub querySeperator:                   u8,
  /// Maximum Bytes of Request Content.
  /// Prevents DOS by Allocation a large Buffer (Header ›Content-Length‹ could contain any decimal value) without ever filling it.
  pub maxContentLength:                 usize,
  /// Maximum Numbers of Headers.
  /// Prevents Slow Lorris Attacks:
  ///   Client might slowly send Header by Header for ever,
  ///     but because neither the Connection times out nor the Request every ends,
  ///       the Server keeps reading the Stream.
  pub maxHeaderEntries:                 usize,
  /// Maximum Length of a single Header Entry.
  pub maxHeaderLength:                  usize,
  /// Maximum Length of a single Cookie Field in Header.
  pub maxCookieLength:                  usize,
  /// Maximum Length of Path.
  /// Prevents Slow Lorris Attacks.
  pub maxPathLength:                    usize,
  /// Duration, after which reading from TCP-Stream without receiving a `Request` should time out.
  pub readTimeOut:                      Duration,
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

/// Copy-on-Write-type of strings.
pub type      CowStr
=   Cow
    <
      'static,
      str,
    >;

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
  Data,
  Handler,
  Promise,
>
(
  configuration:                        Configuration,
  this:                                 Arc < Data    >,
  handler:                              Arc < Handler >,
)
->  Result
    <
      (),
      Error,
    >
where
  Data:                                 Send + Sync + 'static,
  Handler:                              Send + Sync + 'static +
    Fn
    (
      Request,
      Arc < Data  >,
    )
    -> Promise,
  Promise:                              Send + Sync + 'static + Future<Output=Response>,
{
  let     socket
  =   TcpListener::bind ( &configuration.address   )
      .await
      .expect ( "Failed to bind"  );
  println!
  (
    "Waiting for connections on {}",
    configuration.address,
  );
  loop
  {
    let     this                        =   this.clone          ( );
    let     handler                     =   handler.clone       ( );
    let     configuration               =   configuration.clone ( );
    let     address                     =   configuration.address;
    let
    (
      mut tcpStream,
      client
    )
    =   socket
        .accept()
        .await
        .unwrap();
    let mut time                        =   Instant::now  ( );
    let     _
    =   spawn
        (
          async move
          {
            let mut counter             =   0usize;
            let mut buffer              =   vec!  [ 0u8;  1024  ];
            let mut parser              =   Parser  ( configuration );
            loop
            {
              match tcpStream
                    .read ( &mut  buffer  )
                    .await
              {
                Ok  ( length  )
                =>  if  length > 0
                    {
                      match parser
                            .feed
                            (
                              &buffer,
                              length,
                            )
                      {
                        Ok  ( requests )
                        =>  if  requests  > 0
                            {
                              for request
                              in  parser.take ( )
                              {
                                match match tcpStream
                                            .write
                                            (
                                              handler
                                              (
                                                request,
                                                this.clone(),
                                              )
                                              .await
                                              .into_vector  ( )
                                              .as_slice     ( )
                                            )
                                            .await
                                      {
                                        Ok  ( _     ) =>  tcpStream.flush().await,
                                        Err ( error ) =>  Err ( error ),
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
                                }
                              }
                              time      =   Instant::now  ( );
                            }
                            else  if  time.elapsed  ( ) > configuration.readTimeOut
                            {
                              eprintln!
                              (
                                "Reading next Request timed out for {}",
                                client,
                              );
                              break;
                            },
                        Err ( error )
                        =>  {
                              eprintln!
                              (
                                "Parsing Fail: {}\n{:#?}",
                                error,
                                parser,
                              );
                              break;
                            },
                      }
                    }
                    else
                    {
                      //  eventually the client closes the connection
                      println!
                      (
                        "Empty Packet by {} @ {}. Connection Closed.",
                        client,
                        address,
                      );
                      break;
                    },
                Err ( error   )
                =>  {
                      eprintln!
                      (
                        "Input Fail: {}",
                        error,
                      );
                      break;
                    },
              }
              counter                     +=  1;
            }
          }
        );
  }
}
