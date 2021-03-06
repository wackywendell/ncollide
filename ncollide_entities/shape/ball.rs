
/// A Ball shape.
#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Ball<N> {
    radius: N
}

impl<N: Clone> Ball<N> {
    /// Creates a new ball from its radius and center.
    #[inline]
    pub fn new(radius: N) -> Ball<N> {
        Ball {
            radius: radius
        }
    }

    /// The ball radius.
    #[inline]
    pub fn radius(&self) -> N {
        self.radius.clone()
    }
}
