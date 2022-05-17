

// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run


/*
 
   #[derive(AstNode)]
   pub struct Function {
        return_type: Type
        ident: TIdent;
        args: Args;
        body: Body;
   }
   
   impl Parseable for Function {
       fn parse(iter: &mut TokenIter) -> Result<Function, String> {
            let return_type = iter.parse::<Type>()?;
            let ident = iter.parse::<TIdent>()?;
            let args = iter.parse::<Args>()?;
            let body = iter.parse::<Body>()?;

            return Ok(Function{
                return_type,
                ident,
                args,
                body,
            })


       }
   }


   #[derive(AstNode)]
   pub struct Body {
        #[ast(
            del = t!( {} ), 
            sep = t!( ; )
        )]
        statements: Vec<Statement>
   }

   impl ParseAST for Body {
       fn parse(iter: &mut TokenIter) -> Result<Body, String> {
            let statements = iter.collection::<Statement>()
                                                .delimiter(Token::LeftCurly, Token::RightCurly)
                                                .separator(Token::SemiColon)
                                                .parse()?;
            return Ok(Body{
                statements
            })
       }
   }

   #[derive(AstNode)]
   pub enum Statement {
        Assign(AssignStatement)
        Return(ReturnStatement)
   }

   impl ParseAST for Statement {
       fn parse(iter: &mut TokenIter) -> Result<Statement, String> {
           match iter.parse<AssignStatement>() {
                Ok(AssignStatement) => return Ok(Statement::Assign(AssignStatement)),
                Err => ()
           }
           match iter.parse<ReturnStatement>() {
                Ok(return_statement) => return Ok(Statement::Return(return_statement)),
                Err => ()
           }
           Err("Expected statement here");
   }



   #[derive(AstNode)]
   pub struct AssignStatement {

        var_type: Type;
        var_ident: Ident;

        #[ast(token = t!( = ))]
        equal_sign: Token;
        expression: Expression;
   }

   impl ParseAST for AssignStatement {
       fn parse(iter: &mut TokenIter) -> Result<AssignStatement, String> {
            return Ok(AssignmentStatement {
                 iter.parse<Type>()?, 
                 iter.parse<Ident>()?, 
                 iter.parse<t!( = )>()?,
                 iter.parse<Expression>()?,
            })
   }

   #[derive(AstNode)]
   pub struct ReturnStatement {

        #[ast(token = t!( return ))]
        _: KReturn;

        expression: Expression;
   }

   impl ParseAST for ReturnStatement {
       fn parse(iter: &mut TokenIter) -> Result<ReturnStatement, String> {
            return Ok(ReturnStatement {
                 iter.parse<t!( return )>()?, 
                 iter.parse<Expression>()?,
            })
   }

   #[derive(AstNode)]
   pub struct KReturn {

        #[ast(from_token = t!( return ))]
        return_token: String;
   }

   impl ParseAST for KReturn {
       fn parse(iter: &mut TokenIter) -> Result<KReturn, String> {
            return Ok(KReturn {
                return_token: match iter.parse_token(t!(return))? {
                    Token::KWReturn(return_token) => return_token,
                    _ => panic!("Internal consitency error, please report"),
                }, 
            })
   }



   #[derive(AstNode)]
   pub struct Type {
        qualifiers: Vec<Qualifier>
        pointer: Option<Pointer> 
        type: ConcreteType
   }

   impl ParseAST for Type {
       fn parse(iter: &mut TokenIter) -> Result<Type, String> {
            return Ok(Type {
                qualifiers : iter.parse_collection<Qualifier>()?,
                pointer : iter.parse_possibly<Pointer>(),
                concrete_type : iter.parse<ConcreteType()>?,
            })
   }


   pub struct Pointer {
        #[ast(minlen = 1)]
        pointers: Vec<t!( * )>;

        #[ast(count = pointers)]
        pointer_dimension: u8;
   }

   impl ParseAST for Pointer {
       fn parse(iter: &mut TokenIter) -> Result<Pointer, String> {
            let pointers= iter.collection<t!( * )>()
                                .min_len(1)
                                .parse()?,
            return Ok(Pointer {
                pionter_dimension: pointers.len(),
                pointers: pointers,
            })
   }


   #[derive(AstNode)]
   pub enum Qualifier {
        Static(TKwStatic),

        Const(TKwConst),
   }

   impl ParseAST for Qualifier {
       fn parse(iter: &mut TokenIter) -> Result<Qualifier, String> {
           match iter.parse<TKwStatic>() {
                Ok(tkwstatic) => return Ok(Qualifier::Static(tkwstatic)),
                Err => ()
           }
           match iter.parse<TKwConst>() {
                Ok(tkwconst) => return Ok(Qualifier::Const(tkwconst)),
                Err => ()
           }
           Err("Expected qualifier here");
   }





   #[derive(AstNode)]
   pub enum ConcreteType {
        Int(TKwInt),

        Char(TKwChar),

        Long(TKwLong),

        Short(TKwShort),
   }

   impl ParseAST for ConcreteType {
       fn parse(iter: &mut TokenIter) -> Result<ConcreteType, String> {
           match iter.parse<TKwInt>() {
                Ok(tkwint) => return Ok(ConcreteType::Static(tkwint)),
                Err => ()
           };
           match iter.parse<TKwChar>() {
                Ok(tkwchar) => return Ok(ConcreteType::Const(tkwchar)),
                Err => ()
           };
           match iter.parse<TKwLong>() {
                Ok(tkwlong) => return Ok(ConcreteType::Const(tkwlong)),
                Err => ()
           };
           match iter.parse<TkwShort>() {
                Ok(tkwshort) => return Ok(ConcreteType::Const(tkwshort)),
                Err => ()
           };
           Err("Expected ConcreteType here");
   }


   #[derive(AstNode)]
   pub struct FunctionArgs {
        #[ast(del = "()", sep = ",")]
        args: Vec<Arg>
   }

   impl ParseAST for FunctionArgs {
       fn parse(iter: &mut TokenIter) -> Result<FunctionArgs, String> {
            let args = iter.collection::<Arg>()
                                   .delimiter(Token::LeftParen, Token::RightParen)
                                   .separator(Token::Colon)
                                   .parse()?;
            return Ok(FunctionArgs{
                args
            })
       }
   }


   #[derive(AstNode)]
   pub struct Arg {
        arg_type: Type;
        arg_identifier: Ident;
   }

   impl ParseAST for Arg {
       fn parse(iter: &mut TokenIter) -> Result<FunctionArgs, String> {
            return Ok(FunctionArgs{
                arg_type: iter.parse<Type>()?,
                arg_identifier: iter.parse<TIdent>()?,

            })
       }
   }

  
 * */


// use derive_builder::Builder;

// #[derive(Builder)]
// pub struct Command {
//     executable: String,
//
//     #[builder(each = "arg")]
//     args: Vec<String>,
//
//     #[builder(each = "env")]
//     env: Vec<String>,
//     current_dir: Option<String>,
// }

fn main() {
    // let command = Command::builder()
    //     .executable("cargo".to_owned())
    //     .arg("build".to_owned())
    //     .arg("--release".to_owned())
    //     .build()
    //     .unwrap();
    //
    // assert_eq!(command.executable, "cargo");
    // // assert_eq!(command.args, vec!["build", "--release"]);
    println!("Inside main");
}
