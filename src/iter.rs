use Node;
use NodeMut;
use unbox::Unbox;

#[derive(PartialEq)]
enum IterAction {
    Left,
    Right,
}

pub struct Iter<'a, T>
    where T: Node + 'a
{
    stack: Vec<(&'a T, IterAction)>,
}

impl<'a, T> Iter<'a, T>
    where T: Node + 'a
{
    pub fn new(root: Option<&'a T>) -> Iter<'a, T> {
        Iter { stack: root.map_or(vec![], |node| vec![(node, IterAction::Left)]) }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: Node + 'a
{
    type Item = &'a T::Value;

    fn next(&mut self) -> Option<&'a T::Value> {
        if let Some((mut subtree, action)) = self.stack.pop() {
            if action == IterAction::Left {
                while let Some(st) = subtree.left() {
                    self.stack.push((&*subtree, IterAction::Right));
                    subtree = st;
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

pub struct IntoIter<T>
    where T: NodeMut,
          T::NodePtr: Unbox<T>
{
    stack: Vec<(T::NodePtr, IterAction)>,
}

impl<T> IntoIter<T>
    where T: NodeMut,
          T::NodePtr: Unbox<T>
{
    pub fn new(root: Option<T::NodePtr>) -> IntoIter<T> {
        IntoIter { stack: root.map_or(vec![], |node| vec![(node, IterAction::Left)]) }
    }
}

impl<T> Iterator for IntoIter<T>
    where T: NodeMut,
          T::NodePtr: Unbox<T>
{
    type Item = T::Value;

    fn next(&mut self) -> Option<T::Value> {
        if let Some((mut subtree, action)) = self.stack.pop() {
            if action == IterAction::Left {
                while let Some(st) = subtree.detach_left() {
                    self.stack.push((subtree, IterAction::Right));
                    subtree = st;
                }
            }
            if let Some(st) = subtree.detach_right() {
                self.stack.push((st, IterAction::Left));
            }
            Some(subtree.unbox().value_owned())
        } else {
            None
        }
    }
}

impl<T> Drop for IntoIter<T>
    where T: NodeMut,
          T::NodePtr: Unbox<T>
{
    fn drop(&mut self) {
        for _ in self {}
    }
}

#[cfg(test)]
mod tests {
    use NodeMut;
    use test::TestNode;
    use super::Iter;
    use super::IntoIter;

    #[test]
    fn iteration() {
        let mut ct = Box::new(TestNode::new(7));
        let mut ct_l = Box::new(TestNode::new(8));
        ct_l.insert_right(Some(Box::new(TestNode::new(12))));
        ct.insert_left(Some(ct_l));
        ct.insert_right(Some(Box::new(TestNode::new(5))));

        {
            let vals: Vec<_> = Iter::new(Some(&*ct)).collect();
            assert_eq!(vals, [&8, &12, &7, &5]);
        }

        let node_mi: IntoIter<TestNode<_>> = IntoIter::new(Some(ct));
        let vals: Vec<_> = node_mi.collect();
        assert_eq!(vals, [8, 12, 7, 5]);
    }
}
