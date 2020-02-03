grammar phttp;

request: ('[' identifier ']')? SP method SP hostname NL (
		header NL
	)* (NL NL body)?;

assignment: identifier SP* '=' SP* content;
identifier: '@' ALPHANUM+;

expression:
	'{{' SP* (request_response | STR) (
		SP* '|' SP* (request_response | STR)
	)* SP* '}}';
request_response: ('request' | 'response') ('.' ALPHANUM+)+;

content: (STR | expression)+;

hostname: (ALPHANUM | content)+;
header: ALPHANUM+ SP content;

pair: STR SP* ':' SP* (STR | object | array);
object: '{' NL? SP* pair ( SP* ',' NL? SP* pair)* SP* NL? '}';
array: '[' NL? SP* STR ( SP* ',' NL? SP* STR)* SP* ']';
body: '{' NL? (pair ',' NL?)+ '}';

method:
	'GET'
	| 'HEAD'
	| 'POST'
	| 'PUT'
	| 'DELETE'
	| 'CONNECT'
	| 'OPTIONS'
	| 'TRACE';

STR: (ALPHANUM | '"' | '\'' | SP)+;

ALPHANUM: [a-zA-Z0-9];

SP: ' ' | '\t';

NL: '\n' | '\n\r';