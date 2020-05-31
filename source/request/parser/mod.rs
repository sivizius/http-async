/// Import macros.
#[macro_use]
mod macros;

/// Request-Parser Errors.
mod error;
/// Helper internal function.
mod helper;
/// States and Transitions of the State Machine.
mod machine;
// / States and Transitions of the State Machine, Old implementation for benchmarking.
//mod next;

use
{
  error::
  {
    ParserError,
  },
  machine::
  {
    ParserState,
  },
  super::
  {
    Request,
    RequestHeaderField,
    super::
    {
      Configuration,
    },
  },
  std::
  {
    mem,
  },
};

/// Parser for Hyper Text Transfer Protocol Requests.
#[derive(Debug)]
pub struct    Parser
{
  /// Current State of the State Machine.
  state:                                ParserState,
  /// Reset State of the State Machine, for `EscapeX`/`EscapeY` and `NewLineCR`/`NewLineLF`.
  reset:                                ParserState,
  /// Auxiliary State of the State Machine, for `HeadValueAwait`.
  goto:                                 ParserState,
  /// Header Field.
  header:                               RequestHeaderField,
  /// One Request Blueprint, which will be filled over time.
  cache:                                Request,
  /// Buffer of Requests.
  requests:                             Vec < Request >,
  /// Auxiliary buffer, cleared after use.
  auxiliary:                            Vec < u8      >,
  /// Auxiliary number of remaining bytes of request body.
  remaining:                            usize,
  /// Auxiliary key for key-value-pairs.
  key:                                  String,
  /// Auxiliary byte for escape parsing (%xy).
  byte:                                 u8,
  /// Auxiliary unsigned integer for parsing request header value.
  value:                                u64,
  /// Configuration of the Web Server.
  configuration:                        Configuration,
}

/// Constructor for `Parser`.
pub fn  Parser
(
  configuration:                        Configuration
)
->  Parser
{
  Parser
  {
    state:                              ParserState::Method,
    reset:                              ParserState::Method,
    goto:                               ParserState::Method,
    header:                             RequestHeaderField::Dummy,
    cache:                              Request     ( ),
    requests:                           Vec::new    ( ),
    auxiliary:                          Vec::new    ( ),
    remaining:                          0,
    key:                                String::new ( ),
    byte:                               b'\0',
    value:                              0,
    configuration,
  }
}

//  ==  Frontend  ==
impl          Parser
{
  /// Feed parser with some bytes.
  ///
  /// # Arguments
  /// * `bytes`                         –   input for the parser.
  pub fn        feed
  (
    &mut self,
    bytes:                              &[  u8  ],
    length:                             usize,
  )
  ->  Result
      <
        usize,
        ParserError,
      >
  {
    bytes
    .iter (         )
    .take ( length  )
    .try_for_each
    (
      | &byte |
      {
        //println!  ( "next({}={})", byte, byte as char, );
        self.state                    =   self.next ( byte  )?;
        Ok  ( ( ) )
      }
    )?;
    Ok  ( self.requests.len ( ) )
  }

  /// Take requests out of parsers.
  pub fn        take
  (
    &mut self,
  )
  ->  Vec < Request >
  {
    let mut requests                    =   Vec::new  ( );
    mem::swap
    (
      &mut requests,
      &mut self.requests,
    );
    /*
    for request                         in  &requests
    {
      println!("{:#?}", request);
    }
    */
    requests
  }
}
