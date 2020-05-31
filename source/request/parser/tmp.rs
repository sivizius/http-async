/// Parser for Hyper Text Transfer Protocol Requests.
pub struct    Parser
{
  /// Current State of the State Machine.
  pub ( super ) state:                  ParserState,
  /// Reset State of the State Machine, for `EscapeX`/`EscapeY` and `NewLineCR`/`NewLineLF`.
  pub ( super ) reset:                  ParserState,
  /// Auxiliary State of the State Machine, for `HeadValueAwait`.
  pub ( super ) goto:                   ParserState,
  /// Header Field.
  pub ( super ) header:                 RequestHeaderField,
  /// One Request Blueprint, which will be filled over time.
  pub ( super ) cache:                  Request,
  /// Buffer of Requests.
  pub ( super ) requests:               Vec < Request >,
  /// Auxiliary buffer, cleared after use.
  pub ( super ) auxiliary:              Vec < u8  >,
  /// Auxiliary number of remaining bytes of request body.
  pub ( super ) remaining:              usize,
  /// Auxiliary key for key-value-pairs.
  pub ( super ) key:                    String,
  /// Auxiliary byte for escape parsing (%xy).
  pub ( super ) byte:                   u8,
  /// Auxiliary unsigned integer for parsing request header value.
  pub ( super ) value:                  u64,
  /// Split Key-Value-Pairs of the Query at this character.
  pub ( super ) querySeperator:         u8,
}

/// Constructor for `Parser`.
pub fn  Parser  ( )
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
    querySeperator:                     b'&',
  }
}

impl          Parser
{
  /// Try to convert auxiliary bytes to utf8-String.
  pub ( super )
  fn            aux2utf8
  (
    &mut self,
  )
  ->  Result
      <
        String,
        FromUtf8Error,
      >
  {
    let mut auxiliary                   =   Vec::new  ( );
    mem::swap
    (
      &mut auxiliary,
      &mut self.auxiliary,
    );
    String::from_utf8 ( auxiliary )
  }

  /// Helper for Simple State Transitions.
  /// If `condition` is true, return the new state `success` successfully.
  /// Otherwise return the error `failure` unsuccessfully.
  ///
  /// # Arguments
  /// * `input`                         –   left value of comparison,
  /// * `expected`                      –   right value of comparison,
  /// * `success`                       –   result on success,
  /// * `failure`                       –   result on failure.
  pub ( super )
  fn expect
  (
    &mut self,
    input:                              u8,
    expected:                           u8,
    success:                            ParserState,
    failure:                            ParserError,
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    if  input ==  expected
    {
      Ok  ( success )
    }
    else
    {
      Err ( failure )
    }
  }

  /// Helper for Simple State Transitions.
  /// If `condition` is true, return the new state `success` successfully.
  /// Otherwise return the error `failure` unsuccessfully.
  ///
  /// # Arguments
  /// * `input`                         –   left value of comparison,
  /// * `expected`                      –   right value of comparison,
  /// * `success`                       –   result on success,
  /// * `failure`                       –   result on failure.
  pub ( super )
  fn expectAnd < F >
  (
    &mut self,
    input:                              u8,
    expected:                           u8,
    function:                           F,
    failure:                            ParserError,
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  where
    F:
      FnOnce
      (
        &mut  Self,
      )
      ->  Result
          <
            ParserState,
            ParserError,
          >,
  {
    if  input ==  expected
    {
      function  ( self  )
    }
    else
    {
      Err ( failure )
    }
  }

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
    .try_fold
    (
      0,
      |
        _,
        &byte,
      |
      {
        self.state
        =   self
            .next
            (
              byte,
            )?;
        Ok  ( self.requests.len ( ) )
      }
    )
  }

  /// Helper to Parse Header Key.
  /// Failes, if `input` and `expected` are not equal and `input` is invalid in header key.
  /// If `input` and `expected` are just not equal, but valid, this function sets the
  /// `auxiliary`-buffer to the `key` and returns ParserState::HeaderKeyX successfully.
  ///
  /// # Arguments
  /// * `input`                         –   left value of comparison,
  /// * `expected`                      –   right value of comparison,
  /// * `success`                       –   result, if values are equal,
  /// * `key`                           –   key on failure.
  pub ( super )
  fn header
  (
    &mut self,
    input:                              u8,
    expected:                           u8,
    success:                            ParserState,
    key:                                &[  u8  ],
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    if  input ==  expected
    {
      Ok  ( success                 )
    }
    else  if  Self::isToken ( input )
    {
      self.auxiliary                    =  key.to_vec ( );
      Ok  ( ParserState::HeadKeyX   )
    }
    else
    {
      Err ( ParserError::HeaderKey  )
    }
  }

  /// Helper to check if byte is allowed in tokens.
  ///
  /// # Arguments
  /// * `input`                         –   value to check.
  pub ( super )
  fn            isToken
  (
    input:                              u8,
  )
  ->  bool
  {
    match input
    {
      | b'!'  ..= b'&'
      | b'*'
      | b'+'
      | b'-'
      | b'.'
      | b'0'  ..= b'9'
      | b'A'  ..= b'Z'
      | b'^'  ..= b'z'
      | b'|'
      | b'~'
      =>  true,
      _
      =>  false,
    }
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
    requests
  }
}

/// State of the Parser.
#[derive(Clone,Copy)]
pub ( super ) enum  ParserState
{
  BodyAwait,
  BodyConsume,
  EscapeX,
  EscapeY,
  HeadKey,
  HeadKeyC,
  HeadKeyCo,
  HeadKeyCon,
  HeadKeyCont,
  HeadKeyConte,
  HeadKeyConten,
  HeadKeyContent,
  HeadKeyContentDash,
  HeadKeyContentL,
  HeadKeyContentLe,
  HeadKeyContentLen,
  HeadKeyContentLeng,
  HeadKeyContentLengt,
  HeadKeyContentLength,
  HeadKeyCoo,
  HeadKeyCook,
  HeadKeyCooki,
  HeadKeyCookie,
  HeadKeyX,
  HeadValueAwait,
  HeadValueParseCookieKey,
  HeadValueParseCookieQuote,
  HeadValueParseCookieSemicolon,
  HeadValueParseCookieSpace,
  HeadValueParseCookieValue,
  HeadValueParseNumber,
  HeadValueX,
  Method,
  MethodC,
  MethodCO,
  MethodCON,
  MethodCONN,
  MethodCONNE,
  MethodCONNEC,
  MethodD,
  MethodDE,
  MethodDEL,
  MethodDELE,
  MethodDELET,
  MethodG,
  MethodGE,
  MethodH,
  MethodHE,
  MethodHEA,
  MethodO,
  MethodOP,
  MethodOPT,
  MethodOPTI,
  MethodOPTIO,
  MethodOPTION,
  MethodP,
  MethodPA,
  MethodPAT,
  MethodPATC,
  MethodPO,
  MethodPOS,
  MethodPU,
  MethodT,
  MethodTR,
  MethodTRA,
  MethodTRAC,
  NewLineCR,
  NewLineLF,
  PathAwait,
  PathConsume,
  QueryKey,
  QueryValue,
  Version,
  VersionH,
  VersionHT,
  VersionHTT,
  VersionHTTP,
  VersionNumber,
  Version0,
  Version0P,
  Version1,
  Version1P,
  Version2,
  Version2P,
  Version3,
  Version3P,
  _Number_,
}

//  ===============================================================================================
//  ===============================================================================================
//  ===============================================================================================
//  ===============================================================================================

/*
/// Beautiful Discrete State Machine
macro_rules! bdsm
{
  (
    $InputType:ty,
    $InputType:ty,
    $StateType:ident,
    $ResultType:ty,
    $( $State:ident => $Function:tt, )*
  )
  =>  {
        pub enum $StateType
        {
          $( $State, )*
          _Number_,
        }

        const ParserTransitions:
        [
          &dyn Fn ( Self, $InputType, ) -> $ResultType;
          $StateType::_Number_ as usize
        ]
        =   [
              $( $Function, )*
            ];
      };
}

bdsm!
(
  u8,
  ParserState2,
  Result  < ParserState2, ParserError >,
  BodyAwait
  =>  (
        &|
          this,
          byte,
        |
        self.expectAnd
        (
          byte, b'\n',
          | this |
          Ok
          (
            if  this.remaining  > 0
            {
              ParserState::BodyConsume
            }
            else
            {
              let mut request         =   Request ( );
              mem::swap
              (
                &mut request,
                &mut this.cache,
              );
              match requests
              {
                Some  ( requests  )
                =>  requests.push ( request ),
                None
                =>  *requests         =   Some  ( vec!  ( request ) ),
              }
              ParserState::Method
            }
          ),
          ParserError::Body,
        ),
);

const ParserTransitions:
[
  &dyn Fn ( Self, u8 ) -> Result  < ParserState, ParserError>;
  ParserState::_Number_ as usize
]
=   [
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
      &| this,  byte, | { Ok  ( ParserState::_Number_ ) },
    ];
*/

/*
/// State of the Parser parsing a Path.
enum          PathState
{
  Start,
  Key,
  Value,
}
*/
/*
impl          Path
{
  /// Try to parse Path from from Transmission Control Protocol Stream.
  ///
  /// # Arguments
  /// * `stream`                        – Transmission Control Protocol Stream.
  pub async fn  parse
  (
    mut stream:                         &mut TcpStream,
    configuration:                      &Configuration,
  )
  ->  Option  < Path  >
  {
    let mut result                      =   None;
    let mut path                        =   String::new ( );
    let mut key                         =   String::new ( );
    let mut value                       =   String::new ( );
    let mut query                       =   Vec::new    ( );
    let mut state                       =   PathState::Start;
    let mut length                      =   0;
    while let Some  ( char  )           =   Request::readChar ( &mut stream )
    {
      if  length  < configuration.maxPathLength
      {
        length                          +=  1;
        match char
        {
          ' '
          =>  {
                match state
                {
                  | PathState::Value
                  | PathState::Key
                  =>  if !key.is_empty()
                      {
                        query
                          .push
                          (
                            KeyValuePair
                            {
                              key,
                              value,
                            }
                          );
                      },
                  PathState::Start
                  =>  {},
                }
                result
                =   Some
                    (
                      Path
                      {
                        path,
                        query,
                      }
                    );
                break;
              },
          '?'
          =>  match state
              {
                PathState::Start        =>  state = PathState::Key,
                PathState::Key          =>  key.push      ( char  as  char  ),
                PathState::Value        =>  value.push    ( char  as  char  ),
              },
          '='
          =>  match state
              {
                PathState::Start        =>  path.push     ( char  as  char  ),
                PathState::Key          =>  state = PathState::Value,
                PathState::Value        =>  value.push    ( char  as  char  ),
              },
          '\r' | '\n'
          =>  break,
          _
          =>  if  char  ==  configuration.querySeperator
              {
                match state
                {
                  PathState::Start      =>  path.push     ( char  as  char  ),
                  | PathState::Key
                  | PathState::Value
                  =>  {
                        state           =   PathState::Key;
                        if !key.is_empty()
                        {
                          query
                            .push
                            (
                              KeyValuePair
                              {
                                key:    key.clone(),
                                value:  value.clone(),
                              }
                            );
                        }
                        key             =   String::new ( );
                        value           =   String::new ( );
                      },
                }
              }
              else
              {
                match state
                {
                  PathState::Start      =>  path.push     ( char  as  char  ),
                  PathState::Key        =>  key.push      ( char  as  char  ),
                  PathState::Value      =>  value.push    ( char  as  char  ),
                }
              },
        }
      }
      else
      {
        break
      }
    }
    result
  }
}
*/
