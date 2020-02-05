grammar dumdum;

program: oration+;

oration: 'dum' expr 'dum';

expr: method ALPHA;

method: 'some' | 'potato' | 'm1' | 'm2';

ALPHA: [a-zA-Z]+;

WS: [ \r\n] -> skip;
