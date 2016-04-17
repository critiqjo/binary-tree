use BinaryTree;

#[derive(PartialEq)]
enum IterAction {
    Left,
    Right,
}

pub struct Iter<'a, T>
    where T: BinaryTree + 'a
{
    stack: Vec<(&'a T, IterAction)>,
}

impl<'a, T> Iter<'a, T>
    where T: BinaryTree + 'a
{
    pub fn new(tree: &'a T) -> Iter<'a, T> {
        Iter {
            stack: vec![(tree, IterAction::Left)],
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: BinaryTree + 'a
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

#[cfg(test)]
mod tests {
    use BinaryTree;
    use count::CountTree;

    #[test]
    fn iteration() {
        let mut ct = CountTree::new(7);
        let mut ct_l = CountTree::new(8);
        ct_l.insert_right(Some(box CountTree::new(12)));
        ct.insert_left(Some(box ct_l));
        ct.insert_right(Some(box CountTree::new(5)));

        let vals: Vec<_> = (&ct).into_iter().map(|v| *v).collect();
        assert_eq!(vals, [8, 12, 7, 5]);
    }
}
