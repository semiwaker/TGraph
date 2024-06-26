// use super::*;

/// Get a node from the graph, assume it is $var variant of the NodeEnum. Returns `Option<&NodeType>`
///
/// # Example
/// ```
/// use tgraph::*;
///
/// #[derive(TypedNode)]
/// struct NodeA{
///   a: usize
/// }
///
/// node_enum!{
///   enum MyNodeEnum{
///     A(NodeA)
///   }
/// }
///
/// let ctx = Context::new();
/// let mut graph = Graph::<MyNodeEnum>::new(&ctx);
/// let mut trans = Transaction::new(&ctx);
///
/// let x = trans.new_node(MyNodeEnum::A(NodeA{ a: 1 }));
/// graph.commit(trans);
///
/// // a: Option<&NodeA>
/// let a = get_node!(graph, MyNodeEnum::A, x);
/// assert!(a.is_some());
/// assert_eq!(a.unwrap().a, 1);
/// ```
#[macro_export]
macro_rules! get_node {
  ($graph: expr, $var: path, $idx: expr) => {
    if let Some($var(x)) = $graph.get_node($idx) {
      Some(x)
    } else {
      None
    }
  };
}

/// Iterate a type of nodes from the graph, assume they are $var variant of the NodeEnum. `Returns impl Iterator<Item = (NodeIndex, &NodeType)>`
///
/// # Example
/// ```
/// use tgraph::*;
///
/// #[derive(TypedNode)]
/// struct NodeA{
///   a: usize
/// }
/// #[derive(TypedNode)]
/// struct NodeB{
///   b: usize
/// }
///
/// node_enum!{
///   enum MyNodeEnum{
///     A(NodeA),
///     B(NodeB),
///   }
/// }
///
/// let ctx = Context::new();
/// let mut graph = Graph::<MyNodeEnum>::new(&ctx);
/// let mut trans = Transaction::new(&ctx);
///
/// trans.new_node(MyNodeEnum::A(NodeA{ a: 1 }));
/// trans.new_node(MyNodeEnum::A(NodeA{ a: 2 }));
/// trans.new_node(MyNodeEnum::B(NodeB{ b: 0 }));
/// graph.commit(trans);
///
/// let iterator = iter_nodes!(graph, MyNodeEnum::A);
/// // a: (NodeIndex, &NodeA)
/// for (i, a) in (1..3).zip(iterator) {
///   assert_eq!(i, a.1.a)
/// }
/// ```
#[macro_export]
macro_rules! iter_nodes {
  ($graph: expr, $var: path) => {
    $graph.iter_nodes().filter_map(
      |(idx, node)| {
        if let $var(x) = node {
          Some((idx, x))
        } else {
          None
        }
      },
    )
  };
}

/// Use the [`mut_node`](crate::Transaction::<T>::mut_node) method of the transaction, assume the node is $var variant of the NodeEnum.
/// Panics if the enum does not match.
///
/// # Example
/// ```
/// use tgraph::*;
///
/// #[derive(TypedNode)]
/// struct NodeA{
///   a: usize
/// }
///
/// node_enum!{
///   enum MyNodeEnum{
///     A(NodeA)
///   }
/// }
///
/// let ctx = Context::new();
/// let mut graph = Graph::<MyNodeEnum>::new(&ctx);
/// let mut trans = Transaction::new(&ctx);
///
/// let id = trans.new_node(MyNodeEnum::A(NodeA{ a: 1 }));
/// graph.commit(trans);
///
/// trans = Transaction::new(&ctx);
/// // It is similar to this closure |x: &mut NodeA| {x.a =2 }
/// mut_node!(trans, MyNodeEnum::A, id, x, {
///   x.a = 2;
/// });
/// graph.commit(trans);
///
/// let a = get_node!(graph, MyNodeEnum::A, id);
/// assert!(a.is_some());
/// assert_eq!(a.unwrap().a, 2);
/// ```
#[macro_export]
macro_rules! mut_node {
  ($transaction: expr, $var: path, $idx: expr, $node: ident, $func: block) => {
    $transaction.mut_node($idx, |x| {
      if let $var($node) = x {
        $func;
      } else {
        panic!("Type does not match!");
      }
    })
  };
}

/// Use the [`update_node`](crate::Transaction::<T>::update_node) method of the transaction, assume the node is $var variant of the NodeEnum.
/// Panics if the enum does not match.
///
/// # Example
/// ```
/// use tgraph::*;
///
/// #[derive(TypedNode)]
/// struct NodeA{
///   a: usize
/// }
///
/// node_enum!{
///   enum MyNodeEnum{
///     A(NodeA)
///   }
/// }
///
/// let ctx = Context::new();
/// let mut graph = Graph::<MyNodeEnum>::new(&ctx);
/// let mut trans = Transaction::new(&ctx);
///
/// let id = trans.new_node(MyNodeEnum::A(NodeA{ a: 1 }));
/// graph.commit(trans);
///
/// trans = Transaction::new(&ctx);
/// // It is similar to this closure |x: NodeA| { NodeA{ a: x.a + 1 } }
/// update_node!(trans, MyNodeEnum::A, id, x, {
///   NodeA {
///     a: x.a + 1,
///   }
/// });
/// graph.commit(trans);
///
/// let a = get_node!(graph, MyNodeEnum::A, id);
/// assert!(a.is_some());
/// assert_eq!(a.unwrap().a, 2);
/// ```
#[macro_export]
macro_rules! update_node {
  ($transaction: expr, $var: path, $idx: expr, $node: ident, $func: block) => {
    $transaction.update_node($idx, |x| {
      if let $var($node) = x {
        $var($func)
      } else {
        panic!("Type does not match!");
      }
    })
  };
}
