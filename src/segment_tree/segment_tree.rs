use algebra::*;

pub struct SegmentTree<T: Monoid> {
    node: Vec<T>,
    sz: usize,
}

impl<T: Monoid> SegmentTree<T> {
    pub fn init(vec: Vec<T>) -> Self {
        let mut sz = 1;
        while sz < vec.len() { sz *= 2; }
        let mut node = vec![T::identity(); sz << 1];
        for i in 0..vec.len() { node[i + sz] = vec[i].clone(); }
        for i in (1..sz).rev() { node[i] = node[i << 1].op(&node[(i << 1) + 1]); }
        SegmentTree { node: node, sz: sz }
    }

    pub fn update(&mut self, i: usize, x: T) {
        let mut idx = i + self.sz;
        self.node[idx] = x;
        while idx > 1 {
            idx = idx >> 1;
            self.node[idx] = self.node[idx << 1].op(&self.node[(idx << 1)  + 1]);
        }
    }

    pub fn fold(&self, left: usize, right: usize) -> T {
        let mut lx = T::identity();
        let mut rx = T::identity();
        let mut l = left + self.sz;
        let mut r = right + self.sz;
        while l < r {
            if (l & 1) == 1 { lx = lx.op(&self.node[l]); }
            if (r & 1) == 0 { rx = self.node[r].op(&rx); }
            l = (l + 1) >> 1;
            r = (r - 1) >> 1;
        }
        if l == r { lx = lx.op(&self.node[l]); }
        lx.op(&rx)
    }
}

#[cfg(test)]
mod rsq_test {
    use algebra::*;
    use segment_tree::segment_tree::SegmentTree;

    #[derive(Clone)]
    struct Am(usize);

    impl Magma for Am {
        fn op(&self, right: &Self) -> Self { Am(self.0 + right.0) }
    }
    impl Associative for Am {}

    impl Unital for Am {
        fn identity() -> Self { Am(0) }
    }
    #[test]
    fn rsq_test() {
        let seg = SegmentTree::init(vec![Am(1), Am(2), Am(3)]);
        assert!(seg.fold(0, 1).0 == 3);
        assert!(seg.fold(1, 1).0 == 2);
    }
    #[test]
    fn corner_test() {
        let seg = SegmentTree::init(vec![Am(1)]);
        assert!(seg.fold(0, 0).0 == 1);
    }
}
