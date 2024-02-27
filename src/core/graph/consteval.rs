use xla::Literal;

use super::*;

impl Context {
    /// Folds constants in place by replacing any node whose both inputs are Constant
    /// with a Constant of the result of the operation. All existing references to
    /// the old node will still point to it once its replaced, and this process is
    /// repeated until there are no more nodes whose inputs are all constants.
    pub(crate) fn foldconsts<A: Into<NodeIdentifier> + Copy>(
        &mut self,
        input: A,
        modification_limit: usize,
    ) -> Result<bool> {
        if modification_limit == 0 {
            return Ok(true);
        }
        // TODO: implement this
        let input_node = &mut self.nodes[input.into()];
        return match input_node.operation {
            Operation::Add(a, b) => {
                let node_a = &self.nodes[a];
                let node_b = &self.nodes[b];
                

                if node_a.is_zero()? || node_b.is_zero()? {

                    let a_val = node_a.is_const();
                    let b_val = node_b.is_const();
                    
                    let new_val: Literal;

                    if b_val.is_some() && node_b.is_zero()? {

                    } else {

                    }
                }
 
                //TODO at some point possible, for now we're only checkng 
                //for mul by zero/one and adding by 0
                /*if a_const.is_some() && b_const.is_some() {
                    let (a,b) = (a_const.unwrap(), b_const.unwrap());
                    //TODO: Do replacement
                    
                }*/
                Ok(false)
            }
            Operation::Mul(a, b) => {
                let node_a = &self.nodes[a];
                let node_b = &self.nodes[b];

                let a_const = node_a.is_const();
                let b_const = node_b.is_const();

                if a_const.is_some() || b_const.is_some() {
                    //TODO: Do replacement
                    
                } else if node_a.is_zero()? || node_b.is_zero()? {
                    //TODO: x * 0 situation, make it zero
                    
                }

                Ok(false)
            }
            _ =>
            //TODO: Not fully sure if const folding needs to happen when the
            //operation isn't addition or multiplication, returnign false
            //if the operation isn't either of these for now, but definitely
            //let me know if this should be other behavior
            {
                Ok(false)
            }
        };
    }
}
