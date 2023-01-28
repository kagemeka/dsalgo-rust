//! by using fft for large numbers, errors occur.
//! so split each integer as form of a2*base^2 + a_1*base + a_0,
//! convolve 5 times, and roll up at last
