use simple_error::SimpleError;

pub struct VecTuple1<T>(pub T, pub Vec<T>);
pub struct VecTuple2<T>(pub (T, T), pub Vec<T>);

impl<T> TryFrom<Vec<T>> for VecTuple1<T> {
    type Error = SimpleError;

    fn try_from(mut value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(SimpleError::new(String::from("Tried to convert a 0 len vec to a VecTuple1")));
        }
        let first = value.remove(0);
        Ok(VecTuple1 {
            0: first,
            1: value,
        })
    }
}
impl<T> TryFrom<Vec<T>> for VecTuple2<T> {
    type Error = SimpleError;

    fn try_from(mut value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err(SimpleError::new(String::from("Tried to convert a < 2 len vec to a VecTuple2")));
        }
        let first = value.remove(0);
        let second = value.remove(1);
        Ok(VecTuple2 {
            0: (first, second),
            1: value,
        })
    }
}