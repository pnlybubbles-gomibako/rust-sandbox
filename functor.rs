
trait Functor<T> {
  fn fmap<F, S>(self, F) -> Self<S> where F: Fn(T) -> T;
}
