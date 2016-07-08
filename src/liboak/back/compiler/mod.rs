// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod rtype;
mod value;
mod grammar;
mod rule;
mod str_literal;
mod sequence;
mod choice;
mod any_single_char;

pub use back::compiler::grammar::*;
pub use back::context::*;
use back::compiler::str_literal::*;
use back::compiler::sequence::*;
use back::compiler::choice::*;
use back::compiler::any_single_char::*;

pub trait CompileExpr
{
  fn compile_expr<'a, 'b, 'c>(&self, context: &mut Context<'a, 'b, 'c>, cont: Continuation) -> RExpr;
}

pub type ExprCompilerFn = fn(&TGrammar, usize) -> Box<CompileExpr>;

pub fn parser_compiler(grammar: &TGrammar, idx: usize) -> Box<CompileExpr> {
  if grammar[idx].ty.is_unit() {
    recognizer_compiler(grammar, idx)
  }
  else {
    match grammar.expr_by_index(idx) {
      StrLiteral(lit) => Box::new(StrLiteralCompiler::parser(lit)),
      Sequence(seq) => Box::new(SequenceCompiler::parser(seq)),
      AnySingleChar => Box::new(AnySingleCharCompiler::parser()),
      Choice(choices) => Box::new(ChoiceCompiler::parser(choices)),
      _ => unimplemented!()
      // NonTerminalSymbol(id) =>
      // ZeroOrMore(expr) =>
      // OneOrMore(expr) =>
      // Optional(expr) =>
      // NotPredicate(expr) =>
      // AndPredicate(expr) =>
      // CharacterClass(char_class) =>
      // SemanticAction(expr, id) =>
    }
  }
}

pub fn recognizer_compiler(grammar: &TGrammar, idx: usize) -> Box<CompileExpr> {
  match grammar.expr_by_index(idx) {
    StrLiteral(lit) => Box::new(StrLiteralCompiler::recognizer(lit)),
    Sequence(seq) => Box::new(SequenceCompiler::recognizer(seq)),
    AnySingleChar => Box::new(AnySingleCharCompiler::recognizer()),
      Choice(choices) => Box::new(ChoiceCompiler::recognizer(choices)),
    _ => unimplemented!()
    // NonTerminalSymbol(id) =>
    // ZeroOrMore(expr) =>
    // OneOrMore(expr) =>
    // Optional(expr) =>
    // NotPredicate(expr) =>
    // AndPredicate(expr) =>
    // CharacterClass(char_class) =>
    // SemanticAction(expr, id) =>
  }
}
