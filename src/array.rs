use miniscript::iter::{Tree, TreeLike};

/// View of a slice as a balanced binary tree of its elements.
/// The slice must be nonempty.
#[derive(Debug, Copy, Clone)]
pub struct BTreeSlice<'a, A>(&'a [A]);

impl<'a, A> BTreeSlice<'a, A> {
    /// View the slice as a balanced binary tree of its elements.
    ///
    /// ## Panics
    ///
    /// The slice is empty.
    pub fn from_slice(slice: &'a [A]) -> Self {
        assert!(!slice.is_empty(), "Slice must be nonempty");
        Self(slice)
    }
}

impl<'a, A: Clone> BTreeSlice<'a, A> {
    /// Fold the tree in post order, using the binary function `f`.
    pub fn fold<F>(self, f: F) -> A
    where
        F: Fn(A, A) -> A,
    {
        debug_assert!(!self.0.is_empty());

        let mut output = vec![];
        for item in self.post_order_iter() {
            match item.child_indices.len() {
                2 => {
                    let r = output.pop().unwrap();
                    let l = output.pop().unwrap();
                    output.push(f(l, r));
                }
                n => {
                    debug_assert!(n == 0);
                    debug_assert!(item.node.0.len() == 1);
                    output.push(item.node.0[0].clone());
                }
            }
        }

        debug_assert!(output.len() == 1);
        output.pop().unwrap()
    }
}

impl<'a, A: Clone> TreeLike for BTreeSlice<'a, A> {
    fn as_node(&self) -> Tree<Self> {
        match self.0.len() {
            0 => unreachable!("Empty slice"),
            1 => Tree::Nullary,
            n => {
                let next_pow2 = n.next_power_of_two();
                debug_assert!(0 < next_pow2 / 2);
                debug_assert!(0 < n - next_pow2 / 2);
                let half = next_pow2 / 2;
                let left = BTreeSlice::from_slice(&self.0[..half]);
                let right = BTreeSlice::from_slice(&self.0[half..]);
                Tree::Binary(left, right)
            }
        }
    }
}
