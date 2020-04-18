export const ENTER = 13;
export const ARROW_UP = 38;
export const ARROW_DOWN = 40;

export const repoUrl = 'https://github.com/GrzegorzKazana/toy-interpreter';

export const grammarCmd = 'syntax';
export const grammar = `
    <command> ::= <statement> | <expression>;

    <statement> ::= <assignment> | <function-declaration>;

    <assignment> ::= <identifier> = <expression>;
    <function-declaration> ::= fun <identifier>([<declaration-argument-list>]) = <expression>;
    <declaration-argument-list> ::= <identifier>[, <identifier>]*;


    <expression> ::= 
        <number-literal> | 
        <signed-number-literal> | 
        <identifier> | 
        <function-call> | 
        <terenary-expression> | 
        <expression><operator><expression> | 
        (<expression>);

    <signed-number-literal> ::= +<number-literal> | -<number-literal>;
    <function-call> ::= <identifier>([<argument-list>]);
    <argument-list> ::= <expression>[, <expression>]*;
    <terenary-expression> ::= <expression> ? <expression> : <expression>;

    <identifier> ::= '[a-zA-Z]\\w*';
    <number-literal> ::= '\\d+';
    <operator> ::= + | - | * | /;

`;

export const examplesCmd = 'examples';
export const examples = `
    2 + 2
    9 - 2 * 3
    (9 - 2) * 3

    a = 2 + 3
    b = a + 1

    2 ? 3 : 4       // 3
    0 ? 3 : 4       // 4
    -2 ? 3 : 4      // 4 - positive values considered truthy

    fun sum(a, b) = a + b
    fun sub(a, b) = sum(a, -b)

    fun factorial(n) = n ? n * factorial(n - 1) : 1
    fun fibonacci(n) = n ? (n - 1 ? fibonacci(n - 1) + fibonacci(n - 2) : 1) : 0

`;

export const welcomeHeader = `Welcome to toy-interpreter!
A simple interpreter written in Rust running in Your browser using WebAssembly.
`;

export const welcomeFooter = `
Type '${grammarCmd}' or '${examplesCmd}' for help.
`;
