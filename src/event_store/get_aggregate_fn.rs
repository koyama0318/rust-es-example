pub trait GetAggregateFn<Id, State, Error>: Fn(Id) -> Result<State, Error> + Copy {}
impl<F, I, S, E> GetAggregateFn<I, S, E> for F where F: Fn(I) -> Result<S, E> + Copy {}
