use
{
  super::
  {
    Parser,
    error::ParserError,
    super::
    {
      RequestHeaderField,
      super::
      {
        KeyValuePair,
        method::Method,
        version::Version,
      },
    },
  },
  std::mem,
};

impl          Parser
{
  /// Make State Transition.
  ///
  /// # Arguments
  /// * `byte`                          –   input character.
  pub ( super )
  fn            next
  (
    &mut self,
    byte:                               u8,
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    //  O ( 1 ) or faster, if pointer/code already in cache.
    //  `match` might be implemented as many comparisons and conditional jumps,
    //    but tbh, I am not sure about the internals.
    //  Perhabs one should compare it with the implementation in `next.rs`.
    ParserState::Transitions [ self.state  as  usize ]
    (
      self,
      byte,
    )
  }
}

bdsm!
{
  ParserState                           =>  Transitions ( &mut  Parser, u8, )
                                        ->  Result  < ParserState, ParserError >,
  //  This State should be handled differently and this code should be unreachable,
  //    because instead of calling this function for every byte,
  //      appending incoming bytes is faster.
  Body
  =>  &| this, byte |
      {
        this.auxiliary.push ( byte  );
        this.remaining                  -=  1;
        Ok
        (
          if  this.remaining  > 0
          {
            Self::Body
          }
          else
          {
            mem::swap
            (
              &mut this.auxiliary,
              &mut this.cache.content,
            );
            this.done()
          }
        )
      },
  EscapeX
  =>  &| this, byte |
      match byte
      {
        b'0'  ..= b'9'                  =>  { this.byte =   ( byte  - b'0'        ) * 16; Ok  ( Self::EscapeY ) },
        b'A'  ..= b'F'                  =>  { this.byte =   ( byte  - b'A'  + 10  ) * 16; Ok  ( Self::EscapeY ) },
        b'a'  ..= b'f'                  =>  { this.byte =   ( byte  - b'a'  + 10  ) * 16; Ok  ( Self::EscapeY ) },
        _                               =>  Err ( ParserError::Method     ),
      },
  EscapeY
  =>  &| this, byte |
      match byte
      {
        b'0'  ..= b'9'                  =>  { this.auxiliary.push ( this.byte +   ( byte  - b'0'        ) ); Ok  ( this.reset ) },
        b'A'  ..= b'F'                  =>  { this.auxiliary.push ( this.byte +   ( byte  - b'A'  + 10  ) ); Ok  ( this.reset ) },
        b'a'  ..= b'f'                  =>  { this.auxiliary.push ( this.byte +   ( byte  - b'a'  + 10  ) ); Ok  ( this.reset ) },
        _                               =>  Err ( ParserError::Method     ),
      },
  HeaderKey
  =>  &| this, byte |
      match byte
      {
        b'\r'
        =>  {
              this.remaining            =   this.cache.header.bodyLength  ( );
              if  this.remaining  <=  this.configuration.maxContentLength
              {
                this.reset
                =   if  this.remaining  > 0 { Self::Body    }
                    else                    { this.done ( ) };
                Ok  ( Self::NewLineLF )
              }
              else
              {
                Err ( ParserError::Body )
              }
            },
        b'C'                            =>  Ok  ( Self::HeaderKeyC        ),
        b'H'                            =>  Ok  ( Self::HeaderKeyH        ),
        b'U'                            =>  Ok  ( Self::HeaderKeyU        ),
        byte  if  Parser::isToken ( byte  )
        =>  {
              this.auxiliary.push ( byte  );
              Ok  ( Self::HeaderKeyX  )
            },
        _                               =>  Err ( ParserError::HeaderKey  ),
      },
  //  Assuming Clients usually send expected header fields.
  //  Therefore, only in the uncomon case,
  //    e.g. "Cookie-Lengthrofl",
  //  the auxiliary field will be set and
  //    new bytes will be appended.
  //  Could be space-optimised by sending a reference to the same static string with a length,
  //    but compared to the size of the executable,
  //      that is not much.
  //  Only the first argument of `this.expect()`, `this.header()` and `this.parse()` is variable,
  //    the other should be constants known at compile time.
  HeaderKeyC                            =>  &| this, byte | this.header   ( byte, b'o',   Self::HeaderKeyCo,                  b"C"                          ),
  HeaderKeyCo
  =>  &| this, byte |
      match byte
      {
        b'n'                            =>  Ok  ( Self::HeaderKeyCon      ),
        b'o'                            =>  Ok  ( Self::HeaderKeyCoo      ),
        byte  if  Parser::isToken ( byte  )
        =>  {
              this.auxiliary            =   vec!  [ b'C', b'o', byte, ];
              Ok  ( Self::HeaderKeyX  )
            },
        b':'
        =>  {
              this.key                  =   "Co".to_owned ( );
              this.goto                 =   Self::HeaderValueX;
              Ok  ( Self::HeaderValueAwait  )
            },
        _                               =>  Err ( ParserError::HeaderKey  ),
      },
  HeaderKeyCon                          =>  &| this, byte | this.header   ( byte, b't',   Self::HeaderKeyCont,                b"Con"                        ),
  HeaderKeyCont                         =>  &| this, byte | this.header   ( byte, b'e',   Self::HeaderKeyConte,               b"Cont"                       ),
  HeaderKeyConte                        =>  &| this, byte | this.header   ( byte, b'n',   Self::HeaderKeyConten,              b"Conte"                      ),
  HeaderKeyConten                       =>  &| this, byte | this.header   ( byte, b't',   Self::HeaderKeyContent,             b"Conten"                     ),
  HeaderKeyContent                      =>  &| this, byte | this.header   ( byte, b'-',   Self::HeaderKeyContentDash,         b"Content"                    ),
  HeaderKeyContentDash                  =>  &| this, byte | this.header   ( byte, b'L',   Self::HeaderKeyContentL,            b"Content-"                   ),
  HeaderKeyContentL                     =>  &| this, byte | this.header   ( byte, b'e',   Self::HeaderKeyContentLe,           b"Content-L"                  ),
  HeaderKeyContentLe                    =>  &| this, byte | this.header   ( byte, b'n',   Self::HeaderKeyContentLen,          b"Content-Le"                 ),
  HeaderKeyContentLen                   =>  &| this, byte | this.header   ( byte, b'g',   Self::HeaderKeyContentLeng,         b"Content-Len"                ),
  HeaderKeyContentLeng                  =>  &| this, byte | this.header   ( byte, b't',   Self::HeaderKeyContentLengt,        b"Content-Leng"               ),
  HeaderKeyContentLengt                 =>  &| this, byte | this.header   ( byte, b'h',   Self::HeaderKeyContentLength,       b"Content-Lengt"              ),
  HeaderKeyContentLength                =>  &| this, byte | this.parseNum ( byte,         RequestHeaderField::ContentLength,  b"Content-Length",            ),
  HeaderKeyCoo                          =>  &| this, byte | this.header   ( byte, b'k',   Self::HeaderKeyCook,                b"Coo",                       ),
  HeaderKeyCook                         =>  &| this, byte | this.header   ( byte, b'i',   Self::HeaderKeyCooki,               b"Cook",                      ),
  HeaderKeyCooki                        =>  &| this, byte | this.header   ( byte, b'e',   Self::HeaderKeyCookie,              b"Cooki",                     ),
  HeaderKeyCookie                       =>  &| this, byte | this.parse    ( byte,         Self::HeaderValueParseCookieKey,    b"Cookie",                    ),
  HeaderKeyH                            =>  &| this, byte | this.header   ( byte, b'o',   Self::HeaderKeyHo,                  b"H"                          ),
  HeaderKeyHo                           =>  &| this, byte | this.header   ( byte, b's',   Self::HeaderKeyHos,                 b"Ho"                         ),
  HeaderKeyHos                          =>  &| this, byte | this.header   ( byte, b't',   Self::HeaderKeyHost,                b"Hos"                        ),
  HeaderKeyHost                         =>  &| this, byte | this.parseStr ( byte,         RequestHeaderField::Host,           b"Host",                      ),
  HeaderKeyU                            =>  &| this, byte | this.header   ( byte, b's',   Self::HeaderKeyUs,                  b"U"                          ),
  HeaderKeyUs                           =>  &| this, byte | this.header   ( byte, b'e',   Self::HeaderKeyUse,                 b"Us"                         ),
  HeaderKeyUse                          =>  &| this, byte | this.header   ( byte, b'r',   Self::HeaderKeyUser,                b"Use"                        ),
  HeaderKeyUser                         =>  &| this, byte | this.header   ( byte, b'-',   Self::HeaderKeyUserDash,            b"User"                       ),
  HeaderKeyUserDash                     =>  &| this, byte | this.header   ( byte, b'A',   Self::HeaderKeyUserA,               b"User-"                      ),
  HeaderKeyUserA                        =>  &| this, byte | this.header   ( byte, b'g',   Self::HeaderKeyUserAg,              b"User-A"                     ),
  HeaderKeyUserAg                       =>  &| this, byte | this.header   ( byte, b'e',   Self::HeaderKeyUserAge,             b"User-Ag"                    ),
  HeaderKeyUserAge                      =>  &| this, byte | this.header   ( byte, b'n',   Self::HeaderKeyUserAgen,            b"User-Age"                   ),
  HeaderKeyUserAgen                     =>  &| this, byte | this.header   ( byte, b't',   Self::HeaderKeyUserAgent,           b"User-Agen"                  ),
  HeaderKeyUserAgent                    =>  &| this, byte | this.parseStr ( byte,         RequestHeaderField::UserAgent,      b"User-Agent",                ),
  HeaderKeyX
  =>  &| this, byte |
      match byte
      {
        b':'
        =>  this
            .aux2utf8 ( )
            .map_or
            (
              Err ( ParserError::HeaderKey    ),
              | key |
              {
                this.key          =   key;
                this.goto         =   Self::HeaderValueX;
                Ok  ( Self::HeaderValueAwait  )
              },
            ),
        byte  if  Parser::isToken  ( byte  )
        =>  {
              this.auxiliary.push ( byte  );
              Ok  ( Self::HeaderKeyX          )
            },
        _                               =>  Err ( ParserError::HeaderKey    ),
      },
  HeaderValueAwait
  =>  &| this, byte |
      match byte
      {
        | b'\r'
        =>  this.call ( Self::NewLineLF,  Self::HeaderValueAwait, ),
        | b' '
        | b'\t'
        =>  Ok  ( Self::HeaderValueAwait    ),
        | b'0'    ..= b'9'
        | b'A'  ..= b'Z'
        | b'a'  ..= b'z'
        | b'"'
        | b'*'
        //  more might be allowed…
        =>  {
              this.byte                 =   byte;
              this.auxiliary.push ( byte  );
              Ok  ( this.goto )
            },
        _                               =>  Err ( ParserError::HeaderValue  ),
      },
  HeaderValueParseCookieKey
  =>  &| this, byte |
      match byte
      {
        b'='
        =>  this
            .aux2utf8 ( )
            .map_or
            (
              Err   ( ParserError::CookieKey            ),
              | key |
              {
                this.key                =   key;
                Ok  ( Self::HeaderValueParseCookieValue )
              },
            ),
        byte  if  Parser::isToken ( byte  )
        =>  {
              this.auxiliary.push ( byte  );
              Ok    ( Self::HeaderValueParseCookieKey   )
            },
        _                               =>  Err ( ParserError::CookieKey    ),
      },
  HeaderValueParseCookieQuote
  =>  &| this, byte |
      Ok
      (
        if  byte  ==  b'"'
        {
          Self::HeaderValueParseCookieSemicolon
        }
        else
        {
          this.auxiliary.push ( byte  );
          Self::HeaderValueParseCookieQuote
        }
      ),
  HeaderValueParseCookieSemicolon       =>  &| this, byte | this.expect   ( byte, b';',   Self::HeaderValueParseCookieSpace,  ParserError::CookieValue,     ),
  HeaderValueParseCookieSpace
  =>  &| this, byte |
      this.expectAnd
      (
        byte, b' ',
        | this |
        {
          this
          .aux2utf8 ( )
          .map_or
          (
            Err ( ParserError::CookieValue  ),
            | value |
            {
              let mut key               =   String::new ( );
              mem::swap
              (
                &mut key,
                &mut this.key,
              );
              this.cache.cookies.push
              (
                KeyValuePair
                {
                  key,
                  value,
                }
              );
              Ok  ( Self::HeaderValueParseCookieValue )
            }
          )
        },
        ParserError::CookieValue,
      ),
  HeaderValueParseCookieValue
  =>  &| this, byte |
      match byte
      {
        b'\r'
        =>  this
            .aux2utf8 ( )
            .map_or
            (
              Err   ( ParserError::CookieValue          ),
              | value |
              {
                let mut key             =   String::new ( );
                mem::swap
                (
                  &mut key,
                  &mut this.key,
                );
                this.cache.cookies.push
                (
                  KeyValuePair
                  {
                    key,
                    value,
                  }
                );
                this.call ( Self::NewLineLF,  Self::HeaderKey )
              }
            ),
        | b'"'  if  this.auxiliary.is_empty ( )
        =>  Ok        ( Self::HeaderValueParseCookieQuote                   ),
        | b'%'
        =>  this.call ( Self::EscapeX,  Self::HeaderValueParseCookieValue,  ),
        | b';'
        =>  Ok        ( Self::HeaderValueParseCookieSpace                   ),
        | b'\0' ..= b' '
        | b'"'
        | b','
        | b'\\'
        =>  Err       ( ParserError::CookieValue                            ),
        _
        =>  {
              this.auxiliary.push ( byte  );
              Ok  ( Self::HeaderValueParseCookieValue )
            },
      },
  HeaderValueParseNumber
  =>  &| this, byte |
      match byte
      {
        b'0'  ..= b'9'
        =>  {
              this.value                =   10 * this.value + ( this.byte - b'0' ) as u64;
              this.byte                 =   byte;
              Ok  ( Self::HeaderValueParseNumber )
            },
        b'\r'
        =>  {
              let     value             =   10 * this.value + ( this.byte - b'0' ) as u64;
              this.auxiliary.clear  ( );
              this.value                =   0;
              if  this
                  .cache
                  .header
                  .setNumber
                  (
                    this.header,
                    value as  usize,
                  )
                  .is_ok ( )
                    { this.call ( Self::NewLineLF,  Self::HeaderKey,  ) }
              else  { Err       ( ParserError::HeaderValue            ) }
            },
        _                               =>  Err ( ParserError::HeaderValue                                  ),
      },
  HeaderValueParseString
  =>  &| this, byte |
      match byte
      {
        b'%'                            =>  this.call ( Self::EscapeX,  Self::HeaderValueParseString,       ),
        b'\r'
        =>  this
            .aux2utf8 ( )
            .map_or
            (
              Err ( ParserError::HeaderValue      ),
              | value |
              if  this
                  .cache
                  .header
                  .setString
                  (
                    this.header,
                    value,
                  )
                  .is_ok ( )
                    { this.call ( Self::NewLineLF,  Self::HeaderKey,  ) }
              else  { Err       ( ParserError::HeaderValue            ) }
            ),
        byte
        =>  {
              this.auxiliary.push ( byte  );
              Ok  ( Self::HeaderValueParseString  )
            },
      },
  HeaderValueX
  =>  &| this, byte |
      if  byte  ==  b'\r'
      {
        this
        .aux2utf8 ( )
        .map_or
        (
          Err ( ParserError::HeaderValue  ),
          | value |
          {
            let mut key                 =   String::new ( );
            mem::swap
            (
              &mut key,
              &mut this.key,
            );
            this
            .cache
            .header
            .push
            (
              KeyValuePair
              {
                key,
                value,
              }
            );
            this.call ( Self::NewLineLF,  Self::HeaderKey,  )
          }
        )
      }
      else
      {
        this.auxiliary.push ( byte  );
        Ok    ( Self::HeaderValueX  )
      },
  Method
  =>  &| _, byte |
      match byte
      {
        b'C'                            =>  Ok  ( Self::MethodC       ),
        b'D'                            =>  Ok  ( Self::MethodD       ),
        b'G'                            =>  Ok  ( Self::MethodG       ),
        b'H'                            =>  Ok  ( Self::MethodH       ),
        b'O'                            =>  Ok  ( Self::MethodO       ),
        b'P'                            =>  Ok  ( Self::MethodP       ),
        b'T'                            =>  Ok  ( Self::MethodT       ),
        _                               =>  Err ( ParserError::Method ),
      },
  MethodC                               =>  &| this, byte | this.expect   ( byte, b'O',   Self::MethodCO,                     ParserError::Method,          ),
  MethodCO                              =>  &| this, byte | this.expect   ( byte, b'N',   Self::MethodCON,                    ParserError::Method,          ),
  MethodCON                             =>  &| this, byte | this.expect   ( byte, b'N',   Self::MethodCONN,                   ParserError::Method,          ),
  MethodCONN                            =>  &| this, byte | this.expect   ( byte, b'E',   Self::MethodCONNE,                  ParserError::Method,          ),
  MethodCONNE                           =>  &| this, byte | this.expect   ( byte, b'C',   Self::MethodCONNEC,                 ParserError::Method,          ),
  MethodCONNEC                          =>  &| this, byte | this.method   ( byte, b'T',   Method::Connect,                                                  ),
  MethodD                               =>  &| this, byte | this.expect   ( byte, b'E',   Self::MethodDE,                     ParserError::Method,          ),
  MethodDE                              =>  &| this, byte | this.expect   ( byte, b'L',   Self::MethodDEL,                    ParserError::Method,          ),
  MethodDEL                             =>  &| this, byte | this.expect   ( byte, b'E',   Self::MethodDELE,                   ParserError::Method,          ),
  MethodDELE                            =>  &| this, byte | this.expect   ( byte, b'T',   Self::MethodDELET,                  ParserError::Method,          ),
  MethodDELET                           =>  &| this, byte | this.method   ( byte, b'E',   Method::Delete,                                                   ),
  MethodG                               =>  &| this, byte | this.expect   ( byte, b'E',   Self::MethodGE,                     ParserError::Method,          ),
  MethodGE                              =>  &| this, byte | this.method   ( byte, b'T',   Method::Get,                                                      ),
  MethodH                               =>  &| this, byte | this.expect   ( byte, b'E',   Self::MethodHE,                     ParserError::Method,          ),
  MethodHE                              =>  &| this, byte | this.expect   ( byte, b'A',   Self::MethodHEA,                    ParserError::Method,          ),
  MethodHEA                             =>  &| this, byte | this.method   ( byte, b'D',   Method::Head,                                                     ),
  MethodO                               =>  &| this, byte | this.expect   ( byte, b'P',   Self::MethodOP,                     ParserError::Method,          ),
  MethodOP                              =>  &| this, byte | this.expect   ( byte, b'T',   Self::MethodOPT,                    ParserError::Method,          ),
  MethodOPT                             =>  &| this, byte | this.expect   ( byte, b'I',   Self::MethodOPTI,                   ParserError::Method,          ),
  MethodOPTI                            =>  &| this, byte | this.expect   ( byte, b'O',   Self::MethodOPTIO,                  ParserError::Method,          ),
  MethodOPTIO                           =>  &| this, byte | this.expect   ( byte, b'N',   Self::MethodOPTION,                 ParserError::Method,          ),
  MethodOPTION                          =>  &| this, byte | this.method   ( byte, b'S',   Method::Options,                                                  ),
  MethodP
  =>  &| _, byte |
      match byte
      {
        b'A'                            =>  Ok  ( Self::MethodPA      ),
        b'O'                            =>  Ok  ( Self::MethodPO      ),
        b'U'                            =>  Ok  ( Self::MethodPU      ),
        _                               =>  Err ( ParserError::Method ),
      },
  MethodPA                              =>  &| this, byte | this.expect   ( byte, b'T',   Self::MethodPAT,                    ParserError::Method,          ),
  MethodPAT                             =>  &| this, byte | this.expect   ( byte, b'C',   Self::MethodPATC,                   ParserError::Method,          ),
  MethodPATC                            =>  &| this, byte | this.method   ( byte, b'H',   Method::Patch,                                                    ),
  MethodPO                              =>  &| this, byte | this.expect   ( byte, b'S',   Self::MethodPOS,                    ParserError::Method,          ),
  MethodPOS                             =>  &| this, byte | this.method   ( byte, b'T',   Method::Post,                                                     ),
  MethodPU                              =>  &| this, byte | this.method   ( byte, b'T',   Method::Put,                                                      ),
  MethodT                               =>  &| this, byte | this.expect   ( byte, b'A',   Self::MethodTR,                     ParserError::Method,          ),
  MethodTR                              =>  &| this, byte | this.expect   ( byte, b'C',   Self::MethodTRA,                    ParserError::Method,          ),
  MethodTRA                             =>  &| this, byte | this.expect   ( byte, b'C',   Self::MethodTRAC,                   ParserError::Method,          ),
  MethodTRAC                            =>  &| this, byte | this.method   ( byte, b'E',   Method::Trace,                                                    ),
  NewLineCR                             =>  &| this, byte | this.expect   ( byte, b'\r',  Self::NewLineLF,                    ParserError::CarriageReturn,  ),
  NewLineLF                             =>  &| this, byte | this.expect   ( byte, b'\n',  this.reset,                         ParserError::LineFeed,        ),
  PathAwait                             =>  &| this, byte | this.expect   ( byte, b' ',   Self::PathConsume,                  ParserError::Method,          ),
  PathConsume
  =>  &| this, byte |
      match byte
      {
        | b'%'
        =>  this.call ( Self::EscapeX,  Self::PathConsume,  ),
        | b'\r'
        | b' '
        | b'?'
        =>  this
            .aux2utf8 ( )
            .map_or
            (
              Err ( ParserError::Path ),
              | path  |
              match byte
              {
                b'\r'                   =>  this.call ( Self::NewLineLF,  Self::HeaderKey,  ),
                b' '
                =>  {
                      this
                      .cache
                      .path             =   path;
                      Ok  ( Self::Version )
                    },
                _
                =>  {
                      this
                      .cache
                      .path             =   path;
                      Ok  ( Self::QueryKey  )
                    },
              },
            ),
        //  ToDo: Have another look at RFC 1945, 2396 and 2616
        | b'\0'
        | b'\n'
        =>  Err ( ParserError::Path ),
        _
        =>  {
              this.auxiliary.push ( byte  );
              Ok  ( Self::PathConsume )
            },
      },
  QueryKey
  =>  &| this, byte |
      match byte
      {
        | b'%'
        =>  this.call ( Self::EscapeX,  Self::QueryKey, ),
        | b'\0'
        | b'\n'
        =>  Err ( ParserError::Query  ),
        _
        =>  if  byte  ==  this.configuration.querySeperator
            ||  byte  ==  b'\r'
            ||  byte  ==  b'='
            {
              this
              .aux2utf8 ( )
              .map_or
              (
                Err ( ParserError::Query ),
                | key |
                match byte
                {
                  b'\r'
                  =>  {
                        this.cache.query.push
                        (
                          KeyValuePair
                          {
                            key,
                            value:      String::new ( ),
                          }
                        );
                        this.call ( Self::NewLineLF,  Self::HeaderKey,  )
                      },
                  b'='
                  =>  {
                        this.key        =   key;
                        Ok  ( Self::QueryValue  )
                      },
                  _
                  =>  {
                        this.cache.query.push
                        (
                          KeyValuePair
                          {
                            key,
                            value:      String::new ( ),
                          }
                        );
                        Ok  ( Self::QueryKey    )
                      },
                },
              )
            }
            else
            {
              this.auxiliary.push ( byte  );
              Ok  ( Self::QueryKey  )
            },
      },
  QueryValue
  =>  &| this, byte |
      match byte
      {
        | b'%'
        =>  this.call ( Self::EscapeX,  Self::QueryValue, ),
        | b'\0'
        | b'\n'
        =>  Err ( ParserError::Query  ),
        _
        =>  if  byte  ==  this.configuration.querySeperator
            ||  byte  ==  b' '
            ||  byte  ==  b'\r'
            {
              this
              .aux2utf8 ( )
              .map_or
              (
                Err ( ParserError::Query ),
                | value |
                {
                  let mut key           =   String::new ( );
                  mem::swap
                  (
                    &mut key,
                    &mut this.key,
                  );
                  this.cache.query.push
                  (
                    KeyValuePair
                    {
                      key,
                      value,
                    }
                  );
                  match byte
                  {
                    b'\r'               =>  this.call ( Self::NewLineLF,  Self::HeaderKey,  ),
                    b' '                =>  Ok  ( Self::Version   ),
                    _                   =>  Ok  ( Self::QueryKey  ),
                  }
                },
              )
            }
            else  if  this.auxiliary.len  ( ) <=  this.configuration.maxPathLength
            {
              this.auxiliary.push   ( byte  );
              Ok  ( Self::QueryValue  )
            }
            else
            {
              Err ( ParserError::Query )
            },
      },
  Version                               =>  &| this, byte | this.expect   ( byte, b'H',   Self::VersionH,                     ParserError::Version,         ),
  VersionH                              =>  &| this, byte | this.expect   ( byte, b'T',   Self::VersionHT,                    ParserError::Version,         ),
  VersionHT                             =>  &| this, byte | this.expect   ( byte, b'T',   Self::VersionHTT,                   ParserError::Version,         ),
  VersionHTT                            =>  &| this, byte | this.expect   ( byte, b'P',   Self::VersionHTTP,                  ParserError::Version,         ),
  VersionHTTP                           =>  &| this, byte | this.expect   ( byte, b'/',   Self::VersionNumber,                ParserError::Version,         ),
  VersionNumber
  =>  &| _, byte |
      match byte
      {
        b'0'                            =>  Ok  ( Self::Version0        ),
        b'1'                            =>  Ok  ( Self::Version1        ),
        b'2'                            =>  Ok  ( Self::Version2        ),
        b'3'                            =>  Ok  ( Self::Version3        ),
        _                               =>  Err ( ParserError::Version  ),
      },
  Version0                              =>  &| this, byte | this.expect   ( byte, b'.',   Self::Version0P,                    ParserError::Version,         ),
  Version0P                             =>  &| this, byte | this.version  ( byte, b'9',   Version::HTTP_09,                                                 ),
  Version1                              =>  &| this, byte | this.expect   ( byte, b'.',   Self::Version1P,                    ParserError::Version,         ),
  Version1P
  =>  &| this, byte |
      match byte
      {
        b'0'                            =>  { this.cache.version  = Version::HTTP_10; this.call ( Self::NewLineCR,  Self::HeaderKey,  ) },
        b'1'                            =>  { this.cache.version  = Version::HTTP_11; this.call ( Self::NewLineCR,  Self::HeaderKey,  ) },
        _                               =>  Err ( ParserError::Version  ),
      },
  Version2                              =>  &| this, byte | this.expect   ( byte, b'.',   Self::Version2P,                    ParserError::Version,         ),
  Version2P                             =>  &| this, byte | this.version  ( byte, b'0',   Version::HTTP_2,                                                  ),
  Version3                              =>  &| this, byte | this.expect   ( byte, b'.',   Self::Version3P,                    ParserError::Version,         ),
  Version3P                             =>  &| this, byte | this.version  ( byte, b'0',   Version::HTTP_3,                                                  ),
}
