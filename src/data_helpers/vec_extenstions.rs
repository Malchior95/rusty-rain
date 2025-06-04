use super::to_string::ToString;

impl<T> ToString for Vec<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {
        self.iter()
            .fold(String::new(), |acc, i| format!("{} {}", acc, i.to_string()))
    }
}
