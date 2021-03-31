use std::{marker::PhantomData, num::NonZeroI64};

use typenum::Unsigned;

use crate::{currency::Currency, dense::DenseMoney};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DiscreteMoney<C: Currency> {
    amount: u32,
    phantom: PhantomData<C>,
}

impl<C: Currency> DiscreteMoney<C> {
    pub fn new(amount: u32) -> DiscreteMoney<C> {
        DiscreteMoney {
            amount,
            phantom: PhantomData,
        }
    }

    pub fn to_dense(&self) -> DenseMoney<C> {
        self.into()
    }
}

impl<C: Currency> From<&DiscreteMoney<C>> for DenseMoney<C> {
    fn from(discrete: &DiscreteMoney<C>) -> Self {
        let numer: i64 = discrete.amount.into();
        // Safety: The `NonZero` bound on `C::MinorUnits` guarantees `C::MinorUnits::to_i64() != 0`.
        let denom = unsafe { NonZeroI64::new_unchecked(C::MinorUnits::to_i64()) };
        C::new(numer, denom)
    }
}
