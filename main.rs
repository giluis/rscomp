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
   
   impl ParseAST for Function {
       fn parse(iter: &mut TokenIter) -> Result<Function, String> {
            let return_type = iter.get::<Type>()?;
            let ident = iter.get::<TIdent>()?;
            let args = iter.get::<Args>()?;
            let body = iter.get::<Body>()?;

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
            let statements = iter.parse_collection::<Statement>()
                                                .delimiter(Token::LeftCurly, Token::RightCurly)
                                                .separator(Token::SemiColon)?;
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
        _: Token;

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
        asterisks: Vec<t!( * )>;

        #[ast(count = asterisks)]
        count: u8;
   }

   impl ParseAST for Pointer {
       fn parse(iter: &mut TokenIter) -> Result<Pointer, String> {
            
            return Ok(Pointer {
                qualifiers : iter.parse_collection_with<t!( * )>(1,true)?,
                pointer : iter.parse_possibly<Pointer>(),
                concrete_type : iter.parse<ConcreteType()>?,
            })
   }


   #[derive(AstNode)]
   pub enum Qualifier {
        Static(TKwStatic),

        Const(TKwConst),
   }


   #[derive(AstNode)]
   pub enum ConcreteType {
        Int(TKwInt),

        Char(TKwChar),

        Long(TKwLong),

        Short(TKwShort),
   }

   #[derive(AstNode)]
   pub struct FunctionArgs {
        #[ast(del = "()", sep = ",")]
        args: Vec<Arg>
   }

   #[derive(AstNode)]
   pub struct Arg {
        arg_type: Type;
        arg_identifier: Ident;
   }
  
 * */


use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,

    #[builder(each = "arg")]
    args: Vec<String>,

    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
    // assert_eq!(command.args, vec!["build", "--release"]);
}
