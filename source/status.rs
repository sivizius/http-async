use
{
  std::
  {
    fmt::
    {
      self,
      Display,
    },
  },
};

/// Status Code of Response.
#[allow(dead_code)]
pub enum      Status
{
  /// 100 Continue
  Continue,
  /// 101 Switching Protocols
  SwitchingProtocols,
  /// 102 Processing
  Processing,
  /// 103 Early Hints
  EarlyHints,
  /// 200 Ok
  Ok,
  /// 201 Created
  Created,
  /// 202 Accepted
  Accepted,
  /// 203 Non-Authoritative Information
  NonAuthoritativeInformation,
  /// 204 No Content
  NoContent,
  /// 205 Reset Content
  ResetContent,
  /// 206 Partial Content
  PartialContent,
  /// 207 Multi-Status
  MultiStatus,
  /// 208 Already Reported
  AlreadyReported,
  /// 226 IM Used
  IMused,
  /// 404 Not Found
  NotFound,
  /*
  300,Multiple Choices,"[RFC7231, Section 6.4.1]"
  301,Moved Permanently,"[RFC7231, Section 6.4.2]"
  302,Found,"[RFC7231, Section 6.4.3]"
  303,See Other,"[RFC7231, Section 6.4.4]"
  304,Not Modified,"[RFC7232, Section 4.1]"
  305,Use Proxy,"[RFC7231, Section 6.4.5]"
  306,(Unused),"[RFC7231, Section 6.4.6]"
  307,Temporary Redirect,"[RFC7231, Section 6.4.7]"
  308,Permanent Redirect,[RFC7538]
  400,Bad Request,"[RFC7231, Section 6.5.1]"
  401,Unauthorized,"[RFC7235, Section 3.1]"
  402,Payment Required,"[RFC7231, Section 6.5.2]"
  403,Forbidden,"[RFC7231, Section 6.5.3]"
  404,Not Found,"[RFC7231, Section 6.5.4]"
  405,Method Not Allowed,"[RFC7231, Section 6.5.5]"
  406,Not Acceptable,"[RFC7231, Section 6.5.6]"
  407,Proxy Authentication Required,"[RFC7235, Section 3.2]"
  408,Request Timeout,"[RFC7231, Section 6.5.7]"
  409,Conflict,"[RFC7231, Section 6.5.8]"
  410,Gone,"[RFC7231, Section 6.5.9]"
  411,Length Required,"[RFC7231, Section 6.5.10]"
  412,Precondition Failed,"[RFC7232, Section 4.2][RFC8144, Section 3.2]"
  413,Payload Too Large,"[RFC7231, Section 6.5.11]"
  414,URI Too Long,"[RFC7231, Section 6.5.12]"
  415,Unsupported Media Type,"[RFC7231, Section 6.5.13][RFC7694, Section 3]"
  416,Range Not Satisfiable,"[RFC7233, Section 4.4]"
  417,Expectation Failed,"[RFC7231, Section 6.5.14]"
  421,Misdirected Request,"[RFC7540, Section 9.1.2]"
  422,Unprocessable Entity,[RFC4918]
  423,Locked,[RFC4918]
  424,Failed Dependency,[RFC4918]
  425,Too Early,[RFC8470]
  426,Upgrade Required,"[RFC7231, Section 6.5.15]"
  428,Precondition Required,[RFC6585]
  429,Too Many Requests,[RFC6585]
  431,Request Header Fields Too Large,[RFC6585]
  451,Unavailable For Legal Reasons,[RFC7725]
  500,Internal Server Error,"[RFC7231, Section 6.6.1]"
  501,Not Implemented,"[RFC7231, Section 6.6.2]"
  502,Bad Gateway,"[RFC7231, Section 6.6.3]"
  503,Service Unavailable,"[RFC7231, Section 6.6.4]"
  504,Gateway Timeout,"[RFC7231, Section 6.6.5]"
  505,HTTP Version Not Supported,"[RFC7231, Section 6.6.6]"
  506,Variant Also Negotiates,[RFC2295]
  507,Insufficient Storage,[RFC4918]
  508,Loop Detected,[RFC5842]
  510,Not Extended,[RFC2774]
  511,Network Authentication Required,[RFC6585]
  */
}

impl          Display                   for Status
{
  fn fmt
  (
    &self,
    formatter:                          &mut fmt::Formatter<'_>
  )
  ->  fmt::Result
  {
    formatter
      .write_str
      (
        &format!
        (
          "HTTP/{}",
          match self
          {
            Status::Ok                  =>  "200 Ok",
            _                           =>  "??? ?????",
          }
        )
      )
  }
}
