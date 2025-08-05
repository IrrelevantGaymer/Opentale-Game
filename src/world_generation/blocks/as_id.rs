#[const_trait]
pub trait AsId {
    type Name;
    const NAME: Self::Name;

    fn from_id(id: usize) -> Self;
    fn to_id(&self) -> usize;
    fn get_id_span() -> usize;

    fn to_string(&self) -> String;
}

impl AsId for () {
    type Name = ();
    const NAME: Self::Name = ();

    fn from_id(id: usize) -> Self {
        if id == 0 {
            return ();
        }
        panic!("{id} is an invalid Id for ()");
    }

    fn to_id(&self) -> usize {
        0
    }

    fn get_id_span() -> usize {
        1
    }

    fn to_string(&self) -> String {
        "".to_string()
    }
}

impl<T: AsId, U: AsId> AsId for (T, U) {
    type Name = (T::Name, U::Name);
    const NAME: Self::Name = (T::NAME, U::NAME);

    fn from_id(id: usize) -> Self {
        (T::from_id(id / U::get_id_span()), U::from_id(id % U::get_id_span()))
    }

    fn to_id(&self) -> usize {
        let (t, u) = self;
        t.to_id() * U::get_id_span() + u.to_id()
    }

    fn get_id_span() -> usize {
        T::get_id_span() * U::get_id_span()
    }

    fn to_string(&self) -> String {
        let (t, u) = self;
        if T::get_id_span() == 1 && U::get_id_span() == 1 {
            "".to_string()
        } else if T::get_id_span() == 1 {
            u.to_string()
        } else if U::get_id_span() == 1 {
            t.to_string()
        } else {
            t.to_string() + ", " + u.to_string().as_str()
        }
    }
}