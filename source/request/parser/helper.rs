use
{
  super::
  {
    Parser,
    ParserError,
    ParserState,
    Request,
    super::
    {
      RequestHeaderField,
      super::
      {
        method::Method,
        version::Version,
      },
    },
  },
  std::
  {
    mem,
    string::
    {
      FromUtf8Error,
    },
  },
};

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

  /// Go to `state` and then to `reset`.
  ///
  /// # Arguments
  /// `state`                           – go to this state and
  /// `reset`                           – then to this state.
  pub ( super )
  fn            call
  (
    &mut self,
    state:                              ParserState,
    reset:                              ParserState,
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    self.reset                          =   reset;
    Ok  ( state )
  }

  /// Enque `Request` and prepare parsing of next `Request`.
  pub ( super )
  fn            done
  (
    &mut self,
  )
  ->  ParserState
  {
    let mut request                     =   Request ( );
    mem::swap
    (
      &mut request,
      &mut self.cache,
    );
    self.requests.push  ( request );
    ParserState::Method
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
  fn            expect
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
  fn            expectAnd               < F >
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
  fn            header
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
      self.auxiliary.push ( input );
      Ok  ( ParserState::HeaderKeyX  )
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

  /// Helper to Parse the Request Method.
  /// If `input` and `expected` are equal,
  ///   set the method of the request and go to await the Request Path.
  /// Otherwise raise the Method Error.
  ///
  /// # Arguments
  /// * `input`                         –   left value of comparison,
  /// * `expected`                      –   right value of comparison,
  /// * `method`                        –   set method on success.
  pub ( super )
  fn            method
  (
    &mut self,
    input:                              u8,
    expected:                           u8,
    method:                             Method,
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    if  input ==  expected
    {
      self.cache.method                 =   method;
      Ok  ( ParserState::PathAwait  )
    }
    else
    {
      Err ( ParserError::Method     )
    }
  }

  /// Helper to Parse Header Value.
  /// Failes, if `input` and `expected` are not equal and `input` is invalid in header key.
  /// If `input` and `expected` are just not equal, but valid, this function sets the
  /// `auxiliary`-buffer to the `key` and returns ParserState::HeaderKeyX successfully.
  ///
  /// # Arguments
  /// * `byte`                          –   input character,
  /// * `goto`                          –   go here after awaiting value,
  /// * `key`                           –   key on failure.
  pub ( super )
  fn            parse
  (
    &mut self,
    byte:                               u8,
    goto:                               ParserState,
    key:                                &[  u8  ],
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    if  byte  ==  b':'
    {
      self.goto                         =   goto;
      Ok  ( ParserState::HeaderValueAwait )
    }
    else
    {
      self.auxiliary                    =  key.to_vec ( );
      self.auxiliary.push ( byte  );
      Ok  ( ParserState::HeaderKeyX       )
    }
  }

  /// Helper to Parse Header Value of type Number.
  /// Failes, if `input` and `expected` are not equal and `input` is invalid in header key.
  /// If `input` and `expected` are just not equal, but valid, this function sets the
  /// `auxiliary`-buffer to the `key` and returns ParserState::HeaderKeyX successfully.
  ///
  /// # Arguments
  /// * `byte`                          –   input character,
  /// * `field`                         –   set this header field,
  /// * `key`                           –   key on failure.
  pub ( super )
  fn            parseNum
  (
    &mut self,
    byte:                               u8,
    field:                              RequestHeaderField,
    key:                                &[  u8  ],
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    self.header                         =   field;
    self.parse  ( byte, ParserState::HeaderValueParseNumber,  key,  )
  }

  /// Helper to Parse Header Value of type String.
  /// Failes, if `input` and `expected` are not equal and `input` is invalid in header key.
  /// If `input` and `expected` are just not equal, but valid, this function sets the
  /// `auxiliary`-buffer to the `key` and returns ParserState::HeaderKeyX successfully.
  ///
  /// # Arguments
  /// * `byte`                          –   input character,
  /// * `field`                         –   set this header field,
  /// * `key`                           –   key on failure.
  pub ( super )
  fn            parseStr
  (
    &mut self,
    byte:                               u8,
    field:                              RequestHeaderField,
    key:                                &[  u8  ],
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    self.header                         =   field;
    self.parse  ( byte, ParserState::HeaderValueParseString,  key,  )
  }

  /// Helper to Parse the Request Version.
  /// If `input` and `expected` are equal,
  ///   set the method of the request and go to await the Request Path.
  /// Otherwise raise the Method Error.
  ///
  /// # Arguments
  /// * `input`                         –   left value of comparison,
  /// * `expected`                      –   right value of comparison,
  /// * `version`                       –   set version on success.
  pub ( super )
  fn            version
  (
    &mut self,
    input:                              u8,
    expected:                           u8,
    version:                            Version,
  )
  ->  Result
      <
        ParserState,
        ParserError,
      >
  {
    if  input ==  expected
    {
      self.cache.version                =   version;
      self.reset                        =   ParserState::HeaderKey;
      Ok  ( ParserState::NewLineCR      )
    }
    else
    {
      Err ( ParserError::Version     )
    }
  }
}
