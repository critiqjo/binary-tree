use Node;
use NodeMut;

#[derive(PartialEq)]
enum IterAction {
    Left,
    Right,
}

pub struct NodeIter<'a, T>
    where T: Node + 'a
{
    stack: Vec<(&'a T, IterAction)>,
}

impl<'a, T> NodeIter<'a, T>
    where T: Node + 'a
{
    pub fn new(tree: &'a T) -> NodeIter<'a, T> {
        NodeIter {
            stack: vec![(tree, IterAction::Left)],
        }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T>
    where T: Node + 'a
{
    type Item = &'a T::Value;

    fn next(&mut self) -> Option<&'a T::Value> {
        if let Some((mut subtree, action)) = self.stack.pop() {
            if action == IterAction::Left {
                loop {
                    if let Some(st) = subtree.left() {
                        self.stack.push((&*subtree, IterAction::Right));
                        subtree = st;
                    } else {
                        break;
                    }
                }
            }
            if let Some(st) = subtree.right() {
                self.stack.push((&*st, IterAction::Left));
            }
            Some(subtree.value())
        } else {
            None
        }
    }
}

pub struct NodeMutIter<T>
    where T: NodeMut
{
    stack: Vec<(T::NodePtr, IterAction)>,
}

impl<T> NodeMutIter<T>
    where T: NodeMut
{
    pub fn new(tree: T::NodePtr) -> NodeMutIter<T> {
        NodeMutIter {
            stack: vec![(tree, IterAction::Left)],
        }
    }
}

impl<T> Iterator for NodeMutIter<T>
    where T: NodeMut
{
    type Item = T::Value;

    fn next(&mut self) -> Option<T::Value> {
        if let Some((mut subtree, action)) = self.stack.pop() {
            if action == IterAction::Left {
                loop {
                    if let Some(st) = subtree.detach_left() {
                        self.stack.push((subtree, IterAction::Right));
                        subtree = st;
                    } else {
                        break;
                    }
                }
            }
            if let Some(st) = subtree.detach_right() {
                self.stack.push((st, IterAction::Left));
            }
            Some(T::value_owned(subtree))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use NodeMut;
    use count::CountNode;
    use super::NodeIter;

    #[test]
    fn iteration() {
        let mut ct = CountNode::new(7);
        let mut ct_l = CountNode::new(8);
        ct_l.insert_right(Some(Box::new(CountNode::new(12))));
        ct.insert_left(Some(Box::new(ct_l)));
        ct.insert_right(Some(Box::new(CountNode::new(5))));

        let vals: Vec<_> = NodeIter::new(&ct).map(|v| *v).collect();
        assert_eq!(vals, [8, 12, 7, 5]);
    }
}
