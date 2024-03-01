use super::*;

impl Context {

    /// TODO: Typecheck pred
    pub fn select<A: Into<NodeIdentifier> + Copy, B: Into<NodeIdentifier> + Copy, C: Into<NodeIdentifier> + Copy>(
        &mut self,
        pred: A,
        on_true: B,
        on_false: C
    ) -> Result<NodeIdentifier> {
        let pred = pred.into();
        let pred = self.stop_gradient(pred);
        let on_true = on_true.into();
        let on_false = on_false.into();
        let node_pred = &self.nodes[pred];
        let node_true = &self.nodes[on_true];
        let node_false = &self.nodes[on_false];

        if node_true.dtype != node_false.dtype {
            Err(ContextError::IncompatibleOperandTypes(
                node_true.dtype,
                node_false.dtype,
                callsite!(1),
            ))
        } else {
            match node_true.shape.broadcast(&node_false.shape) {
                None => Err(ContextError::IncompatibleOperandShapes(
                    node_true.shape.clone(),
                    node_false.shape.clone(),
                    callsite!(1),
                )),
                Some(s) => {
                    match s.broadcast(&node_pred.shape) {
                        None => Err(ContextError::IncompatibleOperandShapes(
                            s,
                            node_pred.shape.clone(),
                            callsite!(1),
                        )),
                        Some(sh) => {
                            let node = Node {
                                callsite: callsite!(1),
                                shape: sh,
                                operation: Operation::Select{ pred: pred, on_true: on_true, on_false: on_false },
                                dtype: node_true.dtype,
                            };
                            let node_id = self.nodes.insert(node);
                            self.dependent_nodes.entry(pred).or_insert(Vec::new()).push(node_id);
                            self.dependent_nodes.entry(on_true).or_insert(Vec::new()).push(node_id);
                            if on_true != on_false {
                                self.dependent_nodes.entry(on_false).or_insert(Vec::new()).push(node_id);
                            }
                            Ok(node_id)
                        }
                    }
                }
            }
        }
    }
}