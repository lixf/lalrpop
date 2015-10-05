//! Core LR(1) state construction algorithm.

use kernel_set;
use grammar::repr::*;
use lr1::first;
use lr1::{Action, Lookahead, Item, Items, State, StateIndex, TableConstructionError};
use std::rc::Rc;
use util::{map, Multimap, Set};

#[cfg(test)] mod test;

pub fn build_lr1_states<'grammar>(grammar: &'grammar Grammar,
                                  start: NonterminalString)
                                  -> Result<Vec<State<'grammar>>,
                                            TableConstructionError<'grammar>>
{
    let lr1 = LR1::new(grammar);
    lr1.build_states(start)
}

struct LR1<'grammar> {
    grammar: &'grammar Grammar,
    first_sets: first::FirstSets,
}

impl<'grammar> LR1<'grammar> {
    fn new(grammar: &'grammar Grammar) -> LR1 {
        LR1 {
            grammar: grammar,
            first_sets: first::FirstSets::new(grammar),
        }
    }

    fn build_states(&self, start_nt: NonterminalString)
                    -> Result<Vec<State<'grammar>>, TableConstructionError<'grammar>>
    {
        let mut kernel_set = kernel_set::KernelSet::new();
        let mut states = vec![];

        // create the starting state
        kernel_set.add_state(
            self.transitive_closure(
                self.items(start_nt, 0, Lookahead::EOF)));

        while let Some(items) = kernel_set.next() {
            let index = StateIndex(states.len());
            let mut this_state = State { index: index, items: items.clone(),
                                         tokens: map(), gotos: map() };

            // group the items that we can transition into by shifting
            // over a term or nonterm
            let transitions: Multimap<Symbol, Item<'grammar>> =
                items.vec
                     .iter()
                     .filter_map(|item| item.shifted_item())
                     .collect();

            for (symbol, items) in transitions.into_iter() {
                let items = self.transitive_closure(items);
                let next_state = kernel_set.add_state(items);

                match symbol {
                    Symbol::Terminal(s) => {
                        let action = Action::Shift(next_state);
                        let prev = this_state.tokens.insert(Lookahead::Terminal(s), action);
                        assert!(prev.is_none()); // cannot have a shift/shift conflict
                    }

                    Symbol::Nonterminal(s) => {
                        let prev = this_state.gotos.insert(s, next_state);
                        if !prev.is_none() {
                            println!("this is where the shift reduce error is");
                        }
                        assert!(prev.is_none());
                    }
                }
            }

            // finally, consider the reductions
            for item in items.vec.iter().filter(|i| i.can_reduce()) {
                let action = Action::Reduce(item.production);
                let prev = this_state.tokens.insert(item.lookahead, action);
                if let Some(conflict) = prev {
                    return Err(TableConstructionError {
                        items: items.clone(),
                        lookahead: item.lookahead,
                        production: item.production,
                        conflict: conflict,
                    });
                }
            }

            // extract a new state
            states.push(this_state);
        }

        Ok(states)
    }

    fn items(&self,
             id: NonterminalString,
             index: usize,
             lookahead: Lookahead)
             -> Vec<Item<'grammar>>
    {
        self.grammar.productions_for(id)
                    .iter()
                    .map(|production| {
                        debug_assert!(index <= production.symbols.len());
                        Item { production: production,
                                        index: index,
                                        lookahead: lookahead }
                    })
                    .collect()
    }

    // expands `state` with epsilon moves
    fn transitive_closure(&self, mut items: Vec<Item<'grammar>>)
                          -> Items<'grammar>
    {
        let mut counter = 0;

        let mut set: Set<Item<'grammar>> =
            items.iter().cloned().collect();

        while counter < items.len() {
            let new_items: Vec<_> =
                items[counter..]
                .iter()
                .filter_map(|item| {
                    let shift_symbol = item.shift_symbol();
                    match shift_symbol {
                        None => None, // requires a reduce
                        Some((Symbol::Terminal(_), _)) => None, // requires a shift
                        Some((Symbol::Nonterminal(nt), remainder)) => {
                            Some((nt, remainder, item.lookahead))
                        }
                    }
                })
                .flat_map(|(nt, remainder, lookahead)| {
                    let first_set = self.first_sets.first(remainder, lookahead);
                    first_set.into_iter()
                             .flat_map(move |l| self.items(nt, 0, l))
                })
                .filter(|&item| set.insert(item))
                .collect();

            counter = items.len();
            items.extend(new_items);
        }

        items.sort();
        items.dedup();

        Items { vec: Rc::new(items) }
    }
}


