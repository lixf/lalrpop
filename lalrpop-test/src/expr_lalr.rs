#![allow(unused_imports)]
#![allow(unused_variables)]
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Expr<
        __TOKEN: __ToTriple<Error=()>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        scale: i32,
        __tokens: __TOKENS,
    ) -> Result<i32, __ParseError<(),Tok,()>>
    {
        let mut __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match try!(__state0(scale, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Expr(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        ____Expr(i32),
        Term(i32),
        Expr(i32),
        Factor(i32),
    }

    // State 0
    //   Expr = (*) Expr "+" Factor [EOF]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [EOF]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [EOF]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "(" -> Shift(S4)
    //   Num -> Shift(S3)
    //
    //   Factor -> S2
    //   Term -> S1
    //   Expr -> S5
    pub fn __state0<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok));
                __result = try!(__state4(scale, __lookbehind, __tokens, __sym0));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state3(scale, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(scale, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(scale, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(scale, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   EOF -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //
    pub fn __state1<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            None |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Minus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Expr = Factor (*) [EOF]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S7)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   EOF -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S6)
    //
    pub fn __state2<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state7(scale, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state6(scale, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Term = Num (*) [EOF]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //   Term = Num (*) [")"]
    //   Term = Num (*) ["*"]
    //   Term = Num (*) ["+"]
    //   Term = Num (*) ["-"]
    //   Term = Num (*) ["/"]
    //
    //   "/" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   EOF -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "-" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   ")" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "*" -> Reduce(Term = Num => Call(ActionFn(7));)
    //   "+" -> Reduce(Term = Num => Call(ActionFn(7));)
    //
    pub fn __state3<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, Tok::Div(..), _)) |
            None |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [EOF]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S4)
    //   Num -> Shift(S3)
    //
    //   Factor -> S2
    //   Term -> S1
    //   Expr -> S8
    pub fn __state4<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state4(scale, __lookbehind, __tokens, __sym1));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state3(scale, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state2(scale, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state1(scale, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(scale, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 5
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   EOF -> Reduce(__Expr = Expr => Call(ActionFn(0));)
    //   "-" -> Shift(S10)
    //   "+" -> Shift(S9)
    //
    pub fn __state5<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state10(scale, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok));
                __result = try!(__state9(scale, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(scale, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 6
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   Num -> Shift(S3)
    //   "(" -> Shift(S4)
    //
    //   Term -> S11
    pub fn __state6<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state3(scale, __lookbehind, __tokens, __sym2));
            }
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(scale, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state11(scale, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S4)
    //   Num -> Shift(S3)
    //
    //   Term -> S12
    pub fn __state7<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(scale, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state3(scale, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(scale, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   "-" -> Shift(S10)
    //   ")" -> Shift(S13)
    //   "+" -> Shift(S9)
    //
    pub fn __state8<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Minus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state10(scale, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::RParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state13(scale, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, __tok @ Tok::Plus(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state9(scale, __lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Expr = Expr "+" (*) Factor [EOF]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S4)
    //   Num -> Shift(S3)
    //
    //   Term -> S1
    //   Factor -> S14
    pub fn __state9<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(scale, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state3(scale, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state1(scale, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state14(scale, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   Expr = Expr "-" (*) Factor [EOF]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [EOF]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) Num [")"]
    //   Term = (*) Num ["*"]
    //   Term = (*) Num ["+"]
    //   Term = (*) Num ["-"]
    //   Term = (*) Num ["/"]
    //
    //   "(" -> Shift(S4)
    //   Num -> Shift(S3)
    //
    //   Factor -> S15
    //   Term -> S1
    pub fn __state10<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, __tok @ Tok::LParen(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok));
                __result = try!(__state4(scale, __lookbehind, __tokens, __sym2));
            }
            Some((_, Tok::Num(__tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state3(scale, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(scale, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state1(scale, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 11
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   EOF -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state11<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::Minus(..), _)) |
            None |
            Some((_, Tok::Div(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 12
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   EOF -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state12<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Times(..), _)) |
            None |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   EOF -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //
    pub fn __state13<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Tok>,
        __sym1: &mut Option<i32>,
        __sym2: &mut Option<Tok>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        let __lookahead = match __tokens.next() { Some(Ok(v)) => Some(v), None => None, Some(Err(e)) => return Err(__ParseError::User { error: e }) };
        match __lookahead {
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::Plus(..), _)) |
            Some((_, Tok::Div(..), _)) |
            Some((_, Tok::Times(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   Expr = Expr "+" Factor (*) [EOF]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S6)
    //   "*" -> Shift(S7)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //
    pub fn __state14<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state6(scale, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state7(scale, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, Tok::Minus(..), _)) |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Expr = Expr "-" Factor (*) [EOF]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   EOF -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S6)
    //   "*" -> Shift(S7)
    //
    pub fn __state15<
        __TOKENS: Iterator<Item=Result<((), Tok, ()),()>>,
    >(
        scale: i32,
        __lookbehind: Option<()>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<((), Tok, ())>,
        __sym0: &mut Option<i32>,
        __sym1: &mut Option<Tok>,
        __sym2: &mut Option<i32>,
    ) -> Result<(Option<()>, Option<((), Tok, ())>, __Nonterminal<>), __ParseError<(),Tok,()>>
    {
        let mut __result: (Option<()>, Option<((), Tok, ())>, __Nonterminal<>);
        match __lookahead {
            Some((_, __tok @ Tok::Div(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state6(scale, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, __tok @ Tok::Times(..), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok));
                __result = try!(__state7(scale, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, Tok::Minus(..), _)) |
            None |
            Some((_, Tok::RParen(..), _)) |
            Some((_, Tok::Plus(..), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(scale, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }
}
pub use self::__parse__Expr::parse_Expr;

pub fn __action0<
>(
    scale: i32,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action1<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l - r
}

pub fn __action2<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l + r
}

pub fn __action3<
>(
    scale: i32,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action4<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l * r
}

pub fn __action5<
>(
    scale: i32,
    l: i32,
    _: Tok,
    r: i32,
) -> i32
{
    l / r
}

pub fn __action6<
>(
    scale: i32,
    __0: i32,
) -> i32
{
    (__0)
}

pub fn __action7<
>(
    scale: i32,
    n: i32,
) -> i32
{
    n * scale
}

pub fn __action8<
>(
    scale: i32,
    _: Tok,
    __0: i32,
    _: Tok,
) -> i32
{
    (__0)
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<((),Tok,()),Self::Error>;
}

impl<> __ToTriple<> for Tok {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        Ok(((), value, ()))
    }
}
impl<> __ToTriple<> for Result<(Tok),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<((),Tok,()),()> {
        value.map(|v| ((), v, ()))
    }
}
