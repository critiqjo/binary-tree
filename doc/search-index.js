var searchIndex = {};
searchIndex["binary_tree"] = {"doc":"Provides a collection of binary tree based data structures and algorithms.","items":[[4,"WalkAction","binary_tree","List of actions during a `Node::walk` or `NodeMut::walk_*`.",null,null],[13,"Left","","Enter(ed) the left child",0,null],[13,"Right","","Enter(ed) the right child",0,null],[13,"Stop","","Stop walking",0,null],[0,"cow","","Copy-on-Write pointers.",null,null],[3,"RcCow","binary_tree::cow","",null,null],[12,"0","","",1,null],[3,"ArcCow","","",null,null],[12,"0","","",2,null],[11,"new","","",1,{"inputs":[{"name":"t"}],"output":{"name":"rccow"}}],[11,"clone","","",1,{"inputs":[{"name":"rccow"}],"output":{"name":"rccow"}}],[11,"deref","","",1,{"inputs":[{"name":"rccow"}],"output":{"name":"t"}}],[11,"deref_mut","","",1,{"inputs":[{"name":"rccow"}],"output":{"name":"t"}}],[11,"new","","",2,{"inputs":[{"name":"t"}],"output":{"name":"arccow"}}],[11,"clone","","",2,{"inputs":[{"name":"arccow"}],"output":{"name":"arccow"}}],[11,"deref","","",2,{"inputs":[{"name":"arccow"}],"output":{"name":"t"}}],[11,"deref_mut","","",2,{"inputs":[{"name":"arccow"}],"output":{"name":"t"}}],[0,"count","binary_tree","Counting tree implementation.",null,null],[3,"CountTree","binary_tree::count","Counting tree.",null,null],[3,"Iter","","",null,null],[3,"IntoIter","","",null,null],[3,"CountNode","","Node of a `CountTree`.",null,null],[6,"NodePtr","","",null,null],[11,"new","","Returns an empty `CountTree`",3,{"inputs":[],"output":{"name":"counttree"}}],[11,"is_empty","","Returns `true` if the tree contains no elements.",3,{"inputs":[{"name":"counttree"}],"output":{"name":"bool"}}],[11,"len","","Returns the number elements in the tree. Time complexity: O(1)",3,{"inputs":[{"name":"counttree"}],"output":{"name":"usize"}}],[11,"clear","","Clears the tree, dropping all elements iteratively.",3,{"inputs":[{"name":"counttree"}],"output":null}],[11,"get","","Returns the element at the given index, or `None` if index is out of\nbounds. Time complexity: O(log(n))",3,{"inputs":[{"name":"counttree"},{"name":"usize"}],"output":{"name":"option"}}],[11,"get_mut","","Returns a mutable reference to the element at the given index, or `None`\nif out of bounds. Time complexity: O(log(n))",3,{"inputs":[{"name":"counttree"},{"name":"usize"}],"output":{"name":"option"}}],[11,"insert","","Inserts an element at the given index. Time complexity: O(log(n))",3,{"inputs":[{"name":"counttree"},{"name":"usize"},{"name":"t"}],"output":null}],[11,"push_front","","Prepends an element at the beginning.",3,{"inputs":[{"name":"counttree"},{"name":"t"}],"output":null}],[11,"push_back","","Appends an element at the end.",3,{"inputs":[{"name":"counttree"},{"name":"t"}],"output":null}],[11,"remove","","Removes the element at the given index. Time complexity: O(log(n))",3,{"inputs":[{"name":"counttree"},{"name":"usize"}],"output":{"name":"t"}}],[11,"pop_front","","Removes and returns the first element, or `None` if empty.",3,{"inputs":[{"name":"counttree"}],"output":{"name":"option"}}],[11,"pop_back","","Removes and returns the last element, or `None` if empty.",3,{"inputs":[{"name":"counttree"}],"output":{"name":"option"}}],[11,"root","","",3,{"inputs":[{"name":"counttree"}],"output":{"name":"option"}}],[11,"fmt","","",3,{"inputs":[{"name":"counttree"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"drop","","",3,{"inputs":[{"name":"counttree"}],"output":null}],[11,"from_iter","","Time complexity: &amp;Theta;(n + log&lt;sup&gt;2&lt;/sup&gt;(n))",3,{"inputs":[{"name":"i"}],"output":{"name":"self"}}],[11,"next","","",4,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[11,"size_hint","","",4,null],[11,"into_iter","","",3,{"inputs":[{"name":"counttree"}],"output":{"name":"intoiter"}}],[11,"next","","",5,{"inputs":[{"name":"intoiter"}],"output":{"name":"option"}}],[11,"size_hint","","",5,null],[11,"left","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"option"}}],[11,"right","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"option"}}],[11,"value","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"t"}}],[11,"detach_left","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"option"}}],[11,"detach_right","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"option"}}],[11,"insert_left","","",6,{"inputs":[{"name":"countnode"},{"name":"option"}],"output":{"name":"option"}}],[11,"insert_right","","",6,{"inputs":[{"name":"countnode"},{"name":"option"}],"output":{"name":"option"}}],[11,"value_mut","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"t"}}],[11,"into_parts","","",6,null],[11,"left_mut","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"option"}}],[11,"right_mut","","",6,{"inputs":[{"name":"countnode"}],"output":{"name":"option"}}],[11,"fmt","","",6,{"inputs":[{"name":"countnode"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"iter","binary_tree","Generic iterators.",null,null],[3,"Iter","binary_tree::iter","",null,null],[3,"IntoIter","","",null,null],[11,"new","","",7,{"inputs":[{"name":"option"}],"output":{"name":"iter"}}],[11,"next","","",7,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[11,"new","","",8,{"inputs":[{"name":"option"}],"output":{"name":"intoiter"}}],[11,"next","","",8,{"inputs":[{"name":"intoiter"}],"output":{"name":"option"}}],[11,"drop","","",8,{"inputs":[{"name":"intoiter"}],"output":null}],[0,"test","binary_tree","Data structures and algorithms for testing purposes.",null,null],[3,"TestNode","binary_tree::test","A minimal `Node` implementation.",null,null],[12,"val","","",9,null],[12,"left","","",9,null],[12,"right","","",9,null],[4,"Level","","",null,null],[13,"Balanced","","",10,null],[13,"Imbalanced","","",10,null],[5,"compute_level","","Recursively calculate the level of this node and check whether it is\nbalanced.",null,{"inputs":[{"name":"n"},{"name":"u32"}],"output":{"name":"level"}}],[11,"eq","","",10,{"inputs":[{"name":"level"},{"name":"level"}],"output":{"name":"bool"}}],[11,"ne","","",10,{"inputs":[{"name":"level"},{"name":"level"}],"output":{"name":"bool"}}],[11,"fmt","","",10,{"inputs":[{"name":"level"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",10,{"inputs":[{"name":"level"}],"output":{"name":"level"}}],[11,"is_balanced","","",10,{"inputs":[{"name":"level"}],"output":{"name":"bool"}}],[11,"as_u32","","",10,{"inputs":[{"name":"level"}],"output":{"name":"u32"}}],[11,"fmt","","",9,{"inputs":[{"name":"testnode"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",9,{"inputs":[{"name":"t"}],"output":{"name":"testnode"}}],[11,"left","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"option"}}],[11,"right","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"option"}}],[11,"value","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"t"}}],[11,"detach_left","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"option"}}],[11,"detach_right","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"option"}}],[11,"insert_left","","",9,{"inputs":[{"name":"testnode"},{"name":"option"}],"output":{"name":"option"}}],[11,"insert_right","","",9,{"inputs":[{"name":"testnode"},{"name":"option"}],"output":{"name":"option"}}],[11,"value_mut","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"t"}}],[11,"into_parts","","",9,null],[11,"left_mut","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"option"}}],[11,"right_mut","","",9,{"inputs":[{"name":"testnode"}],"output":{"name":"option"}}],[0,"unbox","binary_tree","Pointer unboxing.",null,null],[8,"Unbox","binary_tree::unbox","Trait specifying unboxing capability of a pointer type.",null,null],[16,"Target","","",11,null],[10,"unbox","","",11,{"inputs":[{"name":"unbox"}],"output":{"name":"target"}}],[11,"unbox","alloc::boxed","",12,{"inputs":[{"name":"box"}],"output":{"name":"t"}}],[11,"unbox","alloc::rc","",13,{"inputs":[{"name":"rc"}],"output":{"name":"t"}}],[11,"unbox","alloc::arc","",14,{"inputs":[{"name":"arc"}],"output":{"name":"t"}}],[8,"BinaryTree","binary_tree","",null,null],[16,"Node","","",15,null],[10,"root","","",15,{"inputs":[{"name":"binarytree"}],"output":{"name":"option"}}],[8,"Node","","Generic methods for traversing a binary tree.",null,null],[16,"Value","","",16,null],[10,"left","","Get a reference to the left subtree",16,{"inputs":[{"name":"node"}],"output":{"name":"option"}}],[10,"right","","Get a reference to the right subtree",16,{"inputs":[{"name":"node"}],"output":{"name":"option"}}],[10,"value","","Returns the value of the current node.",16,{"inputs":[{"name":"node"}],"output":{"name":"value"}}],[11,"walk","","Walk down the tree",16,{"inputs":[{"name":"node"},{"name":"f"}],"output":null}],[8,"NodeMut","","Mutating methods on a Binary Tree node.",null,null],[16,"NodePtr","","",17,null],[10,"detach_left","","Try to detach the left sub-tree",17,{"inputs":[{"name":"nodemut"}],"output":{"name":"option"}}],[10,"detach_right","","Try to detach the right sub-tree",17,{"inputs":[{"name":"nodemut"}],"output":{"name":"option"}}],[10,"insert_left","","Replace the left subtree with `tree` and return the old one.",17,{"inputs":[{"name":"nodemut"},{"name":"option"}],"output":{"name":"option"}}],[10,"insert_right","","Replace the right subtree with `tree` and return the old one.",17,{"inputs":[{"name":"nodemut"},{"name":"option"}],"output":{"name":"option"}}],[10,"value_mut","","Returns a mutable reference to the value of the current node.",17,{"inputs":[{"name":"nodemut"}],"output":{"name":"value"}}],[10,"into_parts","","Consume a Node and return its parts: (value, left, right)",17,null],[10,"left_mut","","Returns a mutable reference to the left child",17,{"inputs":[{"name":"nodemut"}],"output":{"name":"option"}}],[10,"right_mut","","Returns a mutable reference to the right child",17,{"inputs":[{"name":"nodemut"}],"output":{"name":"option"}}],[11,"rotate_left","","Try to rotate the tree left if right subtree exists",17,{"inputs":[{"name":"nodemut"}],"output":{"name":"result"}}],[11,"rotate_right","","Try to rotate the tree right if left subtree exists",17,{"inputs":[{"name":"nodemut"}],"output":{"name":"result"}}],[11,"walk_mut","","Simple mutable walk",17,{"inputs":[{"name":"nodemut"},{"name":"fi"},{"name":"fs"}],"output":null}],[11,"walk_reshape","","Walks down the tree by detaching subtrees, then up reattaching them\nback. `step_in` should guide the path taken, `stop` will be called on\nthe node where either `step_in` returned `Stop` or it was not possible\nto proceed. Then `step_out` will be called for each node along the way\nto root, except the final one (that for which `stop` was called).",17,{"inputs":[{"name":"nodemut"},{"name":"fi"},{"name":"fs"},{"name":"fo"}],"output":null}],[11,"insert_before","","Insert `new_node` in-order before `self`. `step_out` will be invoked for\nall nodes in path from (excluding) the point of insertion, to\n(including) `self`, unless `self` is the point of insertion.",17,{"inputs":[{"name":"nodemut"},{"name":"nodeptr"},{"name":"f"}],"output":null}],[11,"walk_extract","","Extract out a node. This can be used in conjuction with `try_remove` to\nremove any node except the root.",17,{"inputs":[{"name":"nodemut"},{"name":"fi"},{"name":"fe"},{"name":"fo"}],"output":{"name":"option"}}],[11,"try_remove","","Replace this node with one of its descendant, returns `None` if it has\nno children.",17,{"inputs":[{"name":"nodemut"},{"name":"f"}],"output":{"name":"option"}}],[11,"eq","","",0,{"inputs":[{"name":"walkaction"},{"name":"walkaction"}],"output":{"name":"bool"}}],[11,"clone","","",0,{"inputs":[{"name":"walkaction"}],"output":{"name":"walkaction"}}]],"paths":[[4,"WalkAction"],[3,"RcCow"],[3,"ArcCow"],[3,"CountTree"],[3,"Iter"],[3,"IntoIter"],[3,"CountNode"],[3,"Iter"],[3,"IntoIter"],[3,"TestNode"],[4,"Level"],[8,"Unbox"],[3,"Box"],[3,"Rc"],[3,"Arc"],[8,"BinaryTree"],[8,"Node"],[8,"NodeMut"]]};
initSearch(searchIndex);