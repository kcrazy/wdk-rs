use super::Vec;
use crate::allocator::POOL_TYPE;
use crate::error::Error;

pub trait TryCollect<I> {
    fn try_collect<C: TryFromIterator<I>>(self, pool_type: POOL_TYPE, tag: u32)
        -> Result<C, Error>;
}

impl<I, T> TryCollect<I> for T
where
    T: IntoIterator<Item = I>,
{
    #[inline(always)]
    fn try_collect<C: TryFromIterator<I>>(
        self,
        pool_type: POOL_TYPE,
        tag: u32,
    ) -> Result<C, Error> {
        C::try_from_iterator(self, pool_type, tag)
    }
}

pub trait TryFromIterator<I>: Sized {
    fn try_from_iterator<T: IntoIterator<Item = I>>(
        iterator: T,
        pool_type: POOL_TYPE,
        tag: u32,
    ) -> Result<Self, Error>;
}

impl<I> TryFromIterator<I> for Vec<I> {
    fn try_from_iterator<T: IntoIterator<Item = I>>(
        iterator: T,
        pool_type: POOL_TYPE,
        tag: u32,
    ) -> Result<Self, Error>
    where
        T: IntoIterator<Item = I>,
    {
        let mut new = Self::new(pool_type, tag)?;
        for i in iterator {
            new.push(i)?;
        }
        Ok(new)
    }
}
