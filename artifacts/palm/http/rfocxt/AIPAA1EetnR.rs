use crate::byte_str::ByteStr;
use bytes::{Bytes, BytesMut};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::str::FromStr;
#[rustfmt::skip]
const HEADER_CHARS: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, b'!', 0, b'#', b'$', b'%', b'&', b'\'', 0, 0, b'*', b'+', 0, b'-',
    b'.', 0, b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', 0, 0, 0, 0, 0,
    0, 0, b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm',
    b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', 0, 0,
    0, b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
    b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
    b'y', b'z', 0, b'|', 0, b'~', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
#[rustfmt::skip]
const HEADER_CHARS_H2: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, b'!', b'"', b'#', b'$', b'%', b'&', b'\'', 0, 0, b'*', b'+', 0, b'-',
    b'.', 0, b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
    b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
    b'x', b'y', b'z', 0, b'|', 0, b'~', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const SCRATCH_BUF_SIZE: usize = 64;
const SCRATCH_BUF_OVERFLOW: usize = SCRATCH_BUF_SIZE + 1;
macro_rules! standard_headers {
    ($($(#[$docs:meta])* ($konst:ident, $upcase:ident, $name_bytes:literal);)+) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)] enum StandardHeader {
        $($konst,)+ } $($(#[$docs])* pub const $upcase : HeaderName = HeaderName { inner
        : Repr::Standard(StandardHeader::$konst), };)+ impl StandardHeader { #[inline] fn
        as_str(& self) -> &'static str { match * self { $(StandardHeader::$konst =>
        unsafe { std::str::from_utf8_unchecked($name_bytes) },)+ } } const fn
        from_bytes(name_bytes : & [u8]) -> Option < StandardHeader > { match name_bytes {
        $($name_bytes => Some(StandardHeader::$konst),)+ _ => None, } } } #[cfg(test)]
        const TEST_HEADERS : &'static [(StandardHeader, &'static [u8])] = &
        [$((StandardHeader::$konst, $name_bytes),)+];
    };
}
standard_headers! {
    #[doc = " Advertises which content types the client is able to understand."] #[doc =
    ""] #[doc =
    " The Accept request HTTP header advertises which content types, expressed"] #[doc =
    " as MIME types, the client is able to understand. Using content"] #[doc =
    " negotiation, the server then selects one of the proposals, uses it and"] #[doc =
    " informs the client of its choice with the Content-Type response header."] #[doc =
    " Browsers set adequate values for this header depending of the context"] #[doc =
    " where the request is done: when fetching a CSS stylesheet a different"] #[doc =
    " value is set for the request than when fetching an image, video or a"] #[doc =
    " script."] (Accept, ACCEPT, b"accept"); #[doc =
    " Advertises which character set the client is able to understand."] #[doc = ""]
    #[doc = " The Accept-Charset request HTTP header advertises which character set"]
    #[doc = " the client is able to understand. Using content negotiation, the server"]
    #[doc = " then selects one of the proposals, uses it and informs the client of its"]
    #[doc = " choice within the Content-Type response header. Browsers usually don't"]
    #[doc = " set this header as the default value for each content type is usually"]
    #[doc = " correct and transmitting it would allow easier fingerprinting."] #[doc =
    ""] #[doc = " If the server cannot serve any matching character set, it can"] #[doc =
    " theoretically send back a 406 (Not Acceptable) error code. But, for a"] #[doc =
    " better user experience, this is rarely done and the more common way is"] #[doc =
    " to ignore the Accept-Charset header in this case."] (AcceptCharset, ACCEPT_CHARSET,
    b"accept-charset"); #[doc =
    " Advertises which content encoding the client is able to understand."] #[doc = ""]
    #[doc = " The Accept-Encoding request HTTP header advertises which content"] #[doc =
    " encoding, usually a compression algorithm, the client is able to"] #[doc =
    " understand. Using content negotiation, the server selects one of the"] #[doc =
    " proposals, uses it and informs the client of its choice with the"] #[doc =
    " Content-Encoding response header."] #[doc = ""] #[doc =
    " Even if both the client and the server supports the same compression"] #[doc =
    " algorithms, the server may choose not to compress the body of a"] #[doc =
    " response, if the identity value is also acceptable. Two common cases"] #[doc =
    " lead to this:"] #[doc = ""] #[doc =
    " * The data to be sent is already compressed and a second compression"] #[doc =
    " won't lead to smaller data to be transmitted. This may the case with"] #[doc =
    " some image formats;"] #[doc = ""] #[doc =
    " * The server is overloaded and cannot afford the computational overhead"] #[doc =
    " induced by the compression requirement. Typically, Microsoft recommends"] #[doc =
    " not to compress if a server use more than 80 % of its computational"] #[doc =
    " power."] #[doc = ""] #[doc =
    " As long as the identity value, meaning no compression, is not explicitly"] #[doc =
    " forbidden, by an identity;q=0 or a *;q=0 without another explicitly set"] #[doc =
    " value for identity, the server must never send back a 406 Not Acceptable"] #[doc =
    " error."] (AcceptEncoding, ACCEPT_ENCODING, b"accept-encoding"); #[doc =
    " Advertises which languages the client is able to understand."] #[doc = ""] #[doc =
    " The Accept-Language request HTTP header advertises which languages the"] #[doc =
    " client is able to understand, and which locale variant is preferred."] #[doc =
    " Using content negotiation, the server then selects one of the proposals,"] #[doc =
    " uses it and informs the client of its choice with the Content-Language"] #[doc =
    " response header. Browsers set adequate values for this header according"] #[doc =
    " their user interface language and even if a user can change it, this"] #[doc =
    " happens rarely (and is frown upon as it leads to fingerprinting)."] #[doc = ""]
    #[doc = " This header is a hint to be used when the server has no way of"] #[doc =
    " determining the language via another way, like a specific URL, that is"] #[doc =
    " controlled by an explicit user decision. It is recommended that the"] #[doc =
    " server never overrides an explicit decision. The content of the"] #[doc =
    " Accept-Language is often out of the control of the user (like when"] #[doc =
    " traveling and using an Internet Cafe in a different country); the user"] #[doc =
    " may also want to visit a page in another language than the locale of"] #[doc =
    " their user interface."] #[doc = ""] #[doc =
    " If the server cannot serve any matching language, it can theoretically"] #[doc =
    " send back a 406 (Not Acceptable) error code. But, for a better user"] #[doc =
    " experience, this is rarely done and more common way is to ignore the"] #[doc =
    " Accept-Language header in this case."] (AcceptLanguage, ACCEPT_LANGUAGE,
    b"accept-language"); #[doc =
    " Marker used by the server to advertise partial request support."] #[doc = ""] #[doc
    = " The Accept-Ranges response HTTP header is a marker used by the server to"] #[doc
    = " advertise its support of partial requests. The value of this field"] #[doc =
    " indicates the unit that can be used to define a range."] #[doc = ""] #[doc =
    " In presence of an Accept-Ranges header, the browser may try to resume an"] #[doc =
    " interrupted download, rather than to start it from the start again."]
    (AcceptRanges, ACCEPT_RANGES, b"accept-ranges"); #[doc =
    " Preflight response indicating if the response to the request can be"] #[doc =
    " exposed to the page."] #[doc = ""] #[doc =
    " The Access-Control-Allow-Credentials response header indicates whether"] #[doc =
    " or not the response to the request can be exposed to the page. It can be"] #[doc =
    " exposed when the true value is returned; it can't in other cases."] #[doc = ""]
    #[doc = " Credentials are cookies, authorization headers or TLS client"] #[doc =
    " certificates."] #[doc = ""] #[doc =
    " When used as part of a response to a preflight request, this indicates"] #[doc =
    " whether or not the actual request can be made using credentials. Note"] #[doc =
    " that simple GET requests are not preflighted, and so if a request is"] #[doc =
    " made for a resource with credentials, if this header is not returned"] #[doc =
    " with the resource, the response is ignored by the browser and not"] #[doc =
    " returned to web content."] #[doc = ""] #[doc =
    " The Access-Control-Allow-Credentials header works in conjunction with"] #[doc =
    " the XMLHttpRequest.withCredentials property or with the credentials"] #[doc =
    " option in the Request() constructor of the Fetch API. Credentials must"] #[doc =
    " be set on both sides (the Access-Control-Allow-Credentials header and in"] #[doc =
    " the XHR or Fetch request) in order for the CORS request with credentials"] #[doc =
    " to succeed."] (AccessControlAllowCredentials, ACCESS_CONTROL_ALLOW_CREDENTIALS,
    b"access-control-allow-credentials"); #[doc =
    " Preflight response indicating permitted HTTP headers."] #[doc = ""] #[doc =
    " The Access-Control-Allow-Headers response header is used in response to"] #[doc =
    " a preflight request to indicate which HTTP headers will be available via"] #[doc =
    " Access-Control-Expose-Headers when making the actual request."] #[doc = ""] #[doc =
    " The simple headers, Accept, Accept-Language, Content-Language,"] #[doc =
    " Content-Type (but only with a MIME type of its parsed value (ignoring"] #[doc =
    " parameters) of either application/x-www-form-urlencoded,"] #[doc =
    " multipart/form-data, or text/plain), are always available and don't need"] #[doc =
    " to be listed by this header."] #[doc = ""] #[doc =
    " This header is required if the request has an"] #[doc =
    " Access-Control-Request-Headers header."] (AccessControlAllowHeaders,
    ACCESS_CONTROL_ALLOW_HEADERS, b"access-control-allow-headers"); #[doc =
    " Preflight header response indicating permitted access methods."] #[doc = ""] #[doc
    = " The Access-Control-Allow-Methods response header specifies the method or"] #[doc
    = " methods allowed when accessing the resource in response to a preflight"] #[doc =
    " request."] (AccessControlAllowMethods, ACCESS_CONTROL_ALLOW_METHODS,
    b"access-control-allow-methods"); #[doc =
    " Indicates whether the response can be shared with resources with the"] #[doc =
    " given origin."] (AccessControlAllowOrigin, ACCESS_CONTROL_ALLOW_ORIGIN,
    b"access-control-allow-origin"); #[doc =
    " Indicates which headers can be exposed as part of the response by"] #[doc =
    " listing their names."] (AccessControlExposeHeaders, ACCESS_CONTROL_EXPOSE_HEADERS,
    b"access-control-expose-headers"); #[doc =
    " Indicates how long the results of a preflight request can be cached."]
    (AccessControlMaxAge, ACCESS_CONTROL_MAX_AGE, b"access-control-max-age"); #[doc =
    " Informs the server which HTTP headers will be used when an actual"] #[doc =
    " request is made."] (AccessControlRequestHeaders, ACCESS_CONTROL_REQUEST_HEADERS,
    b"access-control-request-headers"); #[doc =
    " Informs the server know which HTTP method will be used when the actual"] #[doc =
    " request is made."] (AccessControlRequestMethod, ACCESS_CONTROL_REQUEST_METHOD,
    b"access-control-request-method"); #[doc =
    " Indicates the time in seconds the object has been in a proxy cache."] #[doc = ""]
    #[doc = " The Age header is usually close to zero. If it is Age: 0, it was"] #[doc =
    " probably just fetched from the origin server; otherwise It is usually"] #[doc =
    " calculated as a difference between the proxy's current date and the Date"] #[doc =
    " general header included in the HTTP response."] (Age, AGE, b"age"); #[doc =
    " Lists the set of methods support by a resource."] #[doc = ""] #[doc =
    " This header must be sent if the server responds with a 405 Method Not"] #[doc =
    " Allowed status code to indicate which request methods can be used. An"] #[doc =
    " empty Allow header indicates that the resource allows no request"] #[doc =
    " methods, which might occur temporarily for a given resource, for"] #[doc =
    " example."] (Allow, ALLOW, b"allow"); #[doc =
    " Advertises the availability of alternate services to clients."] (AltSvc, ALT_SVC,
    b"alt-svc"); #[doc =
    " Contains the credentials to authenticate a user agent with a server."] #[doc = ""]
    #[doc = " Usually this header is included after the server has responded with a"]
    #[doc = " 401 Unauthorized status and the WWW-Authenticate header."] (Authorization,
    AUTHORIZATION, b"authorization"); #[doc =
    " Specifies directives for caching mechanisms in both requests and"] #[doc =
    " responses."] #[doc = ""] #[doc =
    " Caching directives are unidirectional, meaning that a given directive in"] #[doc =
    " a request is not implying that the same directive is to be given in the"] #[doc =
    " response."] (CacheControl, CACHE_CONTROL, b"cache-control"); #[doc =
    " Indicates how caches have handled a response and its corresponding request."] #[doc
    = ""] #[doc = " See [RFC 9211](https://www.rfc-editor.org/rfc/rfc9211.html)."]
    (CacheStatus, CACHE_STATUS, b"cache-status"); #[doc =
    " Specifies directives that allow origin servers to control the behavior of CDN caches"]
    #[doc =
    " interposed between them and clients separately from other caches that might handle the"]
    #[doc = " response."] #[doc = ""] #[doc =
    " See [RFC 9213](https://www.rfc-editor.org/rfc/rfc9213.html)."] (CdnCacheControl,
    CDN_CACHE_CONTROL, b"cdn-cache-control"); #[doc =
    " Controls whether or not the network connection stays open after the"] #[doc =
    " current transaction finishes."] #[doc = ""] #[doc =
    " If the value sent is keep-alive, the connection is persistent and not"] #[doc =
    " closed, allowing for subsequent requests to the same server to be done."] #[doc =
    ""] #[doc = " Except for the standard hop-by-hop headers (Keep-Alive,"] #[doc =
    " Transfer-Encoding, TE, Connection, Trailer, Upgrade, Proxy-Authorization"] #[doc =
    " and Proxy-Authenticate), any hop-by-hop headers used by the message must"] #[doc =
    " be listed in the Connection header, so that the first proxy knows he has"] #[doc =
    " to consume them and not to forward them further. Standard hop-by-hop"] #[doc =
    " headers can be listed too (it is often the case of Keep-Alive, but this"] #[doc =
    " is not mandatory."] (Connection, CONNECTION, b"connection"); #[doc =
    " Indicates if the content is expected to be displayed inline."] #[doc = ""] #[doc =
    " In a regular HTTP response, the Content-Disposition response header is a"] #[doc =
    " header indicating if the content is expected to be displayed inline in"] #[doc =
    " the browser, that is, as a Web page or as part of a Web page, or as an"] #[doc =
    " attachment, that is downloaded and saved locally."] #[doc = ""] #[doc =
    " In a multipart/form-data body, the HTTP Content-Disposition general"] #[doc =
    " header is a header that can be used on the subpart of a multipart body"] #[doc =
    " to give information about the field it applies to. The subpart is"] #[doc =
    " delimited by the boundary defined in the Content-Type header. Used on"] #[doc =
    " the body itself, Content-Disposition has no effect."] #[doc = ""] #[doc =
    " The Content-Disposition header is defined in the larger context of MIME"] #[doc =
    " messages for e-mail, but only a subset of the possible parameters apply"] #[doc =
    " to HTTP forms and POST requests. Only the value form-data, as well as"] #[doc =
    " the optional directive name and filename, can be used in the HTTP"] #[doc =
    " context."] (ContentDisposition, CONTENT_DISPOSITION, b"content-disposition"); #[doc
    = " Used to compress the media-type."] #[doc = ""] #[doc =
    " When present, its value indicates what additional content encoding has"] #[doc =
    " been applied to the entity-body. It lets the client know, how to decode"] #[doc =
    " in order to obtain the media-type referenced by the Content-Type header."] #[doc =
    ""] #[doc =
    " It is recommended to compress data as much as possible and therefore to"] #[doc =
    " use this field, but some types of resources, like jpeg images, are"] #[doc =
    " already compressed.  Sometimes using additional compression doesn't"] #[doc =
    " reduce payload size and can even make the payload longer."] (ContentEncoding,
    CONTENT_ENCODING, b"content-encoding"); #[doc =
    " Used to describe the languages intended for the audience."] #[doc = ""] #[doc =
    " This header allows a user to differentiate according to the users' own"] #[doc =
    " preferred language. For example, if \"Content-Language: de-DE\" is set, it"] #[doc
    = " says that the document is intended for German language speakers"] #[doc =
    " (however, it doesn't indicate the document is written in German. For"] #[doc =
    " example, it might be written in English as part of a language course for"] #[doc =
    " German speakers)."] #[doc = ""] #[doc =
    " If no Content-Language is specified, the default is that the content is"] #[doc =
    " intended for all language audiences. Multiple language tags are also"] #[doc =
    " possible, as well as applying the Content-Language header to various"] #[doc =
    " media types and not only to textual documents."] (ContentLanguage,
    CONTENT_LANGUAGE, b"content-language"); #[doc =
    " Indicates the size of the entity-body."] #[doc = ""] #[doc =
    " The header value must be a decimal indicating the number of octets sent"] #[doc =
    " to the recipient."] (ContentLength, CONTENT_LENGTH, b"content-length"); #[doc =
    " Indicates an alternate location for the returned data."] #[doc = ""] #[doc =
    " The principal use case is to indicate the URL of the resource"] #[doc =
    " transmitted as the result of content negotiation."] #[doc = ""] #[doc =
    " Location and Content-Location are different: Location indicates the"] #[doc =
    " target of a redirection (or the URL of a newly created document), while"] #[doc =
    " Content-Location indicates the direct URL to use to access the resource,"] #[doc =
    " without the need of further content negotiation. Location is a header"] #[doc =
    " associated with the response, while Content-Location is associated with"] #[doc =
    " the entity returned."] (ContentLocation, CONTENT_LOCATION, b"content-location");
    #[doc = " Indicates where in a full body message a partial message belongs."]
    (ContentRange, CONTENT_RANGE, b"content-range"); #[doc =
    " Allows controlling resources the user agent is allowed to load for a"] #[doc =
    " given page."] #[doc = ""] #[doc =
    " With a few exceptions, policies mostly involve specifying server origins"] #[doc =
    " and script endpoints. This helps guard against cross-site scripting"] #[doc =
    " attacks (XSS)."] (ContentSecurityPolicy, CONTENT_SECURITY_POLICY,
    b"content-security-policy"); #[doc =
    " Allows experimenting with policies by monitoring their effects."] #[doc = ""] #[doc
    = " The HTTP Content-Security-Policy-Report-Only response header allows web"] #[doc =
    " developers to experiment with policies by monitoring (but not enforcing)"] #[doc =
    " their effects. These violation reports consist of JSON documents sent"] #[doc =
    " via an HTTP POST request to the specified URI."] (ContentSecurityPolicyReportOnly,
    CONTENT_SECURITY_POLICY_REPORT_ONLY, b"content-security-policy-report-only"); #[doc =
    " Used to indicate the media type of the resource."] #[doc = ""] #[doc =
    " In responses, a Content-Type header tells the client what the content"] #[doc =
    " type of the returned content actually is. Browsers will do MIME sniffing"] #[doc =
    " in some cases and will not necessarily follow the value of this header;"] #[doc =
    " to prevent this behavior, the header X-Content-Type-Options can be set"] #[doc =
    " to nosniff."] #[doc = ""] #[doc =
    " In requests, (such as POST or PUT), the client tells the server what"] #[doc =
    " type of data is actually sent."] (ContentType, CONTENT_TYPE, b"content-type");
    #[doc = " Contains stored HTTP cookies previously sent by the server with the"] #[doc
    = " Set-Cookie header."] #[doc = ""] #[doc =
    " The Cookie header might be omitted entirely, if the privacy setting of"] #[doc =
    " the browser are set to block them, for example."] (Cookie, COOKIE, b"cookie");
    #[doc = " Indicates the client's tracking preference."] #[doc = ""] #[doc =
    " This header lets users indicate whether they would prefer privacy rather"] #[doc =
    " than personalized content."] (Dnt, DNT, b"dnt"); #[doc =
    " Contains the date and time at which the message was originated."] (Date, DATE,
    b"date"); #[doc = " Identifier for a specific version of a resource."] #[doc = ""]
    #[doc = " This header allows caches to be more efficient, and saves bandwidth, as"]
    #[doc = " a web server does not need to send a full response if the content has"]
    #[doc = " not changed. On the other side, if the content has changed, etags are"]
    #[doc = " useful to help prevent simultaneous updates of a resource from"] #[doc =
    " overwriting each other (\"mid-air collisions\")."] #[doc = ""] #[doc =
    " If the resource at a given URL changes, a new Etag value must be"] #[doc =
    " generated. Etags are therefore similar to fingerprints and might also be"] #[doc =
    " used for tracking purposes by some servers. A comparison of them allows"] #[doc =
    " to quickly determine whether two representations of a resource are the"] #[doc =
    " same, but they might also be set to persist indefinitely by a tracking"] #[doc =
    " server."] (Etag, ETAG, b"etag"); #[doc =
    " Indicates expectations that need to be fulfilled by the server in order"] #[doc =
    " to properly handle the request."] #[doc = ""] #[doc =
    " The only expectation defined in the specification is Expect:"] #[doc =
    " 100-continue, to which the server shall respond with:"] #[doc = ""] #[doc =
    " * 100 if the information contained in the header is sufficient to cause"] #[doc =
    " an immediate success,"] #[doc = ""] #[doc =
    " * 417 (Expectation Failed) if it cannot meet the expectation; or any"] #[doc =
    " other 4xx status otherwise."] #[doc = ""] #[doc =
    " For example, the server may reject a request if its Content-Length is"] #[doc =
    " too large."] #[doc = ""] #[doc =
    " No common browsers send the Expect header, but some other clients such"] #[doc =
    " as cURL do so by default."] (Expect, EXPECT, b"expect"); #[doc =
    " Contains the date/time after which the response is considered stale."] #[doc = ""]
    #[doc = " Invalid dates, like the value 0, represent a date in the past and mean"]
    #[doc = " that the resource is already expired."] #[doc = ""] #[doc =
    " If there is a Cache-Control header with the \"max-age\" or \"s-max-age\""] #[doc =
    " directive in the response, the Expires header is ignored."] (Expires, EXPIRES,
    b"expires"); #[doc =
    " Contains information from the client-facing side of proxy servers that"] #[doc =
    " is altered or lost when a proxy is involved in the path of the request."] #[doc =
    ""] #[doc = " The alternative and de-facto standard versions of this header are the"]
    #[doc = " X-Forwarded-For, X-Forwarded-Host and X-Forwarded-Proto headers."] #[doc =
    ""] #[doc = " This header is used for debugging, statistics, and generating"] #[doc =
    " location-dependent content and by design it exposes privacy sensitive"] #[doc =
    " information, such as the IP address of the client. Therefore the user's"] #[doc =
    " privacy must be kept in mind when deploying this header."] (Forwarded, FORWARDED,
    b"forwarded"); #[doc =
    " Contains an Internet email address for a human user who controls the"] #[doc =
    " requesting user agent."] #[doc = ""] #[doc =
    " If you are running a robotic user agent (e.g. a crawler), the From"] #[doc =
    " header should be sent, so you can be contacted if problems occur on"] #[doc =
    " servers, such as if the robot is sending excessive, unwanted, or invalid"] #[doc =
    " requests."] (From, FROM, b"from"); #[doc =
    " Specifies the domain name of the server and (optionally) the TCP port"] #[doc =
    " number on which the server is listening."] #[doc = ""] #[doc =
    " If no port is given, the default port for the service requested (e.g.,"] #[doc =
    " \"80\" for an HTTP URL) is implied."] #[doc = ""] #[doc =
    " A Host header field must be sent in all HTTP/1.1 request messages. A 400"] #[doc =
    " (Bad Request) status code will be sent to any HTTP/1.1 request message"] #[doc =
    " that lacks a Host header field or contains more than one."] (Host, HOST, b"host");
    #[doc = " Makes a request conditional based on the E-Tag."] #[doc = ""] #[doc =
    " For GET and HEAD methods, the server will send back the requested"] #[doc =
    " resource only if it matches one of the listed ETags. For PUT and other"] #[doc =
    " non-safe methods, it will only upload the resource in this case."] #[doc = ""]
    #[doc = " The comparison with the stored ETag uses the strong comparison"] #[doc =
    " algorithm, meaning two files are considered identical byte to byte only."] #[doc =
    " This is weakened when the  W/ prefix is used in front of the ETag."] #[doc = ""]
    #[doc = " There are two common use cases:"] #[doc = ""] #[doc =
    " * For GET and HEAD methods, used in combination with an Range header, it"] #[doc =
    " can guarantee that the new ranges requested comes from the same resource"] #[doc =
    " than the previous one. If it doesn't match, then a 416 (Range Not"] #[doc =
    " Satisfiable) response is returned."] #[doc = ""] #[doc =
    " * For other methods, and in particular for PUT, If-Match can be used to"] #[doc =
    " prevent the lost update problem. It can check if the modification of a"] #[doc =
    " resource that the user wants to upload will not override another change"] #[doc =
    " that has been done since the original resource was fetched. If the"] #[doc =
    " request cannot be fulfilled, the 412 (Precondition Failed) response is"] #[doc =
    " returned."] (IfMatch, IF_MATCH, b"if-match"); #[doc =
    " Makes a request conditional based on the modification date."] #[doc = ""] #[doc =
    " The If-Modified-Since request HTTP header makes the request conditional:"] #[doc =
    " the server will send back the requested resource, with a 200 status,"] #[doc =
    " only if it has been last modified after the given date. If the request"] #[doc =
    " has not been modified since, the response will be a 304 without any"] #[doc =
    " body; the Last-Modified header will contain the date of last"] #[doc =
    " modification. Unlike If-Unmodified-Since, If-Modified-Since can only be"] #[doc =
    " used with a GET or HEAD."] #[doc = ""] #[doc =
    " When used in combination with If-None-Match, it is ignored, unless the"] #[doc =
    " server doesn't support If-None-Match."] #[doc = ""] #[doc =
    " The most common use case is to update a cached entity that has no"] #[doc =
    " associated ETag."] (IfModifiedSince, IF_MODIFIED_SINCE, b"if-modified-since");
    #[doc = " Makes a request conditional based on the E-Tag."] #[doc = ""] #[doc =
    " The If-None-Match HTTP request header makes the request conditional. For"] #[doc =
    " GET and HEAD methods, the server will send back the requested resource,"] #[doc =
    " with a 200 status, only if it doesn't have an ETag matching the given"] #[doc =
    " ones. For other methods, the request will be processed only if the"] #[doc =
    " eventually existing resource's ETag doesn't match any of the values"] #[doc =
    " listed."] #[doc = ""] #[doc =
    " When the condition fails for GET and HEAD methods, then the server must"] #[doc =
    " return HTTP status code 304 (Not Modified). For methods that apply"] #[doc =
    " server-side changes, the status code 412 (Precondition Failed) is used."] #[doc =
    " Note that the server generating a 304 response MUST generate any of the"] #[doc =
    " following header fields that would have been sent in a 200 (OK) response"] #[doc =
    " to the same request: Cache-Control, Content-Location, Date, ETag,"] #[doc =
    " Expires, and Vary."] #[doc = ""] #[doc =
    " The comparison with the stored ETag uses the weak comparison algorithm,"] #[doc =
    " meaning two files are considered identical not only if they are"] #[doc =
    " identical byte to byte, but if the content is equivalent. For example,"] #[doc =
    " two pages that would differ only by the date of generation in the footer"] #[doc =
    " would be considered as identical."] #[doc = ""] #[doc =
    " When used in combination with If-Modified-Since, it has precedence (if"] #[doc =
    " the server supports it)."] #[doc = ""] #[doc = " There are two common use cases:"]
    #[doc = ""] #[doc =
    " * For `GET` and `HEAD` methods, to update a cached entity that has an associated ETag."]
    #[doc =
    " * For other methods, and in particular for `PUT`, `If-None-Match` used with"] #[doc
    = " the `*` value can be used to save a file not known to exist,"] #[doc =
    " guaranteeing that another upload didn't happen before, losing the data"] #[doc =
    " of the previous put; this problems is the variation of the lost update"] #[doc =
    " problem."] (IfNoneMatch, IF_NONE_MATCH, b"if-none-match"); #[doc =
    " Makes a request conditional based on range."] #[doc = ""] #[doc =
    " The If-Range HTTP request header makes a range request conditional: if"] #[doc =
    " the condition is fulfilled, the range request will be issued and the"] #[doc =
    " server sends back a 206 Partial Content answer with the appropriate"] #[doc =
    " body. If the condition is not fulfilled, the full resource is sent back,"] #[doc =
    " with a 200 OK status."] #[doc = ""] #[doc =
    " This header can be used either with a Last-Modified validator, or with"] #[doc =
    " an ETag, but not with both."] #[doc = ""] #[doc =
    " The most common use case is to resume a download, to guarantee that the"] #[doc =
    " stored resource has not been modified since the last fragment has been"] #[doc =
    " received."] (IfRange, IF_RANGE, b"if-range"); #[doc =
    " Makes the request conditional based on the last modification date."] #[doc = ""]
    #[doc = " The If-Unmodified-Since request HTTP header makes the request"] #[doc =
    " conditional: the server will send back the requested resource, or accept"] #[doc =
    " it in the case of a POST or another non-safe method, only if it has not"] #[doc =
    " been last modified after the given date. If the request has been"] #[doc =
    " modified after the given date, the response will be a 412 (Precondition"] #[doc =
    " Failed) error."] #[doc = ""] #[doc = " There are two common use cases:"] #[doc =
    ""] #[doc = " * In conjunction non-safe methods, like POST, it can be used to"] #[doc
    = " implement an optimistic concurrency control, like done by some wikis:"] #[doc =
    " editions are rejected if the stored document has been modified since the"] #[doc =
    " original has been retrieved."] #[doc = ""] #[doc =
    " * In conjunction with a range request with a If-Range header, it can be"] #[doc =
    " used to ensure that the new fragment requested comes from an unmodified"] #[doc =
    " document."] (IfUnmodifiedSince, IF_UNMODIFIED_SINCE, b"if-unmodified-since"); #[doc
    = " The Last-Modified header contains the date and time when the origin believes"]
    #[doc = " the resource was last modified."] #[doc = ""] #[doc =
    " The value is a valid Date/Time string defined in [RFC9910](https://datatracker.ietf.org/doc/html/rfc9110#section-5.6.7)"]
    (LastModified, LAST_MODIFIED, b"last-modified"); #[doc =
    " Allows the server to point an interested client to another resource"] #[doc =
    " containing metadata about the requested resource."] (Link, LINK, b"link"); #[doc =
    " Indicates the URL to redirect a page to."] #[doc = ""] #[doc =
    " The Location response header indicates the URL to redirect a page to. It"] #[doc =
    " only provides a meaning when served with a 3xx status response."] #[doc = ""] #[doc
    = " The HTTP method used to make the new request to fetch the page pointed"] #[doc =
    " to by Location depends of the original method and of the kind of"] #[doc =
    " redirection:"] #[doc = ""] #[doc =
    " * If 303 (See Also) responses always lead to the use of a GET method,"] #[doc =
    " 307 (Temporary Redirect) and 308 (Permanent Redirect) don't change the"] #[doc =
    " method used in the original request;"] #[doc = ""] #[doc =
    " * 301 (Permanent Redirect) and 302 (Found) doesn't change the method"] #[doc =
    " most of the time, though older user-agents may (so you basically don't"] #[doc =
    " know)."] #[doc = ""] #[doc =
    " All responses with one of these status codes send a Location header."] #[doc = ""]
    #[doc = " Beside redirect response, messages with 201 (Created) status also"] #[doc =
    " include the Location header. It indicates the URL to the newly created"] #[doc =
    " resource."] #[doc = ""] #[doc =
    " Location and Content-Location are different: Location indicates the"] #[doc =
    " target of a redirection (or the URL of a newly created resource), while"] #[doc =
    " Content-Location indicates the direct URL to use to access the resource"] #[doc =
    " when content negotiation happened, without the need of further content"] #[doc =
    " negotiation. Location is a header associated with the response, while"] #[doc =
    " Content-Location is associated with the entity returned."] (Location, LOCATION,
    b"location"); #[doc =
    " Indicates the max number of intermediaries the request should be sent"] #[doc =
    " through."] (MaxForwards, MAX_FORWARDS, b"max-forwards"); #[doc =
    " Indicates where a fetch originates from."] #[doc = ""] #[doc =
    " It doesn't include any path information, but only the server name. It is"] #[doc =
    " sent with CORS requests, as well as with POST requests. It is similar to"] #[doc =
    " the Referer header, but, unlike this header, it doesn't disclose the"] #[doc =
    " whole path."] (Origin, ORIGIN, b"origin"); #[doc =
    " HTTP/1.0 header usually used for backwards compatibility."] #[doc = ""] #[doc =
    " The Pragma HTTP/1.0 general header is an implementation-specific header"] #[doc =
    " that may have various effects along the request-response chain. It is"] #[doc =
    " used for backwards compatibility with HTTP/1.0 caches where the"] #[doc =
    " Cache-Control HTTP/1.1 header is not yet present."] (Pragma, PRAGMA, b"pragma");
    #[doc = " Defines the authentication method that should be used to gain access to"]
    #[doc = " a proxy."] #[doc = ""] #[doc =
    " Unlike `www-authenticate`, the `proxy-authenticate` header field applies"] #[doc =
    " only to the next outbound client on the response chain. This is because"] #[doc =
    " only the client that chose a given proxy is likely to have the"] #[doc =
    " credentials necessary for authentication. However, when multiple proxies"] #[doc =
    " are used within the same administrative domain, such as office and"] #[doc =
    " regional caching proxies within a large corporate network, it is common"] #[doc =
    " for credentials to be generated by the user agent and passed through the"] #[doc =
    " hierarchy until consumed. Hence, in such a configuration, it will appear"] #[doc =
    " as if Proxy-Authenticate is being forwarded because each proxy will send"] #[doc =
    " the same challenge set."] #[doc = ""] #[doc =
    " The `proxy-authenticate` header is sent along with a `407 Proxy"] #[doc =
    " Authentication Required`."] (ProxyAuthenticate, PROXY_AUTHENTICATE,
    b"proxy-authenticate"); #[doc =
    " Contains the credentials to authenticate a user agent to a proxy server."] #[doc =
    ""] #[doc = " This header is usually included after the server has responded with a"]
    #[doc = " 407 Proxy Authentication Required status and the Proxy-Authenticate"] #[doc
    = " header."] (ProxyAuthorization, PROXY_AUTHORIZATION, b"proxy-authorization");
    #[doc = " Associates a specific cryptographic public key with a certain server."]
    #[doc = ""] #[doc =
    " This decreases the risk of MITM attacks with forged certificates. If one"] #[doc =
    " or several keys are pinned and none of them are used by the server, the"] #[doc =
    " browser will not accept the response as legitimate, and will not display"] #[doc =
    " it."] (PublicKeyPins, PUBLIC_KEY_PINS, b"public-key-pins"); #[doc =
    " Sends reports of pinning violation to the report-uri specified in the"] #[doc =
    " header."] #[doc = ""] #[doc =
    " Unlike `Public-Key-Pins`, this header still allows browsers to connect"] #[doc =
    " to the server if the pinning is violated."] (PublicKeyPinsReportOnly,
    PUBLIC_KEY_PINS_REPORT_ONLY, b"public-key-pins-report-only"); #[doc =
    " Indicates the part of a document that the server should return."] #[doc = ""] #[doc
    = " Several parts can be requested with one Range header at once, and the"] #[doc =
    " server may send back these ranges in a multipart document. If the server"] #[doc =
    " sends back ranges, it uses the 206 Partial Content for the response. If"] #[doc =
    " the ranges are invalid, the server returns the 416 Range Not Satisfiable"] #[doc =
    " error. The server can also ignore the Range header and return the whole"] #[doc =
    " document with a 200 status code."] (Range, RANGE, b"range"); #[doc =
    " Contains the address of the previous web page from which a link to the"] #[doc =
    " currently requested page was followed."] #[doc = ""] #[doc =
    " The Referer header allows servers to identify where people are visiting"] #[doc =
    " them from and may use that data for analytics, logging, or optimized"] #[doc =
    " caching, for example."] (Referer, REFERER, b"referer"); #[doc =
    " Governs which referrer information should be included with requests"] #[doc =
    " made."] (ReferrerPolicy, REFERRER_POLICY, b"referrer-policy"); #[doc =
    " Informs the web browser that the current page or frame should be"] #[doc =
    " refreshed."] (Refresh, REFRESH, b"refresh"); #[doc =
    " The Retry-After response HTTP header indicates how long the user agent"] #[doc =
    " should wait before making a follow-up request. There are two main cases"] #[doc =
    " this header is used:"] #[doc = ""] #[doc =
    " * When sent with a 503 (Service Unavailable) response, it indicates how"] #[doc =
    " long the service is expected to be unavailable."] #[doc = ""] #[doc =
    " * When sent with a redirect response, such as 301 (Moved Permanently),"] #[doc =
    " it indicates the minimum time that the user agent is asked to wait"] #[doc =
    " before issuing the redirected request."] (RetryAfter, RETRY_AFTER, b"retry-after");
    #[doc = " The |Sec-WebSocket-Accept| header field is used in the WebSocket"] #[doc =
    " opening handshake. It is sent from the server to the client to"] #[doc =
    " confirm that the server is willing to initiate the WebSocket"] #[doc =
    " connection."] (SecWebSocketAccept, SEC_WEBSOCKET_ACCEPT, b"sec-websocket-accept");
    #[doc = " The |Sec-WebSocket-Extensions| header field is used in the WebSocket"]
    #[doc = " opening handshake. It is initially sent from the client to the"] #[doc =
    " server, and then subsequently sent from the server to the client, to"] #[doc =
    " agree on a set of protocol-level extensions to use for the duration"] #[doc =
    " of the connection."] (SecWebSocketExtensions, SEC_WEBSOCKET_EXTENSIONS,
    b"sec-websocket-extensions"); #[doc =
    " The |Sec-WebSocket-Key| header field is used in the WebSocket opening"] #[doc =
    " handshake. It is sent from the client to the server to provide part"] #[doc =
    " of the information used by the server to prove that it received a"] #[doc =
    " valid WebSocket opening handshake. This helps ensure that the server"] #[doc =
    " does not accept connections from non-WebSocket clients (e.g., HTTP"] #[doc =
    " clients) that are being abused to send data to unsuspecting WebSocket"] #[doc =
    " servers."] (SecWebSocketKey, SEC_WEBSOCKET_KEY, b"sec-websocket-key"); #[doc =
    " The |Sec-WebSocket-Protocol| header field is used in the WebSocket"] #[doc =
    " opening handshake. It is sent from the client to the server and back"] #[doc =
    " from the server to the client to confirm the subprotocol of the"] #[doc =
    " connection.  This enables scripts to both select a subprotocol and be"] #[doc =
    " sure that the server agreed to serve that subprotocol."] (SecWebSocketProtocol,
    SEC_WEBSOCKET_PROTOCOL, b"sec-websocket-protocol"); #[doc =
    " The |Sec-WebSocket-Version| header field is used in the WebSocket"] #[doc =
    " opening handshake.  It is sent from the client to the server to"] #[doc =
    " indicate the protocol version of the connection.  This enables"] #[doc =
    " servers to correctly interpret the opening handshake and subsequent"] #[doc =
    " data being sent from the data, and close the connection if the server"] #[doc =
    " cannot interpret that data in a safe manner."] (SecWebSocketVersion,
    SEC_WEBSOCKET_VERSION, b"sec-websocket-version"); #[doc =
    " Contains information about the software used by the origin server to"] #[doc =
    " handle the request."] #[doc = ""] #[doc =
    " Overly long and detailed Server values should be avoided as they"] #[doc =
    " potentially reveal internal implementation details that might make it"] #[doc =
    " (slightly) easier for attackers to find and exploit known security"] #[doc =
    " holes."] (Server, SERVER, b"server"); #[doc =
    " Used to send cookies from the server to the user agent."] (SetCookie, SET_COOKIE,
    b"set-cookie"); #[doc =
    " Tells the client to communicate with HTTPS instead of using HTTP."]
    (StrictTransportSecurity, STRICT_TRANSPORT_SECURITY, b"strict-transport-security");
    #[doc = " Informs the server of transfer encodings willing to be accepted as part"]
    #[doc = " of the response."] #[doc = ""] #[doc =
    " See also the Transfer-Encoding response header for more details on"] #[doc =
    " transfer encodings. Note that chunked is always acceptable for HTTP/1.1"] #[doc =
    " recipients and you that don't have to specify \"chunked\" using the TE"] #[doc =
    " header. However, it is useful for setting if the client is accepting"] #[doc =
    " trailer fields in a chunked transfer coding using the \"trailers\" value."] (Te,
    TE, b"te"); #[doc =
    " Allows the sender to include additional fields at the end of chunked"] #[doc =
    " messages."] (Trailer, TRAILER, b"trailer"); #[doc =
    " Specifies the form of encoding used to safely transfer the entity to the"] #[doc =
    " client."] #[doc = ""] #[doc =
    " `transfer-encoding` is a hop-by-hop header, that is applying to a"] #[doc =
    " message between two nodes, not to a resource itself. Each segment of a"] #[doc =
    " multi-node connection can use different `transfer-encoding` values. If"] #[doc =
    " you want to compress data over the whole connection, use the end-to-end"] #[doc =
    " header `content-encoding` header instead."] #[doc = ""] #[doc =
    " When present on a response to a `HEAD` request that has no body, it"] #[doc =
    " indicates the value that would have applied to the corresponding `GET`"] #[doc =
    " message."] (TransferEncoding, TRANSFER_ENCODING, b"transfer-encoding"); #[doc =
    " Contains a string that allows identifying the requesting client's"] #[doc =
    " software."] (UserAgent, USER_AGENT, b"user-agent"); #[doc =
    " Used as part of the exchange to upgrade the protocol."] (Upgrade, UPGRADE,
    b"upgrade"); #[doc =
    " Sends a signal to the server expressing the client’s preference for an"] #[doc =
    " encrypted and authenticated response."] (UpgradeInsecureRequests,
    UPGRADE_INSECURE_REQUESTS, b"upgrade-insecure-requests"); #[doc =
    " Determines how to match future requests with cached responses."] #[doc = ""] #[doc
    = " The `vary` HTTP response header determines how to match future request"] #[doc =
    " headers to decide whether a cached response can be used rather than"] #[doc =
    " requesting a fresh one from the origin server. It is used by the server"] #[doc =
    " to indicate which headers it used when selecting a representation of a"] #[doc =
    " resource in a content negotiation algorithm."] #[doc = ""] #[doc =
    " The `vary` header should be set on a 304 Not Modified response exactly"] #[doc =
    " like it would have been set on an equivalent 200 OK response."] (Vary, VARY,
    b"vary"); #[doc = " Added by proxies to track routing."] #[doc = ""] #[doc =
    " The `via` general header is added by proxies, both forward and reverse"] #[doc =
    " proxies, and can appear in the request headers and the response headers."] #[doc =
    " It is used for tracking message forwards, avoiding request loops, and"] #[doc =
    " identifying the protocol capabilities of senders along the"] #[doc =
    " request/response chain."] (Via, VIA, b"via"); #[doc =
    " General HTTP header contains information about possible problems with"] #[doc =
    " the status of the message."] #[doc = ""] #[doc =
    " More than one `warning` header may appear in a response. Warning header"] #[doc =
    " fields can in general be applied to any message, however some warn-codes"] #[doc =
    " are specific to caches and can only be applied to response messages."] (Warning,
    WARNING, b"warning"); #[doc =
    " Defines the authentication method that should be used to gain access to"] #[doc =
    " a resource."] (WwwAuthenticate, WWW_AUTHENTICATE, b"www-authenticate"); #[doc =
    " Marker used by the server to indicate that the MIME types advertised in"] #[doc =
    " the `content-type` headers should not be changed and be followed."] #[doc = ""]
    #[doc = " This allows to opt-out of MIME type sniffing, or, in other words, it is"]
    #[doc = " a way to say that the webmasters knew what they were doing."] #[doc = ""]
    #[doc = " This header was introduced by Microsoft in IE 8 as a way for webmasters"]
    #[doc = " to block content sniffing that was happening and could transform"] #[doc =
    " non-executable MIME types into executable MIME types. Since then, other"] #[doc =
    " browsers have introduced it, even if their MIME sniffing algorithms were"] #[doc =
    " less aggressive."] #[doc = ""] #[doc =
    " Site security testers usually expect this header to be set."] (XContentTypeOptions,
    X_CONTENT_TYPE_OPTIONS, b"x-content-type-options"); #[doc =
    " Controls DNS prefetching."] #[doc = ""] #[doc =
    " The `x-dns-prefetch-control` HTTP response header controls DNS"] #[doc =
    " prefetching, a feature by which browsers proactively perform domain name"] #[doc =
    " resolution on both links that the user may choose to follow as well as"] #[doc =
    " URLs for items referenced by the document, including images, CSS,"] #[doc =
    " JavaScript, and so forth."] #[doc = ""] #[doc =
    " This prefetching is performed in the background, so that the DNS is"] #[doc =
    " likely to have been resolved by the time the referenced items are"] #[doc =
    " needed. This reduces latency when the user clicks a link."] (XDnsPrefetchControl,
    X_DNS_PREFETCH_CONTROL, b"x-dns-prefetch-control"); #[doc =
    " Indicates whether or not a browser should be allowed to render a page in"] #[doc =
    " a frame."] #[doc = ""] #[doc =
    " Sites can use this to avoid clickjacking attacks, by ensuring that their"] #[doc =
    " content is not embedded into other sites."] #[doc = ""] #[doc =
    " The added security is only provided if the user accessing the document"] #[doc =
    " is using a browser supporting `x-frame-options`."] (XFrameOptions, X_FRAME_OPTIONS,
    b"x-frame-options"); #[doc =
    " Stop pages from loading when an XSS attack is detected."] #[doc = ""] #[doc =
    " The HTTP X-XSS-Protection response header is a feature of Internet"] #[doc =
    " Explorer, Chrome and Safari that stops pages from loading when they"] #[doc =
    " detect reflected cross-site scripting (XSS) attacks. Although these"] #[doc =
    " protections are largely unnecessary in modern browsers when sites"] #[doc =
    " implement a strong Content-Security-Policy that disables the use of"] #[doc =
    " inline JavaScript ('unsafe-inline'), they can still provide protections"] #[doc =
    " for users of older web browsers that don't yet support CSP."] (XXssProtection,
    X_XSS_PROTECTION, b"x-xss-protection");
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Custom(ByteStr);
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct ByteStr {
    bytes: Bytes,
}
impl Hash for Custom {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.0.as_bytes())
    }
}
