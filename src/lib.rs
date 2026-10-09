#![doc = include_str!("../README.md")]

use sz3_sys::SZ3_Config;

#[derive(Clone, Debug, Copy)]
#[non_exhaustive]
pub enum CompressionAlgorithm {
    Interpolation,
    InterpolationLorenzo,
    LorenzoRegression {
        lorenzo: bool,
        lorenzo_second_order: bool,
        regression: bool,
    },
    BiologyMolecularData,
    NoPrediction,
    Lossless,
}

impl CompressionAlgorithm {
    fn decode(config: sz3_sys::SZ3_Config) -> Self {
        match config.cmprAlgo as _ {
            sz3_sys::SZ3::ALGO_ALGO_INTERP => Self::Interpolation,
            sz3_sys::SZ3::ALGO_ALGO_INTERP_LORENZO => Self::InterpolationLorenzo,
            sz3_sys::SZ3::ALGO_ALGO_LORENZO_REG => Self::LorenzoRegression {
                lorenzo: config.lorenzo,
                lorenzo_second_order: config.lorenzo2,
                regression: config.regression,
            },
            sz3_sys::SZ3::ALGO_ALGO_BIOMD => Self::BiologyMolecularData,
            sz3_sys::SZ3::ALGO_ALGO_NOPRED => Self::NoPrediction,
            sz3_sys::SZ3::ALGO_ALGO_LOSSLESS => Self::Lossless,
            algo => panic!("unsupported compression algorithm {}", algo),
        }
    }

    fn code(&self) -> u8 {
        (match self {
            Self::Interpolation { .. } => sz3_sys::SZ3::ALGO_ALGO_INTERP,
            Self::InterpolationLorenzo { .. } => sz3_sys::SZ3::ALGO_ALGO_INTERP_LORENZO,
            Self::LorenzoRegression { .. } => sz3_sys::SZ3::ALGO_ALGO_LORENZO_REG,
            Self::BiologyMolecularData => sz3_sys::SZ3::ALGO_ALGO_BIOMD,
            Self::NoPrediction => sz3_sys::SZ3::ALGO_ALGO_NOPRED,
            Self::Lossless => sz3_sys::SZ3::ALGO_ALGO_LOSSLESS,
        }) as _
    }

    fn lorenzo(&self) -> bool {
        match self {
            Self::LorenzoRegression { lorenzo, .. } => *lorenzo,
            _ => true,
        }
    }

    fn lorenzo_second_order(&self) -> bool {
        match self {
            Self::LorenzoRegression {
                lorenzo_second_order,
                ..
            } => *lorenzo_second_order,
            _ => true,
        }
    }

    fn regression(&self) -> bool {
        match self {
            Self::LorenzoRegression { regression, .. } => *regression,
            _ => true,
        }
    }

    pub fn interpolation() -> Self {
        Self::Interpolation
    }

    pub fn interpolation_lorenzo() -> Self {
        Self::InterpolationLorenzo
    }

    pub fn lorenzo_regression() -> Self {
        Self::LorenzoRegression {
            lorenzo: true,
            lorenzo_second_order: false,
            regression: true,
        }
    }

    pub fn lorenzo_regression_custom(
        lorenzo: Option<bool>,
        lorenzo_second_order: Option<bool>,
        regression: Option<bool>,
    ) -> Self {
        Self::LorenzoRegression {
            lorenzo: lorenzo.unwrap_or(true),
            lorenzo_second_order: lorenzo_second_order.unwrap_or(false),
            regression: regression.unwrap_or(true),
        }
    }

    pub fn biology_molecular_data() -> Self {
        Self::BiologyMolecularData
    }

    pub fn no_prediction() -> Self {
        Self::NoPrediction
    }

    pub fn lossless() -> Self {
        Self::Lossless
    }
}

impl Default for CompressionAlgorithm {
    fn default() -> Self {
        Self::interpolation_lorenzo()
    }
}

#[derive(Clone, Debug, Copy)]
#[non_exhaustive]
pub enum ErrorBound {
    Absolute(f64),
    Relative(f64),
    PSNR(f64),
    L2Norm(f64),
    AbsoluteAndRelative {
        absolute_bound: f64,
        relative_bound: f64,
    },
    AbsoluteOrRelative {
        absolute_bound: f64,
        relative_bound: f64,
    },
}

impl ErrorBound {
    fn decode(config: sz3_sys::SZ3_Config) -> Self {
        match config.errorBoundMode as _ {
            sz3_sys::SZ3::EB_EB_ABS => Self::Absolute(config.absErrorBound),
            sz3_sys::SZ3::EB_EB_REL => Self::Relative(config.relErrorBound),
            sz3_sys::SZ3::EB_EB_PSNR => Self::PSNR(config.psnrErrorBound),
            sz3_sys::SZ3::EB_EB_L2NORM => Self::L2Norm(config.l2normErrorBound),
            sz3_sys::SZ3::EB_EB_ABS_OR_REL => Self::AbsoluteOrRelative {
                absolute_bound: config.absErrorBound,
                relative_bound: config.relErrorBound,
            },
            sz3_sys::SZ3::EB_EB_ABS_AND_REL => Self::AbsoluteAndRelative {
                absolute_bound: config.absErrorBound,
                relative_bound: config.relErrorBound,
            },
            mode => panic!("unsupported error bound {}", mode),
        }
    }

    fn code(&self) -> u8 {
        (match self {
            Self::Absolute(_) => sz3_sys::SZ3::EB_EB_ABS,
            Self::Relative(_) => sz3_sys::SZ3::EB_EB_REL,
            Self::PSNR(_) => sz3_sys::SZ3::EB_EB_PSNR,
            Self::L2Norm(_) => sz3_sys::SZ3::EB_EB_L2NORM,
            Self::AbsoluteAndRelative { .. } => sz3_sys::SZ3::EB_EB_ABS_AND_REL,
            Self::AbsoluteOrRelative { .. } => sz3_sys::SZ3::EB_EB_ABS_OR_REL,
        }) as _
    }

    fn abs_bound(&self) -> f64 {
        match self {
            Self::Absolute(bound) => *bound,
            Self::AbsoluteOrRelative { absolute_bound, .. } => *absolute_bound,
            Self::AbsoluteAndRelative { absolute_bound, .. } => *absolute_bound,
            _ => 0.0,
        }
    }

    fn rel_bound(&self) -> f64 {
        match self {
            Self::Relative(bound) => *bound,
            Self::AbsoluteOrRelative { relative_bound, .. } => *relative_bound,
            Self::AbsoluteAndRelative { relative_bound, .. } => *relative_bound,
            _ => 0.0,
        }
    }

    fn l2norm_bound(&self) -> f64 {
        match self {
            Self::L2Norm(bound) => *bound,
            _ => 0.0,
        }
    }

    fn psnr_bound(&self) -> f64 {
        match self {
            Self::PSNR(bound) => *bound,
            _ => 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    compression_algorithm: CompressionAlgorithm,
    error_bound: ErrorBound,
    openmp: bool,
    quantization_bincount: u32,
    block_size: Option<u32>,
}

pub trait SZ3Compressible: private::Sealed + Sized {}
impl SZ3Compressible for f32 {}
impl SZ3Compressible for f64 {}
impl SZ3Compressible for u8 {}
impl SZ3Compressible for i8 {}
impl SZ3Compressible for u16 {}
impl SZ3Compressible for i16 {}
impl SZ3Compressible for u32 {}
impl SZ3Compressible for i32 {}
impl SZ3Compressible for u64 {}
impl SZ3Compressible for i64 {}

mod private {
    pub trait Sealed: Copy {
        const SZ_DATA_TYPE: u8;

        unsafe fn compress_size_bound(
            config: sz3_sys::SZ3_Config,
            bound: *mut usize,
            message: *mut *mut std::ffi::c_char,
        ) -> sz3_sys::SZ3_Status;

        unsafe fn compress(
            config: sz3_sys::SZ3_Config,
            data: *const Self,
            compressed_data: *mut u8,
            compressed_capacity: usize,
            compressed_len: *mut usize,
            message: *mut *mut std::ffi::c_char,
        ) -> sz3_sys::SZ3_Status;

        unsafe fn decompress(
            compressed_data: *const u8,
            compressed_len: usize,
            decompressed_data: *mut Self,
            message: *mut *mut std::ffi::c_char,
        ) -> sz3_sys::SZ3_Status;
    }

    macro_rules! impl_sealed {
        ($($impl:ident),*) => {
            $(impl Sealed for sz3_sys::$impl::ty {
                const SZ_DATA_TYPE: u8 = sz3_sys::$impl::DATA_TYPE_TYPE;

                unsafe fn compress_size_bound(
                    config: sz3_sys::SZ3_Config,
                    bound: *mut usize,
                    message: *mut *mut std::ffi::c_char,
                ) -> sz3_sys::SZ3_Status {
                    sz3_sys::$impl::compress_size_bound(config, bound, message)
                }

                unsafe fn compress(
                    config: sz3_sys::SZ3_Config,
                    data: *const Self,
                    compressed_data: *mut u8,
                    compressed_capacity: usize,
                    compressed_len: *mut usize,
                    message: *mut *mut std::ffi::c_char,
                ) -> sz3_sys::SZ3_Status {
                    sz3_sys::$impl::compress(
                        config,
                        data,
                        compressed_data.cast(),
                        compressed_capacity,
                        compressed_len,
                        message,
                    )
                }

                unsafe fn decompress(
                    compressed_data: *const u8,
                    compressed_len: usize,
                    decompressed_data: *mut Self,
                    message: *mut *mut std::ffi::c_char,
                ) -> sz3_sys::SZ3_Status {
                    sz3_sys::$impl::decompress(compressed_data.cast(), compressed_len, decompressed_data, message)
                }
            })*
        }
    }

    impl_sealed!(
        impl_u8, impl_i8, impl_u16, impl_i16, impl_u32, impl_i32, impl_u64, impl_i64, impl_f32,
        impl_f64
    );
}

#[derive(Clone, Debug)]
pub struct DimensionedData<V: SZ3Compressible, T: std::ops::Deref<Target = [V]>> {
    data: T,
    dims: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct DimensionedDataBuilder<'a, V> {
    data: &'a [V],
    dims: Vec<usize>,
    remainder: usize,
}

impl<V: SZ3Compressible, T: std::ops::Deref<Target = [V]>> DimensionedData<V, T> {
    pub fn build<'a>(data: &'a T) -> DimensionedDataBuilder<'a, V> {
        DimensionedDataBuilder {
            data,
            dims: vec![],
            remainder: data.len(),
        }
    }

    pub fn data(&self) -> &[V] {
        &self.data
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_ptr(&self) -> *const V {
        self.data.as_ptr()
    }
}

#[derive(Debug)]
pub struct DimensionedDataBuilderMut<'a, V> {
    data: &'a mut [V],
    dims: Vec<usize>,
    remainder: usize,
}

impl<V: SZ3Compressible, T: std::ops::DerefMut<Target = [V]>> DimensionedData<V, T> {
    pub fn build_mut<'a>(data: &'a mut T) -> DimensionedDataBuilderMut<'a, V> {
        DimensionedDataBuilderMut {
            remainder: data.len(),
            data,
            dims: vec![],
        }
    }

    pub fn data_mut(&mut self) -> &mut [V] {
        &mut self.data
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SZ3Error {
    #[error(
        "invalid dimension specification for data of length {len}: already specified dimensions \
         {dims:?}, and wanted to add dimension with length {wanted}, but this does not divide \
         {remainder} cleanly"
    )]
    InvalidDimensionSize {
        dims: Vec<usize>,
        len: usize,
        wanted: usize,
        remainder: usize,
    },
    #[error("dimension with size one has no use")]
    OneSizedDimension,
    #[error(
        "dimension specification {dims:?} for data of length {len} does not cover whole space, \
         missing a dimension of {remainder}"
    )]
    UnderSpecifiedDimensions {
        dims: Vec<usize>,
        len: usize,
        remainder: usize,
    },
    #[error("cannot decompress to array with a different data type")]
    DecompressedDataTypeMismatch,
    #[error("cannot decompress array with dimensions {found:?} to array with different dimensions {expected:?}")]
    DecompressedDimsMismatch {
        found: Vec<usize>,
        expected: Vec<usize>,
    },
    #[error(
        "BiologyMolecularData compresses coordinates of shape {{atoms, 3}} or {{frames, atoms, 3}}, \
         not dimensions {dims:?}"
    )]
    BiologyMolecularDataShape { dims: Vec<usize> },
    #[error("SZ3 reported an error ({kind:?}): {message}")]
    InternalError {
        kind: InternalErrorKind,
        message: String,
    },
}

/// Kinds of internal SZ3 errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InternalErrorKind {
    /// Unsupported configuration, data type or dimensions
    InvalidArgument,
    /// Truncated or corrupt compressed data
    OutOfRange,
    Runtime,
    OutOfMemory,
    Other,
}

type Result<T> = std::result::Result<T, SZ3Error>;

/// Convert the SZ3 return status and error message into a `Result`
///
/// The wrapper sets the message only when it returns an error, so callers pass it in null, and on
/// `SZ3_OK` it is still null. On an error this takes ownership of the message and frees it.
fn check_sz3_status(kind: sz3_sys::SZ3_Status, message: *mut std::ffi::c_char) -> Result<()> {
    let kind = match kind {
        sz3_sys::SZ3_Status_SZ3_OK => return Ok(()),
        sz3_sys::SZ3_Status_SZ3_INVALID_ARGUMENT => InternalErrorKind::InvalidArgument,
        sz3_sys::SZ3_Status_SZ3_OUT_OF_RANGE => InternalErrorKind::OutOfRange,
        sz3_sys::SZ3_Status_SZ3_RUNTIME => InternalErrorKind::Runtime,
        sz3_sys::SZ3_Status_SZ3_OUT_OF_MEMORY => InternalErrorKind::OutOfMemory,
        _ => InternalErrorKind::Other,
    };
    // the wrapper leaves the message null if it could not allocate it
    let message = if message.is_null() {
        String::new()
    } else {
        let copy = unsafe { std::ffi::CStr::from_ptr(message) }
            .to_string_lossy()
            .into_owned();
        unsafe { sz3_sys::free_error_message(message) };
        copy
    };
    Err(SZ3Error::InternalError { kind, message })
}

macro_rules! impl_dimensioned_data_builder {
    ($($builder:ident => $data:ty),*) => {
        $(impl<'a, V: SZ3Compressible> $builder<'a, V> {
            pub fn dim(mut self, length: usize) -> Result<Self> {
                if length == 1 {
                    if self.dims.is_empty() && self.remainder == 1 {
                        self.dims.push(1);
                        Ok(self)
                    } else {
                        Err(SZ3Error::OneSizedDimension)
                    }
                } else if self.remainder.rem_euclid(length) != 0 {
                    Err(SZ3Error::InvalidDimensionSize {
                        dims: self.dims,
                        len: self.data.len(),
                        wanted: length,
                        remainder: self.remainder,
                    })
                } else {
                    self.dims.push(length as _);
                    self.remainder /= length;
                    Ok(self)
                }
            }

            pub fn remainder_dim(self) -> Result<$data> {
                let remainder = self.remainder;
                self.dim(remainder)?.finish()
            }

            pub fn finish(self) -> Result<$data> {
                if self.remainder != 1 {
                    Err(SZ3Error::UnderSpecifiedDimensions {
                        dims: self.dims,
                        len: self.data.len(),
                        remainder: self.remainder,
                    })
                } else {
                    Ok(DimensionedData {
                        data: self.data,
                        dims: self.dims,
                    })
                }
            }
        })*
    };
}

impl_dimensioned_data_builder! {
    DimensionedDataBuilder => DimensionedData<V, &'a [V]>,
    DimensionedDataBuilderMut => DimensionedData<V, &'a mut  [V]>
}

impl Config {
    fn from_decompressed(config: sz3_sys::SZ3_Config) -> Self {
        Self {
            compression_algorithm: CompressionAlgorithm::decode(config),
            error_bound: ErrorBound::decode(config),
            openmp: config.openmp,
            quantization_bincount: config.quantbinCnt as _,
            block_size: Some(config.blockSize as _),
        }
    }
}

impl Config {
    pub fn new(error_bound: ErrorBound) -> Self {
        Self {
            compression_algorithm: CompressionAlgorithm::default(),
            error_bound,
            openmp: false,
            quantization_bincount: 65536,
            block_size: None,
        }
    }

    pub fn compression_algorithm(mut self, compression_algorithm: CompressionAlgorithm) -> Self {
        self.compression_algorithm = compression_algorithm;
        self
    }

    pub fn error_bound(mut self, error_bound: ErrorBound) -> Self {
        self.error_bound = error_bound;
        self
    }

    #[cfg(feature = "openmp")]
    pub fn openmp(mut self, openmp: bool) -> Self {
        self.openmp = openmp;
        self
    }

    pub fn quantization_bincount(mut self, quantization_bincount: u32) -> Self {
        self.quantization_bincount = quantization_bincount;
        self
    }

    pub fn block_size(mut self, block_size: u32) -> Self {
        self.block_size = Some(block_size);
        self
    }

    pub fn automatic_block_size(mut self) -> Self {
        self.block_size = None;
        self
    }
}

pub fn compress<V: SZ3Compressible, T: std::ops::Deref<Target = [V]>>(
    data: &DimensionedData<V, T>,
    error_bound: ErrorBound,
) -> Result<Vec<u8>> {
    let config = Config::new(error_bound);
    compress_with_config(data, &config)
}

pub fn compress_with_config<V: SZ3Compressible, T: std::ops::Deref<Target = [V]>>(
    data: &DimensionedData<V, T>,
    config: &Config,
) -> Result<Vec<u8>> {
    let mut compressed_data = Vec::new();
    compress_into_with_config(data, config, &mut compressed_data)?;
    Ok(compressed_data)
}

pub fn compress_into<V: SZ3Compressible, T: std::ops::Deref<Target = [V]>>(
    data: &DimensionedData<V, T>,
    error_bound: ErrorBound,
    compressed_data: &mut Vec<u8>,
) -> Result<()> {
    let config = Config::new(error_bound);
    compress_into_with_config(data, &config, compressed_data)
}

pub fn compress_into_with_config<V: SZ3Compressible, T: std::ops::Deref<Target = [V]>>(
    data: &DimensionedData<V, T>,
    config: &Config,
    compressed_data: &mut Vec<u8>,
) -> Result<()> {
    if matches!(
        config.compression_algorithm,
        CompressionAlgorithm::BiologyMolecularData
    ) && (data.dims().len() > 3 || data.dims().last() != Some(&3))
    {
        return Err(SZ3Error::BiologyMolecularDataShape {
            dims: data.dims().to_vec(),
        });
    }

    let block_size = config.block_size.unwrap_or(match data.dims().len() {
        1 => 128,
        2 => 16,
        _ => 6,
    });

    let raw_config = sz3_sys::SZ3_Config {
        N: data.dims().len() as _,
        dims: data.dims.as_ptr() as _,
        num: data.len() as _,
        errorBoundMode: config.error_bound.code(),
        absErrorBound: config.error_bound.abs_bound(),
        relErrorBound: config.error_bound.rel_bound(),
        l2normErrorBound: config.error_bound.l2norm_bound(),
        psnrErrorBound: config.error_bound.psnr_bound(),
        cmprAlgo: config.compression_algorithm.code(),
        lorenzo: config.compression_algorithm.lorenzo(),
        lorenzo2: config.compression_algorithm.lorenzo_second_order(),
        regression: config.compression_algorithm.regression(),
        openmp: config.openmp,
        dataType: V::SZ_DATA_TYPE as _,
        blockSize: block_size as _,
        quantbinCnt: config.quantization_bincount as _,
    };

    let mut message = std::ptr::null_mut();

    let mut capacity: usize = 0;
    check_sz3_status(
        unsafe { V::compress_size_bound(raw_config, &mut capacity, &mut message) },
        message,
    )?;
    compressed_data.reserve(capacity);

    let mut len: usize = 0;
    check_sz3_status(
        unsafe {
            V::compress(
                raw_config,
                data.as_ptr(),
                compressed_data
                    .spare_capacity_mut()
                    .as_mut_ptr()
                    .cast::<u8>(),
                capacity,
                &mut len,
                &mut message,
            )
        },
        message,
    )?;
    unsafe { compressed_data.set_len(compressed_data.len() + len) };

    Ok(())
}

pub fn decompress<V: SZ3Compressible, T: std::ops::Deref<Target = [u8]>>(
    compressed_data: T,
) -> Result<(Config, DimensionedData<V, Vec<V>>)> {
    let DecompressedConfig {
        config,
        len,
        dims,
        data_type,
    } = DecompressedConfig::from_compressed(&compressed_data)?;

    if data_type != (V::SZ_DATA_TYPE as _) {
        return Err(SZ3Error::DecompressedDataTypeMismatch);
    }

    let decompressed_data = unsafe {
        let mut decompressed_data = Vec::with_capacity(len);

        // safety: decompressed data is uninitialized and valid for 0..len
        let mut message = std::ptr::null_mut();
        check_sz3_status(
            V::decompress(
                compressed_data.as_ptr(),
                compressed_data.len(),
                decompressed_data
                    .spare_capacity_mut()
                    .as_mut_ptr()
                    .cast::<V>(),
                &mut message,
            ),
            message,
        )?;

        // safety: decompressed data is initialized for 0..len
        decompressed_data.set_len(len);
        decompressed_data
    };

    Ok((
        config,
        DimensionedData {
            data: decompressed_data,
            dims,
        },
    ))
}

pub fn decompress_into_dimensioned<
    V: SZ3Compressible,
    C: std::ops::Deref<Target = [u8]>,
    D: std::ops::DerefMut<Target = [V]>,
>(
    compressed_data: C,
    decompressed_data: &mut DimensionedData<V, D>,
) -> Result<Config> {
    let DecompressedConfig {
        config,
        len,
        dims,
        data_type,
    } = DecompressedConfig::from_compressed(&compressed_data)?;

    if data_type != (V::SZ_DATA_TYPE as _) {
        return Err(SZ3Error::DecompressedDataTypeMismatch);
    }

    if decompressed_data.dims() != dims.as_slice() {
        return Err(SZ3Error::DecompressedDimsMismatch {
            found: dims,
            expected: decompressed_data.dims.clone(),
        });
    }

    // must succeed unless SZ3 decompression was corrupted
    assert_eq!(decompressed_data.len(), len);

    // safety: decompressed data is initialized for 0..len
    //         *and* V: Copy, so we can just override the elements
    let mut message = std::ptr::null_mut();
    check_sz3_status(
        unsafe {
            V::decompress(
                compressed_data.as_ptr(),
                compressed_data.len(),
                decompressed_data.data.as_mut_ptr(),
                &mut message,
            )
        },
        message,
    )?;

    Ok(config)
}

struct DecompressedConfig {
    config: Config,
    len: usize,
    dims: Vec<usize>,
    data_type: u8,
}

impl DecompressedConfig {
    fn from_compressed(compressed_data: &[u8]) -> Result<Self> {
        let mut config = std::mem::MaybeUninit::<SZ3_Config>::uninit();
        let mut message = std::ptr::null_mut();
        check_sz3_status(
            unsafe {
                sz3_sys::decompress_config(
                    compressed_data.as_ptr().cast(),
                    compressed_data.len(),
                    config.as_mut_ptr(),
                    &mut message,
                )
            },
            message,
        )?;
        // safety: decompress_config initialized the config when it returned SZ3_OK
        let config = unsafe { config.assume_init() };
        let dims = (0..config.N)
            .map(|i| unsafe { std::ptr::read(config.dims.add(i as _)) })
            .collect();
        unsafe {
            sz3_sys::dealloc_size_t(config.dims);
        }
        let SZ3_Config {
            num: len,
            dataType: data_type,
            ..
        } = config;
        let config = Config::from_decompressed(config);
        Ok(Self {
            config,
            len,
            dims,
            data_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: [f32; 8847360] = unsafe {
        align_data::include_transmute!("../test_data/darkframe_1.0x_10.872624ms_1024n.blob")
    };

    fn test_data<T: SZ3Compressible + From<f32>>() -> Vec<T> {
        bytemuck::cast_slice(&TEST_DATA[..])
            .iter()
            .map(|v| T::from(*v))
            .take(64 * 64 * 64)
            .collect()
    }

    fn check_error_bound<T: SZ3Compressible + Copy + 'static>(
        data: &DimensionedData<T, &[T]>,
        config: &Config,
        error_bound: ErrorBound,
    ) -> Result<()>
    where
        f64: From<T>,
    {
        let config = config.clone().error_bound(error_bound);
        let (_decompressed_config, decompressed_data) =
            decompress::<T, _>(&*compress_with_config(data, &config)?)?;
        let min = data
            .data()
            .iter()
            .copied()
            .map(f64::from)
            .fold(f64::MAX, f64::min);
        let max = data
            .data()
            .iter()
            .copied()
            .map(f64::from)
            .fold(f64::MIN, f64::max);
        let range = max - min;

        match error_bound {
            ErrorBound::Absolute(absolute_bound) => {
                for (orig, compressed) in data.data().iter().zip(decompressed_data.data()) {
                    assert!((f64::from(*orig) - f64::from(*compressed)).abs() <= absolute_bound)
                }
            }
            ErrorBound::Relative(relative_bound) => {
                for (orig, compressed) in data.data().iter().zip(decompressed_data.data()) {
                    let orig = f64::from(*orig);
                    let compressed = f64::from(*compressed);
                    assert!((orig - compressed).abs() / range < relative_bound)
                }
            }
            ErrorBound::PSNR(psnr_bound) => {
                let mse: f64 = data
                    .data()
                    .iter()
                    .zip(decompressed_data.data())
                    .map(|(a, b)| {
                        let diff = f64::from(*a) - f64::from(*b);
                        diff * diff
                    })
                    .sum();
                let psnr = 20. * (max - min).log10() - 10. * mse.log10();
                // PSNR for zero error is infinity but meets the bound
                assert!(mse == 0.0 || psnr < psnr_bound);
            }
            ErrorBound::L2Norm(l2norm_bound) => {
                let mse: f64 = data
                    .data()
                    .iter()
                    .zip(decompressed_data.data())
                    .map(|(a, b)| {
                        let diff = f64::from(*a) - f64::from(*b);
                        diff * diff
                    })
                    .sum();
                let l2norm = mse.sqrt();
                // random fudge factor because its not always really good at matching
                assert!(l2norm < l2norm_bound * 1.005);
            }
            ErrorBound::AbsoluteAndRelative {
                absolute_bound,
                relative_bound,
            } => {
                for (orig, compressed) in data.data().iter().zip(decompressed_data.data()) {
                    let orig = f64::from(*orig);
                    let compressed = f64::from(*compressed);
                    let abs = (orig - compressed).abs();
                    let rel = abs / range;
                    assert!((rel < relative_bound) && (abs < absolute_bound));
                }
            }
            ErrorBound::AbsoluteOrRelative {
                absolute_bound,
                relative_bound,
            } => {
                for (orig, compressed) in data.data().iter().zip(decompressed_data.data()) {
                    let orig = f64::from(*orig);
                    let compressed = f64::from(*compressed);
                    let abs = (orig - compressed).abs();
                    let rel = abs / range;
                    assert!((rel < relative_bound) || (abs < absolute_bound));
                }
            }
        }

        if matches!(config.compression_algorithm, CompressionAlgorithm::Lossless) {
            for (orig, compressed) in data.data().iter().zip(decompressed_data.data()) {
                assert_eq!(f64::from(*orig).to_bits(), f64::from(*compressed).to_bits());
            }
        }

        Ok(())
    }

    macro_rules! foreach_combination {
        (([$($values:tt),*], $the_rest:tt); $to_call:ident, $accum:tt) => {
            $(
                foreach_combination!(@concat $the_rest; $to_call, $accum, $values);
            )*
        };

        (([$($values:tt),*]); $to_call:ident, $accum:tt) => {
            $(
                foreach_combination!(@concat_call $to_call $accum, $values);
            )*
        };

        (@concat $the_rest:tt; $to_call:ident, ($($accum:tt),*), $value:tt) => {
            foreach_combination!($the_rest; $to_call, ($($accum,)* $value));
        };

        (@concat_call $to_call:ident ($($accum:tt),*), $value:tt) => {
            $to_call!($($accum,)* $value);
        };
    }

    macro_rules! gen_test {
        (($openmp:expr, $openmp_cfg:meta), ($eb_name:ident, $eb:expr), ($ca_name: ident, $ca:expr), $qb:expr, $block_size:expr) => {
            paste::paste! {
                #[$openmp_cfg]
                #[test]
                fn [<test_ $openmp _ $eb_name _ $ca_name _ $qb _ $block_size>]() -> Result<()> {
                    let data = test_data::<f32>();
                    // ALGO_BIOMD takes coordinates, {frames, atoms, 3}
                    let biomd = matches!($ca, CompressionAlgorithm::BiologyMolecularData);
                    let data = if biomd { data[..(64 * 192 * 3)].to_vec() } else { data };
                    let data = if biomd {
                        DimensionedData::build(&data)
                            .dim(64)?
                            .dim(192)?
                            .remainder_dim()?
                    } else {
                        DimensionedData::build(&data)
                            .dim(64)?
                            .dim(64)?
                            .remainder_dim()?
                    };
                    let config = Config::new($eb)
                        .error_bound($eb)
                        .compression_algorithm($ca)
                        .quantization_bincount($qb)
                        .block_size($block_size);
                    #[cfg(feature = "openmp")]
                    let config = config.openmp($openmp);

                    check_error_bound(&data, &config, $eb)?;

                    Ok(())
                }
            }
        };
    }

    foreach_combination!(
        ([(true, cfg(feature = "openmp")), (false, cfg(all()))],
        ([
            (absolute_0, ErrorBound::Absolute(0.)),
            (absolute_1_0, ErrorBound::Absolute(1.)),
            (absolute_0_1, ErrorBound::Absolute(0.1)),
            (absolute_0_01, ErrorBound::Absolute(0.01)),
            (relative_0_01, ErrorBound::Relative(0.01)),
            (relative_0_001, ErrorBound::Relative(0.001)),
            (relative_0_0001, ErrorBound::Relative(0.0001)),
            (psnr_120, ErrorBound::PSNR(120.0)),
            (psnr_100, ErrorBound::PSNR(100.0)),
            (psnr_80, ErrorBound::PSNR(80.0)),
            (l2norm_40, ErrorBound::L2Norm(40.0)),
            (l2norm_30, ErrorBound::L2Norm(30.0)),
            (l2norm_20, ErrorBound::L2Norm(20.0)),
            (abs_and_rel_0_1_0_001, ErrorBound::AbsoluteAndRelative {
                absolute_bound: 0.1,
                relative_bound: 0.001,
            }),
            (abs_or_rel_0_1_0_001, ErrorBound::AbsoluteOrRelative {
                absolute_bound: 0.1,
                relative_bound: 0.001,
            })
        ],
        ([
            (interpolation, CompressionAlgorithm::Interpolation),
            (interpolation_lorenzo, CompressionAlgorithm::InterpolationLorenzo),
            (lorenzo_reg, CompressionAlgorithm::lorenzo_regression()),
            (lorenzo_reg_r, CompressionAlgorithm::lorenzo_regression_custom(
                Some(false),
                Some(false),
                Some(true),
            )),
            (lorenzo_reg_l2, CompressionAlgorithm::lorenzo_regression_custom(
                Some(false),
                Some(true),
                Some(false),
            )),
            (lorenzo_reg_l2r, CompressionAlgorithm::lorenzo_regression_custom(
                Some(false),
                Some(false),
                Some(true),
            )),
            (lorenzo_reg_l, CompressionAlgorithm::lorenzo_regression_custom(
                Some(true),
                Some(false),
                Some(false),
            )),
            (lorenzo_reg_lr, CompressionAlgorithm::lorenzo_regression_custom(
                Some(true),
                Some(false),
                Some(true),
            )),
            (lorenzo_reg_ll2, CompressionAlgorithm::lorenzo_regression_custom(
                Some(true),
                Some(true),
                Some(false),
            )),
            (lorenzo_reg_ll2r, CompressionAlgorithm::lorenzo_regression_custom(
                Some(true),
                Some(true),
                Some(true),
            )),
            (biomd, CompressionAlgorithm::BiologyMolecularData),
            (no_prediction, CompressionAlgorithm::NoPrediction),
            (lossless, CompressionAlgorithm::Lossless)
        ],
        ([65536, 256, 2097152],
        ([2, 4, 8, 16])))));
        gen_test, ());

    fn compressed_test_data() -> Result<Vec<u8>> {
        let data = test_data::<f32>();
        let data = DimensionedData::build(&data)
            .dim(64)?
            .dim(64)?
            .remainder_dim()?;
        compress(&data, ErrorBound::Absolute(0.1))
    }

    fn internal_error_kind<T: std::fmt::Debug>(result: Result<T>) -> Option<InternalErrorKind> {
        match result {
            Err(SZ3Error::InternalError { kind, message }) => {
                assert!(!message.is_empty());
                Some(kind)
            }
            other => panic!("expected an InternalError, got {other:?}"),
        }
    }

    #[test]
    fn decompress_refuses_data_shorter_than_the_header() -> Result<()> {
        let compressed = compressed_test_data()?;
        assert_eq!(
            internal_error_kind(decompress::<f32, _>(&compressed[..10])),
            Some(InternalErrorKind::OutOfRange)
        );
        Ok(())
    }

    #[test]
    fn decompress_refuses_a_payload_past_the_end() -> Result<()> {
        let mut compressed = compressed_test_data()?;
        // the payload length, after the magic and the version
        compressed[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
        assert_eq!(
            internal_error_kind(decompress::<f32, _>(&compressed[..])),
            Some(InternalErrorKind::OutOfRange)
        );
        Ok(())
    }

    #[test]
    fn decompress_refuses_a_truncated_config() -> Result<()> {
        let compressed = compressed_test_data()?;
        assert_eq!(
            internal_error_kind(decompress::<f32, _>(&compressed[..compressed.len() - 4])),
            Some(InternalErrorKind::OutOfRange)
        );
        Ok(())
    }

    #[test]
    fn biomd_checks_the_shape() -> Result<()> {
        let data = test_data::<f32>();
        let data = DimensionedData::build(&data)
            .dim(64)?
            .dim(64)?
            .remainder_dim()?;
        let config = Config::new(ErrorBound::Absolute(0.1))
            .compression_algorithm(CompressionAlgorithm::BiologyMolecularData);
        assert!(matches!(
            compress_with_config(&data, &config),
            Err(SZ3Error::BiologyMolecularDataShape { .. })
        ));
        Ok(())
    }
}
