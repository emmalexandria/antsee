pub trait Styleable<T>
where
    T: AsRef<str>,
{
    fn style(&self, style: Style) -> T {}
}

impl<T> Styleable<T> for T where T: AsRef<str> {}

pub struct Style {}
