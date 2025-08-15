pub struct BuilderTable<T: 'static + ?Sized>(pub &'static [&'static T]);

impl<T: 'static + ?Sized> BuilderTable<T> {
    pub fn iter(&self) -> impl Iterator<Item = &'static T> + '_ {
        self.0.iter().copied()
    }
}

impl<T: 'static + ?Sized> IntoIterator for &BuilderTable<T> {
    type Item = &'static T;

    type IntoIter = std::iter::Copied<std::slice::Iter<'static, &'static T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

pub struct Table<T>(Box<[T]>);

impl<T> Table<T> {
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.0.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.0.get_mut(idx)
    }

    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {
        unsafe {
            self.0.get_unchecked(idx)
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut T {
        unsafe {
            self.0.get_unchecked_mut(idx)
        }
    }
}