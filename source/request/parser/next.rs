use
{
  super::
  {
    Parser,
    ParserState,
    error::
    {
      ParserError,
    },
    super::
    {
      super::
      {
        KeyValuePair,
        method::Method,
        version::Version,
      },
    },
  },
  std::
  {
    mem,
  },
};

impl          Parser
{
  /// Depending on the input character `byte` and the previous, try to calculate the next state and
  ///   return it, but do not go there!
  /// Just ignore the calculated cognitive complexity, this function is actually quite trivial.
  ///
  /// # Arguments
  /// * `byte`                          –   character.
  //#[allow(clippy::cognitive_complexity)]
  pub ( super ) fn  next2
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
    match self.state
    {
      //  This State should be handled differently and this code should be unreachable,
      //    because instead of calling this function for every byte,
      //      appending incoming bytes is faster.
      //  But for completeness,
      //    this code is still here.
      ParserState::Body
      =>  Ok
          (
            if  self.remaining  > 0
            {
              self.auxiliary.push ( byte  );
              self.remaining                -=  1;
              ParserState::Body
            }
            else
            {
              self.done()
            }
          ),
      ParserState::NewLineCR            =>  self.expect ( byte, b'\r',  ParserState::NewLineLF,             ParserError::CarriageReturn,  ),
      ParserState::NewLineLF            =>  self.expect ( byte, b'\n',  self.reset,                         ParserError::LineFeed,        ),
      ParserState::EscapeX
      =>  match byte
          {
            b'0'  ..= b'9'              =>  { self.byte = ( byte  - b'0'        ) * 16; Ok  ( ParserState::EscapeY  ) },
            b'A'  ..= b'F'              =>  { self.byte = ( byte  - b'A'  + 10  ) * 16; Ok  ( ParserState::EscapeY  ) },
            b'a'  ..= b'f'              =>  { self.byte = ( byte  - b'a'  + 10  ) * 16; Ok  ( ParserState::EscapeY  ) },
            _                           =>  Err ( ParserError::Method   ),
          },
      ParserState::EscapeY
      =>  match byte
          {
            b'0'  ..= b'9'              =>  { self.auxiliary.push ( self.byte + ( byte  - b'0'        ) ); Ok  ( self.reset ) },
            b'A'  ..= b'F'              =>  { self.auxiliary.push ( self.byte + ( byte  - b'A'  + 10  ) ); Ok  ( self.reset ) },
            b'a'  ..= b'f'              =>  { self.auxiliary.push ( self.byte + ( byte  - b'a'  + 10  ) ); Ok  ( self.reset ) },
            _                           =>  Err ( ParserError::Method   ),
          },
      //  ToDo: Match more stuff…
      ParserState::HeaderKey
      =>  match byte
          {
            b'\r'                       =>  Err ( ParserError::HeaderKey  ),
            b'C'                        =>  Ok  ( ParserState::HeaderKeyC   ),
            _                           =>  Ok  ( ParserState::HeaderKeyX   ),
          },
      ParserState::HeaderKeyC             =>  self.header ( byte, b'o',   ParserState::HeaderKeyCo,             b"C"                          ),
      ParserState::HeaderKeyCo
      =>  match byte
          {
            b'n'                        =>  Ok  ( ParserState::HeaderKeyCon ),
            b'o'                        =>  Ok  ( ParserState::HeaderKeyCoo ),
            _
            =>  {
                  self.auxiliary        =  b"Co".to_vec ( );
                  Ok  ( ParserState::HeaderKeyX )
                },
          },
      ParserState::HeaderKeyCon           =>  self.header ( byte, b't',   ParserState::HeaderKeyCont,           b"Con"                        ),
      ParserState::HeaderKeyCont          =>  self.header ( byte, b'e',   ParserState::HeaderKeyConte,          b"Cont"                       ),
      ParserState::HeaderKeyConte         =>  self.header ( byte, b'n',   ParserState::HeaderKeyConten,         b"Conte"                      ),
      ParserState::HeaderKeyConten        =>  self.header ( byte, b't',   ParserState::HeaderKeyContent,        b"Conten"                     ),
      ParserState::HeaderKeyContent       =>  self.header ( byte, b'-',   ParserState::HeaderKeyContentDash,    b"Content"                    ),
      //  ToDo: Match -Length, -Type, …
      ParserState::HeaderKeyContentDash   =>  self.header ( byte, b'L',   ParserState::HeaderKeyContentL,       b"Content-"                   ),
      ParserState::HeaderKeyContentL      =>  self.header ( byte, b'e',   ParserState::HeaderKeyContentLe,      b"Content-L"                  ),
      ParserState::HeaderKeyContentLe     =>  self.header ( byte, b'n',   ParserState::HeaderKeyContentLen,     b"Content-Le"                 ),
      ParserState::HeaderKeyContentLen    =>  self.header ( byte, b'g',   ParserState::HeaderKeyContentLeng,    b"Content-Len"                ),
      ParserState::HeaderKeyContentLeng   =>  self.header ( byte, b't',   ParserState::HeaderKeyContentLengt,   b"Content-Leng"               ),
      ParserState::HeaderKeyContentLengt  =>  self.header ( byte, b'h',   ParserState::HeaderKeyContentLength,  b"Content-Lengt"              ),
      ParserState::HeaderKeyContentLength
      =>  if  byte  ==  b':'
          {
            self.goto                   =   ParserState::HeaderValueParseNumber;
            Ok  ( ParserState::HeaderValueAwait )
          }
          else
          {
            self.auxiliary              =   b"Content-Length".to_vec ( );
            Ok  ( ParserState::HeaderKeyX       )
          },
      ParserState::HeaderKeyCoo           =>  self.header ( byte, b'k',   ParserState::HeaderKeyCook,           b"Coo"                        ),
      ParserState::HeaderKeyCook          =>  self.header ( byte, b'i',   ParserState::HeaderKeyCooki,          b"Cook"                       ),
      ParserState::HeaderKeyCooki         =>  self.header ( byte, b'e',   ParserState::HeaderKeyCookie,         b"Cooki"                      ),
      ParserState::HeaderKeyCookie
      =>  if  byte  ==  b':'
          {
            self.goto                   =   ParserState::HeaderValueParseCookieKey;
            Ok  ( ParserState::HeaderValueAwait )
          }
          else
          {
            self.auxiliary              =  b"Cookie".to_vec ( );
            Ok  ( ParserState::HeaderKeyX       )
          },
      ParserState::HeaderKeyX
      =>  match byte
          {
            b'\r'
            =>  {
                  self.reset            =   ParserState::Body;
                  Ok  ( ParserState::NewLineLF    )
                },
            b':'
            =>  self
                .aux2utf8 ( )
                .map_or
                (
                  Err ( ParserError::HeaderKey          ),
                  | key |
                  {
                    self.key          =   key;
                    self.goto         =   ParserState::HeaderValueX;
                    Ok  ( ParserState::HeaderValueAwait )
                  },
                ),
            _
            =>  if Self::isToken  ( byte  )
                {
                  self.auxiliary.push ( byte  );
                  Ok  ( ParserState::HeaderKeyX )
                }
                else
                {
                  Err ( ParserError::HeaderKey  )
                },
          },
      ParserState::HeaderValueAwait
      =>  match byte
          {
            b'\r'
            =>  {
                  self.reset            =   ParserState::HeaderValueAwait;
                  Ok  ( ParserState::NewLineLF      )
                },
            | b' '
            | b'\t'
            =>  Ok    ( ParserState::HeaderValueAwait ),
            | b'0'  ..= b'9'
            | b'A'  ..= b'Z'
            | b'a'  ..= b'z'
            | b'"'
            | b'*'
            //  more might be allowed…
            =>  {
                  self.byte             =   byte;
                  Ok  ( self.goto                   )
                },
            _   =>  Err ( ParserError::HeaderValue  ),
          },
      ParserState::HeaderValueParseCookieKey
      =>  if        byte  ==  b'='
          {
            self
            .aux2utf8 ( )
            .map_or
            (
              Err ( ParserError::HeaderValue  ),
              | key |
              {
                self.key                =   key;
                Ok  ( ParserState::HeaderValueParseCookieValue  )
              },
            )
          }
          else  if  Self::isToken  ( byte  )
          {
            self.auxiliary.push ( byte  );
            Ok  ( ParserState::HeaderValueParseCookieKey    )
          }
          else
          {
            Err ( ParserError::HeaderValue                )
          },
      ParserState::HeaderValueParseCookieQuote
      =>  if  byte  ==  b'"'
          {
            Ok  ( ParserState::HeaderValueParseCookieSemicolon  )
          }
          else
          {
            self.auxiliary.push ( byte  );
            Ok  ( ParserState::HeaderValueParseCookieQuote      )
          },
      ParserState::HeaderValueParseCookieSemicolon
      =>  self.expect ( byte, b';',   ParserState::HeaderValueParseCookieSpace, ParserError::HeaderValue, ),
      ParserState::HeaderValueParseCookieSpace
      =>  self.expectAnd
          (
            byte, b' ',
            | this |
            {
              this
              .aux2utf8 ( )
              .map_or
              (
                Err ( ParserError::HeaderValue  ),
                | value |
                {
                  let mut key           =   String::new ( );
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
                  Ok  ( ParserState::HeaderValueParseCookieValue  )
                }
              )
            },
            ParserError::HeaderValue,
          ),
      ParserState::HeaderValueParseCookieValue
      =>  match byte
          {
            b'"' if self.auxiliary.is_empty ( )
            =>  Ok    ( ParserState::HeaderValueParseCookieQuote  ),
            b'%'
            =>  {
                  self.reset            =   ParserState::HeaderValueParseCookieValue;
                  Ok  ( ParserState::EscapeX  )
                },
            b';'
            =>  Ok    ( ParserState::HeaderValueParseCookieSpace  ),
            | b'\0' ..= b' '
            | b'"'
            | b','
            | b'\\'
            =>  Err   ( ParserError::HeaderValue                ),
            _
            =>  {
                  self.auxiliary.push ( byte  );
                  Ok  ( ParserState::HeaderValueParseCookieValue  )
                },
          },
      ParserState::HeaderValueParseNumber
      =>  match byte
          {
            b'0'  ..= b'9'
            =>  {
                  self.value            =   10 * self.value + ( self.byte - b'0' ) as u64;
                  self.byte             =   byte;
                  Ok  ( ParserState::HeaderValueParseNumber )
                },
            b'\n'
            =>  {
                  let     value         =   10 * self.value + ( self.byte - b'0' ) as u64;
                  self.value            =   0;
                  if  self
                      .cache
                      .header
                      .setNumber
                      (
                        self.header,
                        value as  usize,
                      )
                      .is_ok ( )
                  {
                    self.reset          =   ParserState::HeaderKey;
                    Ok  ( ParserState::NewLineCR    )
                  }
                  else
                  {
                    Err ( ParserError::HeaderValue  )
                  }
                },
            _                           =>  Err ( ParserError::HeaderValue  ),
          },
      ParserState::HeaderValueX
      =>  if  byte  ==  b'\r'
          {
            self
            .aux2utf8 ( )
            .map_or
            (
              Err ( ParserError::HeaderValue          ),
              | value |
              {
                let mut key         =   String::new ( );
                mem::swap
                (
                  &mut key,
                  &mut self.key,
                );
                self
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
                self.reset              =   ParserState::HeaderKey;
                Ok  ( ParserState::NewLineLF          )
              }
            )
          }
          else
          {
            self.auxiliary.push ( byte  );
            Ok    ( ParserState::HeaderValueX )
          },
      ParserState::Method
      =>  match byte
          {
            b'C'                        =>  Ok  ( ParserState::MethodC      ),
            b'D'                        =>  Ok  ( ParserState::MethodD      ),
            b'G'                        =>  Ok  ( ParserState::MethodG      ),
            b'H'                        =>  Ok  ( ParserState::MethodH      ),
            b'O'                        =>  Ok  ( ParserState::MethodO      ),
            b'P'                        =>  Ok  ( ParserState::MethodP      ),
            b'T'                        =>  Ok  ( ParserState::MethodT      ),
            _                           =>  Err ( ParserError::Method       ),
          },
      ParserState::MethodC              =>  self.expect ( byte, b'O',   ParserState::MethodCO,                ParserError::Method,          ),
      ParserState::MethodCO             =>  self.expect ( byte, b'N',   ParserState::MethodCON,               ParserError::Method,          ),
      ParserState::MethodCON            =>  self.expect ( byte, b'N',   ParserState::MethodCONN,              ParserError::Method,          ),
      ParserState::MethodCONN           =>  self.expect ( byte, b'E',   ParserState::MethodCONNE,             ParserError::Method,          ),
      ParserState::MethodCONNE          =>  self.expect ( byte, b'C',   ParserState::MethodCONNEC,            ParserError::Method,          ),
      ParserState::MethodCONNEC         =>  self.method ( byte, b'T',   Method::Connect,                                                    ),
      ParserState::MethodD              =>  self.expect ( byte, b'E',   ParserState::MethodDE,                ParserError::Method,          ),
      ParserState::MethodDE             =>  self.expect ( byte, b'L',   ParserState::MethodDEL,               ParserError::Method,          ),
      ParserState::MethodDEL            =>  self.expect ( byte, b'E',   ParserState::MethodDELE,              ParserError::Method,          ),
      ParserState::MethodDELE           =>  self.expect ( byte, b'T',   ParserState::MethodDELET,             ParserError::Method,          ),
      ParserState::MethodDELET          =>  self.method ( byte, b'E',   Method::Delete,                                                     ),
      ParserState::MethodG              =>  self.expect ( byte, b'E',   ParserState::MethodGE,                ParserError::Method,          ),
      ParserState::MethodGE             =>  self.method ( byte, b'T',   Method::Get,                                                        ),
      ParserState::MethodH              =>  self.expect ( byte, b'E',   ParserState::MethodHE,                ParserError::Method,          ),
      ParserState::MethodHE             =>  self.expect ( byte, b'A',   ParserState::MethodHEA,               ParserError::Method,          ),
      ParserState::MethodHEA            =>  self.method ( byte, b'D',   Method::Head,                                                       ),
      ParserState::MethodO              =>  self.expect ( byte, b'P',   ParserState::MethodOP,                ParserError::Method,          ),
      ParserState::MethodOP             =>  self.expect ( byte, b'T',   ParserState::MethodOPT,               ParserError::Method,          ),
      ParserState::MethodOPT            =>  self.expect ( byte, b'I',   ParserState::MethodOPTI,              ParserError::Method,          ),
      ParserState::MethodOPTI           =>  self.expect ( byte, b'O',   ParserState::MethodOPTIO,             ParserError::Method,          ),
      ParserState::MethodOPTIO          =>  self.expect ( byte, b'N',   ParserState::MethodOPTION,            ParserError::Method,          ),
      ParserState::MethodOPTION         =>  self.method ( byte, b'S',   Method::Options,                                                    ),
      ParserState::MethodP
      =>  match byte
          {
            b'A'                        =>  Ok  ( ParserState::MethodPA ),
            b'O'                        =>  Ok  ( ParserState::MethodPO ),
            b'U'                        =>  Ok  ( ParserState::MethodPU ),
            _                           =>  Err ( ParserError::Method   ),
          },
      ParserState::MethodPA             =>  self.expect ( byte, b'T',   ParserState::MethodPAT,               ParserError::Method,          ),
      ParserState::MethodPAT            =>  self.expect ( byte, b'C',   ParserState::MethodPATC,              ParserError::Method,          ),
      ParserState::MethodPATC           =>  self.method ( byte, b'H',   Method::Patch,                                                      ),
      ParserState::MethodPO             =>  self.expect ( byte, b'S',   ParserState::MethodPOS,               ParserError::Method,          ),
      ParserState::MethodPOS            =>  self.method ( byte, b'T',   Method::Post,                                                       ),
      ParserState::MethodPU             =>  self.method ( byte, b'T',   Method::Put,                                                        ),
      ParserState::MethodT              =>  self.expect ( byte, b'R',   ParserState::MethodTR,                ParserError::Method,          ),
      ParserState::MethodTR             =>  self.expect ( byte, b'A',   ParserState::MethodTRA,               ParserError::Method,          ),
      ParserState::MethodTRA            =>  self.expect ( byte, b'C',   ParserState::MethodTRAC,              ParserError::Method,          ),
      ParserState::MethodTRAC           =>  self.method ( byte, b'E',   Method::Trace,                                                      ),
      ParserState::PathAwait            =>  self.expect ( byte, b' ',   ParserState::PathConsume,             ParserError::Method,          ),
      ParserState::PathConsume
      =>  match byte
          {
            b'%'
            =>  {
                  self.reset            =   self.state;
                  Ok  ( ParserState::EscapeX  )
                },
            | b'\r'
            | b' '
            | b'?'
            =>  self
                .aux2utf8 ( )
                .map_or
                (
                  Err ( ParserError::Path ),
                  | path  |
                  Ok
                  (
                    match byte
                    {
                      b'\r'
                      =>  {
                            self.reset  =   ParserState::HeaderKey;
                            ParserState::NewLineLF
                          },
                      b' '
                      =>  {
                            self
                            .cache
                            .path       =   path;
                            ParserState::Version
                          },
                      _
                      =>  {
                            self
                            .cache
                            .path       =   path;
                            ParserState::QueryKey
                          },
                    }
                  ),
                ),
            //  ToDo: Have another look at RFC 1945, 2396 and 2616
            | b'\0'
            | b'\n'
            =>  Err ( ParserError::Path ),
            _
            =>  {
                  self.auxiliary.push ( byte  );
                  Ok  ( ParserState::PathConsume  )
                },
          },
      ParserState::QueryKey
      =>  match byte
          {
            b'%'
            =>  {
                  self.reset            =   self.state;
                  Ok  ( ParserState::EscapeX  )
                },
            | b'\0'
            | b'\n'
            =>  Err ( ParserError::Query  ),
            _
            =>  if  byte  ==  self.querySeperator
                ||  byte  ==  b'\r'
                ||  byte  ==  b'='
                {
                  self
                  .aux2utf8 ( )
                  .map_or
                  (
                    Err ( ParserError::Query ),
                    | key |
                    Ok
                    (
                      match byte
                      {
                        b'\r'
                        =>  {
                              self
                              .reset    =   ParserState::HeaderKey;
                              ParserState::NewLineLF
                            },
                        b'='
                        =>  {
                              self.key  =   key;
                              ParserState::QueryValue
                            },
                        _
                        =>  {
                              self.cache.query.push
                              (
                                KeyValuePair
                                {
                                  key,
                                  value:    String::new ( ),
                                }
                              );
                              ParserState::QueryKey
                            },
                      }
                    ),
                  )
                }
                else
                {
                  self.auxiliary.push ( byte  );
                  Ok  ( ParserState::QueryKey   )
                },
          },
      ParserState::QueryValue
      =>  match byte
          {
            b'%'
            =>  {
                  self.reset            =   self.state;
                  Ok  ( ParserState::EscapeX  )
                },
            | b'\0'
            | b'\n'
            =>  Err ( ParserError::Query  ),
            _
            =>  if  byte  ==  self.querySeperator
                ||  byte  ==  b' '
                ||  byte  ==  b'\r'
                {
                  self
                  .aux2utf8 ( )
                  .map_or
                  (
                    Err ( ParserError::Query ),
                    | value |
                    {
                      let mut key       =   String::new ( );
                      mem::swap
                      (
                        &mut key,
                        &mut self.key,
                      );
                      self.cache.query.push
                      (
                        KeyValuePair
                        {
                          key,
                          value,
                        }
                      );
                      Ok
                      (
                        match byte
                        {
                          b'\r'
                          =>  {
                                self
                                .reset  =   ParserState::HeaderKey;
                                ParserState::NewLineLF
                              },
                          b' '          =>  ParserState::Version,
                          _             =>  ParserState::QueryKey,
                        }
                      )
                    },
                  )
                }
                else
                {
                  self.auxiliary.push ( byte  );
                  Ok  ( ParserState::QueryKey   )
                },
          },
      ParserState::Version              =>  self.expect ( byte, b'T',   ParserState::VersionH,                ParserError::Version,         ),
      ParserState::VersionH             =>  self.expect ( byte, b'T',   ParserState::VersionHT,               ParserError::Version,         ),
      ParserState::VersionHT            =>  self.expect ( byte, b'T',   ParserState::VersionHTT,              ParserError::Version,         ),
      ParserState::VersionHTT           =>  self.expect ( byte, b'P',   ParserState::VersionHTTP,             ParserError::Version,         ),
      ParserState::VersionHTTP          =>  self.expect ( byte, b'/',   ParserState::VersionNumber,           ParserError::Version,         ),
      ParserState::VersionNumber
      =>  match byte
          {
            b'0'                        =>  Ok  ( ParserState::Version0 ),
            b'1'                        =>  Ok  ( ParserState::Version1 ),
            b'2'                        =>  Ok  ( ParserState::Version2 ),
            b'3'                        =>  Ok  ( ParserState::Version3 ),
            _                           =>  Err ( ParserError::Version  ),
          },
      ParserState::Version0             =>  self.expect ( byte, b'.',   ParserState::Version0P,               ParserError::Version,         ),
      ParserState::Version0P            =>  self.version( byte, b'9',   Version::HTTP_09,                                                   ),
      ParserState::Version1             =>  self.expect ( byte, b'.',   ParserState::Version1P,               ParserError::Version,         ),
      ParserState::Version1P
      =>  match byte
          {
            b'0'
            =>  {
                  self.cache.version    =   Version::HTTP_10;
                  self.reset            =   ParserState::HeaderKey;
                  Ok  ( ParserState::NewLineCR  )
                },
            b'1'
            =>  {
                  self.cache.version    =   Version::HTTP_11;
                  self.reset            =   ParserState::HeaderKey;
                  Ok  ( ParserState::NewLineCR  )
                },
            _
            =>  Err ( ParserError::Version  ),
          },
      ParserState::Version2             =>  self.expect ( byte, b'.',   ParserState::Version2P,               ParserError::Version,         ),
      ParserState::Version2P            =>  self.version( byte, b'0',   Version::HTTP_2,                                                    ),
      ParserState::Version3             =>  self.expect ( byte, b'.',   ParserState::Version3P,               ParserError::Version,         ),
      ParserState::Version3P            =>  self.version( byte, b'0',   Version::HTTP_3,                                                    ),
    }
  }
}
