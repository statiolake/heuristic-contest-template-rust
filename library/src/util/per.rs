#[macro_export]
macro_rules! make_per {
    ($per_ty:ident, $index_ty:ident, $max_index:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $per_ty<T> {
            data: [T; $max_index],
        }

        impl<T> $per_ty<T> {
            pub fn new(data: [T; $max_index]) -> Self {
                Self { data }
            }
        }

        impl<T: Default + Copy> Default for $per_ty<T> {
            fn default() -> Self {
                Self {
                    data: [T::default(); $max_index],
                }
            }
        }

        impl<T> std::ops::Index<$index_ty> for $per_ty<T> {
            type Output = T;

            fn index(&self, index: $index_ty) -> &Self::Output {
                &self.data[index as usize]
            }
        }

        impl<T> std::ops::IndexMut<$index_ty> for $per_ty<T> {
            fn index_mut(&mut self, index: $index_ty) -> &mut Self::Output {
                &mut self.data[index as usize]
            }
        }
    };
}
