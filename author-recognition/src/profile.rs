#[derive(Default)]
pub struct Profile<T: ProfileData + Default> {
    pub author: String,
    pub data: T,
}

impl<T: ProfileData + Default> Profile<T> {
    pub fn new(author: &str) -> Self {
        Self {
            author: author.to_owned(),
            data: T::default(),
        }
    }
}

pub trait ProfileData {
    fn process(&mut self, txt: &str);
    fn check_difference(
        &self,
        other: &Self,
        evaluation_function: fn(own: &Self, other: &Self) -> f64,
    ) -> f64;
}
