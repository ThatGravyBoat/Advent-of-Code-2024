pub trait IndexOf<A> {

    fn index_of(&self, value: &A) -> i32;
}

impl <A : Eq> IndexOf<A> for Vec<A> {

    fn index_of(&self, value: &A) -> i32 {
        self.iter().position(|it| it == value).map(|it| it as i32).unwrap_or(-1)
    }
}