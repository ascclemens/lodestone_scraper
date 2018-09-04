crate enum Either<L, R> {
  Left(L),
  Right(R),
}

impl<L, R> std::fmt::Debug for Either<L, R>
  where L: std::fmt::Debug,
        R: std::fmt::Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Either::Left(ref l) => f.debug_tuple("Left").field(l).finish(),
      Either::Right(ref r) => f.debug_tuple("Right").field(r).finish(),
    }
  }
}
