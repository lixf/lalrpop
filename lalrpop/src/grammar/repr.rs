/*!
 * Compiled representation of a grammar. Simplified, normalized
 * version of `parse_tree`. The normalization passes produce this
 * representation incrementally.
 */

use intern::{self, InternedString};
use grammar::pattern::{Pattern};
use std::fmt::{Debug, Display, Formatter, Error};
use util::{map, Map, Sep};

// These concepts we re-use wholesale
pub use grammar::parse_tree::{InternToken,
                              NonterminalString,
                              Path,
                              Span,
                              TerminalString, TypeParameter};

#[derive(Clone, Debug)]
pub struct Grammar {
    // a unique prefix that can be appended to identifiers to ensure
    // that they do not conflict with any action strings
    pub prefix: String,

    // algorithm user requested for this parser
    pub algorithm: Algorithm,

    // these are the nonterminals that were declared to be public; the
    // key is the user's name for the symbol, the value is the
    // artificial symbol we introduce, which will always have a single
    // production like `Foo' = Foo`.
    pub start_nonterminals: Map<NonterminalString, NonterminalString>,

    // the "use foo;" statements that the user declared
    pub uses: Vec<String>,

    // type parameters declared on the grammar, like `grammar<T>;`
    pub type_parameters: Vec<TypeParameter>,

    // actual parameters declared on the grammar, like the `x: u32` in `grammar(x: u32);`
    pub parameters: Vec<Parameter>,

    // where clauses declared on the grammar, like `grammar<T> where T: Sized`
    pub where_clauses: Vec<String>,

    // optional tokenizer DFA; this is only needed if the user did not supply
    // an extern token declaration
    pub intern_token: Option<InternToken>,

    // the grammar proper:

    pub action_fn_defns: Vec<ActionFnDefn>,
    pub productions: Map<NonterminalString, Vec<Production>>,
    pub token_span: Span,
    pub conversions: Map<TerminalString, Pattern<TypeRepr>>,
    pub types: Types,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Algorithm {
    LR1,
    LALR1,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: InternedString,
    pub ty: TypeRepr,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Production {
    // this overlaps with the key in the hashmap, obviously, but it's
    // handy to have it
    pub nonterminal: NonterminalString,
    pub symbols: Vec<Symbol>,
    pub action: ActionKind,
    pub span: Span,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Nonterminal(NonterminalString),
    Terminal(TerminalString),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionKind {
    // execute code provided by the user
    Call(ActionFn),
    TryCall(ActionFn),
    Lookahead,
    Lookbehind,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ActionFnDefn {
    pub arg_patterns: Vec<InternedString>,
    pub arg_types: Vec<TypeRepr>,
    pub ret_type: TypeRepr,
    pub fallible: bool,
    pub code: String,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TypeRepr {
    Tuple(Vec<TypeRepr>),
    Nominal(NominalTypeRepr),
    Lifetime(InternedString),
    Ref {
        lifetime: Option<InternedString>,
        mutable: bool,
        referent: Box<TypeRepr>,
    },
}

#[derive(Clone, PartialEq, Eq)]
pub struct NominalTypeRepr {
    pub path: Path,
    pub types: Vec<TypeRepr>
}

#[derive(Clone, Debug)]
pub struct Types {
    terminal_token_type: TypeRepr,
    terminal_loc_type: Option<TypeRepr>,
    error_type: Option<TypeRepr>,
    terminal_types: Map<TerminalString, TypeRepr>,
    nonterminal_types: Map<NonterminalString, TypeRepr>
}

impl Types {
    pub fn new(terminal_loc_type: Option<TypeRepr>,
               error_type: Option<TypeRepr>,
               terminal_token_type: TypeRepr)
               -> Types {
        Types { terminal_loc_type: terminal_loc_type,
                error_type: error_type,
                terminal_token_type: terminal_token_type,
                terminal_types: map(),
                nonterminal_types: map() }
    }

    pub fn add_type(&mut self, nt_id: NonterminalString, ty: TypeRepr) {
        assert!(self.nonterminal_types.insert(nt_id, ty).is_none());
    }

    pub fn add_term_type(&mut self, term: TerminalString, ty: TypeRepr) {
        assert!(self.terminal_types.insert(term, ty).is_none());
    }

    pub fn terminal_token_type(&self) -> &TypeRepr {
        &self.terminal_token_type
    }

    pub fn opt_terminal_loc_type(&self) -> Option<&TypeRepr> {
        self.terminal_loc_type.as_ref()
    }

    pub fn terminal_loc_type(&self) -> TypeRepr {
        self.terminal_loc_type.clone()
                              .unwrap_or_else(|| TypeRepr::Tuple(vec![]))
    }

    pub fn error_type(&self) -> TypeRepr {
        self.error_type.clone()
                       .unwrap_or_else(|| TypeRepr::Tuple(vec![]))
    }

    pub fn terminal_type(&self, id: TerminalString) -> &TypeRepr {
        self.terminal_types.get(&id).unwrap_or(&self.terminal_token_type)
    }

    pub fn lookup_nonterminal_type(&self, id: NonterminalString) -> Option<&TypeRepr> {
        self.nonterminal_types.get(&id)
    }

    pub fn nonterminal_type(&self, id: NonterminalString) -> &TypeRepr {
        &self.nonterminal_types[&id]
    }

    pub fn triple_type(&self) -> TypeRepr {
        let enum_type = self.terminal_token_type();
        let location_type = self.terminal_loc_type();
        TypeRepr::Tuple(vec![location_type.clone(),
                             enum_type.clone(),
                             location_type])
    }
}

impl Display for Parameter {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}: {}", self.name, self.ty)
    }
}

impl Display for TypeRepr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            TypeRepr::Tuple(ref types) =>
                write!(fmt, "({})", Sep(", ", types)),
            TypeRepr::Nominal(ref data) =>
                write!(fmt, "{}", data),
            TypeRepr::Lifetime(id) =>
                write!(fmt, "{}", id),
            TypeRepr::Ref { lifetime: None, mutable: false, ref referent } =>
                write!(fmt, "&{}", referent),
            TypeRepr::Ref { lifetime: Some(l), mutable: false, ref referent } =>
                write!(fmt, "&{} {}", l, referent),
            TypeRepr::Ref { lifetime: None, mutable: true, ref referent } =>
                write!(fmt, "&mut {}", referent),
            TypeRepr::Ref { lifetime: Some(l), mutable: true, ref referent } =>
                write!(fmt, "&{} mut {}", l, referent),
        }
    }
}

impl Debug for TypeRepr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Display for NominalTypeRepr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.types.len() == 0 {
            write!(fmt, "{}", self.path)
        } else {
            write!(fmt, "{}<{}>", self.path, Sep(", ", &self.types))
        }
    }
}

impl Debug for NominalTypeRepr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ActionFn(u32);

impl ActionFn {
    pub fn new(x: usize) -> ActionFn {
        ActionFn(x as u32)
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl Symbol {
    pub fn is_terminal(&self) -> bool {
        match *self {
            Symbol::Terminal(..) => true,
            Symbol::Nonterminal(..) => false,
        }
    }

    pub fn ty<'ty>(&self, t: &'ty Types) -> &'ty TypeRepr {
        match *self {
            Symbol::Terminal(id) => t.terminal_type(id),
            Symbol::Nonterminal(id) => t.nonterminal_type(id),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Symbol::Nonterminal(id) => write!(fmt, "{}", id),
            Symbol::Terminal(id) => write!(fmt, "{}", id),
        }
    }
}

impl Debug for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, fmt)
    }
}

impl Debug for Production {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt,
               "{} = {} => {:?};",
               self.nonterminal, Sep(", ", &self.symbols), self.action)
    }
}

impl Debug for ActionFnDefn {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.to_fn_string("_"))
    }
}

impl ActionFnDefn {
    fn to_fn_string(&self, name: &str) -> String {
        let arg_strings: Vec<String> =
               self.arg_patterns
                   .iter()
                   .zip(self.arg_types.iter())
                   .map(|(p, t)| format!("{}: {}", p, t))
                   .collect();

        format!("fn {}({}) -> {} {{ {} }}",
                name, Sep(", ", &arg_strings), self.ret_type, self.code)
    }
}

impl Grammar {
    pub fn pattern(&self, t: TerminalString) -> &Pattern<TypeRepr> {
        &self.conversions[&t]
    }

    pub fn productions_for(&self, nonterminal: NonterminalString) -> &[Production] {
        match self.productions.get(&nonterminal) {
            Some(v) => &v[..],
            None => &[], // this...probably shouldn't happen actually?
        }
    }

    pub fn user_parameter_refs(&self) -> String {
        let mut result = String::new();
        for parameter in &self.parameters {
            result.push_str(&format!("{}, ", parameter.name));
        }
        result
    }

    pub fn user_type_parameter_decls(&self) -> String {
        let mut result = String::new();
        for parameter in &self.type_parameters {
            result.push_str(&format!("{}, ", parameter));
        }
        result
    }

    pub fn user_type_parameter_refs(&self) -> String {
        self.user_type_parameter_decls()
    }
}

impl Algorithm {
    pub fn from_str(s: InternedString) -> Option<Algorithm> {
        intern::read(|r| match r.data(s) {
            "LR" | "LR(1)" => Some(Algorithm::LR1),
            "LALR" | "LALR(1)" => Some(Algorithm::LALR1),
            _ => None,
        })
    }
}
