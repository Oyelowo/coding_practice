use std::ops::Index;

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let p = &vec[..];
    let x = &vec[2..3];

    let v = WrappedVec(vec);
    let m = &v[3usize];
    let m = &v[5i128];
    let m = &v[-5i128];
}

struct WrappedVec<T>(Vec<T>);

impl<T> Index<usize> for WrappedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index % self.0.len()]
        // todo!()
    }
}
impl<T> Index<i128> for WrappedVec<T> {
    type Output = T;

    fn index(&self, index: i128) -> &Self::Output {
        let self_len = self.0.len() as i128;
        let idx = (((index % self_len) + self_len) % self_len) as usize;
        &self.0[idx]
    }
}
