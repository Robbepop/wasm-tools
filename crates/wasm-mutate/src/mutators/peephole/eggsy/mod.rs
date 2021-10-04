use std::{cmp::Ordering, collections::HashMap, num::Wrapping};

use egg::{define_language, Analysis, CostFunction, EClass, EGraph, Id, Language, RecExpr, Symbol};
use rand::{
    prelude::{SliceRandom, SmallRng},
    Rng,
};
use wasm_encoder::{Function, Instruction};
use wasmparser::Operator;

pub mod analysis;
pub mod encoder;
pub mod lang;

use crate::{error::EitherType, mutators::peephole::eggsy::lang::Lang, ModuleInfo};

use super::{
    dfg::{BBlock, MiniDFG, StackEntry},
    OperatorAndByteOffset,
};

/// This struct is a wrapper of egg::Extractor
/// The majority of the methods are copied and adapted to our needs
pub struct RandomExtractor<'a, CF: CostFunction<L>, L: Language, N: Analysis<L>> {
    cost_function: CF,
    costs: HashMap<Id, (CF::Cost, usize)>,
    egraph: &'a EGraph<L, N>,
}

fn cmp<T: PartialOrd>(a: &Option<T>, b: &Option<T>) -> Ordering {
    // None is high
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(a), Some(b)) => a.partial_cmp(&b).unwrap(),
    }
}

impl<'a, CF, L, N> RandomExtractor<'a, CF, L, N>
where
    CF: CostFunction<L>,
    L: Language,
    N: Analysis<L, Data = Option<i32>>, // The analysis should return the index of the node in the e-class
{
    /// Returns a new Random extractor from an egraph and a custom cost function
    pub fn new(egraph: &'a EGraph<L, N>, cost_function: CF) -> Self {
        let costs = HashMap::default();

        let mut extractor = RandomExtractor {
            costs,
            egraph,
            cost_function,
        };
        extractor.costs = extractor.find_costs();
        extractor
    }

    fn find_costs(&mut self) -> HashMap<Id, (CF::Cost, usize)> {
        let mut costs = HashMap::new();

        let mut did_something = true;
        while did_something {
            did_something = false;

            for class in self.egraph.classes() {
                let pass = self.make_pass(&mut costs, class);
                match (costs.get(&class.id), pass) {
                    (None, Some(new)) => {
                        costs.insert(class.id, new);
                        did_something = true;
                    }
                    (Some(old), Some(new)) if new.0 < old.0 => {
                        costs.insert(class.id, new);
                        did_something = true;
                    }
                    _ => (),
                }
            }
        }

        costs
    }

    fn make_pass(
        &mut self,
        costs: &mut HashMap<Id, (CF::Cost, usize)>,
        eclass: &EClass<L, Option<i32>>,
    ) -> Option<(CF::Cost, usize)> {
        let (cost, node_idx) = eclass
            .iter()
            .enumerate()
            .map(|(i, n)| (self.node_total_cost(n, costs), i))
            .min_by(|a, b| cmp(&a.0, &b.0))
            .unwrap_or_else(|| panic!("Can't extract, eclass is empty: {:#?}", eclass));
        cost.map(|c| (c, node_idx))
    }

    fn node_total_cost(
        &mut self,
        node: &L,
        costs: &mut HashMap<Id, (CF::Cost, usize)>,
    ) -> Option<CF::Cost> {
        let egraph = self.egraph;
        let has_cost = |&id| costs.contains_key(&egraph.find(id));
        if node.children().iter().all(has_cost) {
            let costs = &costs;
            let cost_f = |id| costs[&egraph.find(id)].0.clone();
            Some(self.cost_function.cost(&node, cost_f))
        } else {
            None
        }
    }

    /// The the cost of the egraph nodes
    pub fn get_costs(&self) -> &HashMap<Id, (<CF as CostFunction<L>>::Cost, usize)> {
        &self.costs
    }

    /// Do a pre-order traversal of the e-graph. As we visit each e-class, choose
    /// one of its e-nodes at random and then do the same with its children,
    /// etc. You can imagine this process as a kind of rolling wave function
    /// collapse, where we choose a concrete expression out of the potentially
    /// infinite number of equivalent expressions each e-class represents.
    ///
    /// `worklist` constains the operands we still need to process. These are
    /// currently a pair of the parent node and its operand e-class.
    pub fn extract_random(
        &self,
        rnd: &mut rand::prelude::SmallRng,
        eclass: Id,
        max_depth: u32,
        encoder: impl Fn(Id, &Vec<&L>, &Vec<Vec<Id>>) -> RecExpr<L>,
    ) -> crate::Result<RecExpr<L>> // return the random tree, TODO, improve the way the tree is returned
    {
        // A map from a node's id to its actual node data.
        let mut id_to_node = vec![];
        // A map from a parent node id to its child operand node ids.
        let mut operands = vec![];

        // Select a random node in this e-class
        let rootidx = rnd.gen_range(0, self.egraph[eclass].nodes.len());
        println!("{:?} taken path {:?}", &self.egraph[eclass].nodes, rootidx);
        let rootnode = &self.egraph[eclass].nodes[rootidx];

        id_to_node.push(&self.egraph[eclass].nodes[rootidx]);
        operands.push(vec![]);

        let mut worklist: Vec<_> = rootnode
            .children()
            .iter()
            .rev()
            .map(|id| (eclass, 0, id, 0)) // (root, operant, depth)
            .collect();

        while let Some((parent, parentidx, &node, depth)) = worklist.pop() {
            let node_idx = if depth >= max_depth {
                // look nearest leaf path, in this case, the best in AST size
                self.costs[&node].1
            } else {
                rnd.gen_range(0, self.egraph[node].nodes.len())
            };

            let operand = Id::from(id_to_node.len());
            let operandidx = id_to_node.len();
            let last_node_id = parentidx; // id_to_node.len() - 1;

            id_to_node.push(&self.egraph[node].nodes[node_idx]);
            operands.push(vec![]);

            operands[last_node_id].push(operand);

            //let operand = &egraph[node].nodes[node_idx];

            worklist.extend(
                self.egraph[node].nodes[node_idx]
                    .children()
                    .iter()
                    .rev()
                    .map(|id| (operand, operandidx, id, depth + 1)),
            );
        }
        // Build the tree with the right language constructor
        Ok(encoder(
            Id::from(0), /* The root of the expr is the node at position 0 */
            &id_to_node,
            &operands,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        module::PrimitiveTypeInfo,
        mutators::{
            peephole::{
                dfg::DFGIcator, eggsy::encoder::Encoder, OperatorAndByteOffset, PeepholeMutator,
            },
            Mutator,
        },
        WasmMutate,
    };
    use egg::{rewrite, AstSize, Id, Pattern, RecExpr, Rewrite, Runner, Searcher};
    use rand::{prelude::SliceRandom, rngs::SmallRng, Rng, SeedableRng};
    use wasm_encoder::Function;
    use wasmparser::Parser;

    use super::RandomExtractor;
    use crate::mutators::peephole::Lang;
    use crate::mutators::peephole::PeepholeMutationAnalysis;

    #[test]
    fn test_random_generation() {
        let rules: &[Rewrite<Lang, PeepholeMutationAnalysis>] = &[
            rewrite!("unfold-2";  "?x" => "(unfold ?x)"), // Use a custom instruction-mutator for this
                                                          // rewrite!("strength-undo";  "(i32.shl ?x 1)" => "(i32.mul ?x ?x)"),
        ];

        let start = "?x".parse().unwrap();
        let runner = Runner::default().with_expr(&start).run(rules);
        let mut egraph = runner.egraph;
        let cf = AstSize;
        let mut rnd = SmallRng::seed_from_u64(4);

        // ?x is the root
        let root = egraph.add_expr(&start);
        let extractor = RandomExtractor::new(&egraph, cf);
        let encoder = Encoder::build_expr;

        let expr = extractor
            .extract_random(&mut rnd, root, 10, encoder)
            .unwrap();
    }

    #[test]
    fn test_wasm2expr() {
        let original = &wat::parse_str(
            r#"
        (module
            (memory 1)
            (func (export "exported_func") (param i32) (result i32)
                local.get 0
                i32.const 32
                i32.const 32
                i32.add
                i32.add
            )
        )
        "#,
        )
        .unwrap();

        let mut parser = Parser::new(0);
        let mut consumed = 0;

        loop {
            let (payload, size) = match parser.parse(&original[consumed..], true).unwrap() {
                wasmparser::Chunk::NeedMoreData(_) => {
                    panic!("This should not happen")
                }
                wasmparser::Chunk::Parsed { consumed, payload } => (payload, consumed),
            };

            consumed += size;

            match payload {
                wasmparser::Payload::CodeSectionEntry(reader) => {
                    let operators = reader
                        .get_operators_reader()
                        .unwrap()
                        .into_iter_with_offsets()
                        .collect::<wasmparser::Result<Vec<OperatorAndByteOffset>>>()
                        .unwrap();

                    let bb = DFGIcator::new()
                        .get_bb_from_operator(4, &operators)
                        .unwrap();

                    let roots = DFGIcator::new()
                        .get_dfg(&operators, &bb, &vec![PrimitiveTypeInfo::I32])
                        .unwrap();

                    let mut exprroot = RecExpr::<Lang>::default();
                    let (_, _) = Encoder::wasm2expr(&roots, 4, &operators, &mut exprroot).unwrap();
                }
                wasmparser::Payload::End => {
                    break;
                }
                _ => {
                    // Do nothing
                }
            }
        }
    }

    #[test]
    fn test_expr2wasm() {
        let original = &wat::parse_str(
            r#"
        (module
            (memory 1)
            (func (export "exported_func") (param i32) (result i32)
                local.get 0
                i32.const 32
                i32.const 32
                i32.add
                i32.add
            )
        )
        "#,
        )
        .unwrap();
        let wasmmutate = WasmMutate::default();
        let mut info = wasmmutate.get_module_info(original).unwrap();

        let mut parser = Parser::new(0);
        let mut rnd = SmallRng::seed_from_u64(0);
        let mut consumed = 0;

        loop {
            let (payload, size) = match parser.parse(&original[consumed..], true).unwrap() {
                wasmparser::Chunk::NeedMoreData(_) => {
                    panic!("This should not happen")
                }
                wasmparser::Chunk::Parsed { consumed, payload } => (payload, consumed),
            };

            consumed += size;

            match payload {
                wasmparser::Payload::CodeSectionEntry(reader) => {
                    let operators = reader
                        .get_operators_reader()
                        .unwrap()
                        .into_iter_with_offsets()
                        .collect::<wasmparser::Result<Vec<OperatorAndByteOffset>>>()
                        .unwrap();

                    let bb = DFGIcator::new()
                        .get_bb_from_operator(4, &operators)
                        .unwrap();

                    let roots = DFGIcator::new()
                        .get_dfg(&operators, &bb, &vec![PrimitiveTypeInfo::I32])
                        .unwrap();

                    let mut exprroot = RecExpr::<Lang>::default();
                    let (root, symbolsmap) =
                        Encoder::wasm2expr(&roots, 4, &operators, &mut exprroot).unwrap();

                    let mut newfunc = Function::new(vec![]);

                    Encoder::expr2wasm(
                        &info,
                        &mut rnd,
                        4,
                        &exprroot,
                        &mut newfunc,
                        &symbolsmap,
                        &roots,
                        &operators,
                    )
                    .unwrap();
                }
                wasmparser::Payload::End => {
                    break;
                }
                _ => {
                    // Do nothing
                }
            }
        }
    }
}
