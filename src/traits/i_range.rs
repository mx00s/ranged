use crate::traits::ITyEq;
use num_traits::{Num, SaturatingSub};

pub trait IRange {
    type ValueType: Num;
    fn contains(value: &Self::ValueType) -> bool
    where
        Self: Sized;
}

pub trait IRangeFrom: IRange + IntoIterator<Item = <Self as IRange>::ValueType> {
    fn start() -> <Self as IRange>::ValueType;
}

pub trait IRangeTo: IRange {
    fn end() -> <Self as IRange>::ValueType;
}

pub trait IRangeFinite: IRange + IRangeFrom + IRangeTo {}

pub trait IRangeToInclusive: IRangeFinite {}

pub trait IRangeLen<TValue>: IRange + IRangeFrom + IRangeTo
where
    (TValue, <Self as IRange>::ValueType): ITyEq,
    TValue: SaturatingSub, {
    fn is_empty(&self) -> bool;
    fn len(&self) -> Option<usize>;
}
