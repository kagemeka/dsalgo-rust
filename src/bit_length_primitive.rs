use crate::count_leading_zeros::CountLeadingZeros;

pub fn bit_length<T: CountLeadingZeros<Output = usize>>(x: &T) -> usize {
    std::mem::size_of::<T>() * 8 - x.clz()
}
