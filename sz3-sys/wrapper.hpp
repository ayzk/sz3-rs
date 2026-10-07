#include <cstdint>

#include "SZ3/api/sz.hpp"

struct SZ3_Config {
    char N;
    size_t * dims;
    size_t num;
    uint8_t cmprAlgo;
    uint8_t errorBoundMode;
    double absErrorBound;
    double relErrorBound;
    double psnrErrorBound;
    double l2normErrorBound;
    bool lorenzo;
    bool lorenzo2;
    bool regression;
    bool openmp;
    uint8_t dataType;
    int quantbinCnt;
    int blockSize;

    SZ3::Config into() {
        auto conf = SZ3::Config{};
        conf.N = N;
        conf.dims = std::vector<size_t>(dims, dims + N);
        conf.num = num;
        conf.cmprAlgo = cmprAlgo;
        conf.errorBoundMode = errorBoundMode;
        conf.absErrorBound = absErrorBound;
        conf.relErrorBound = relErrorBound;
        conf.psnrErrorBound = psnrErrorBound;
        conf.l2normErrorBound = l2normErrorBound;
        conf.lorenzo = lorenzo;
        conf.lorenzo2 = lorenzo2;
        conf.regression = regression;
        conf.openmp = openmp;
        conf.dataType = dataType;
        conf.quantbinCnt = quantbinCnt;
        conf.blockSize = blockSize;
        return conf;
    }

    SZ3_Config(SZ3::Config &conf) {
        dims = new size_t[conf.N];
        std::copy(conf.dims.begin(), conf.dims.end(), dims);
        N = conf.N;
        num = conf.num;
        cmprAlgo = conf.cmprAlgo;
        errorBoundMode = conf.errorBoundMode;
        absErrorBound = conf.absErrorBound;
        relErrorBound = conf.relErrorBound;
        psnrErrorBound = conf.psnrErrorBound;
        l2normErrorBound = conf.l2normErrorBound;
        lorenzo = conf.lorenzo;
        lorenzo2 = conf.lorenzo2;
        regression = conf.regression;
        openmp = conf.openmp;
        dataType = conf.dataType;
        quantbinCnt = conf.quantbinCnt;
        blockSize = conf.blockSize;
    }
};


#define func(ns, type, dt) \
  namespace ns { \
    using ty = type; \
	enum DATA_TYPE : uint8_t { \
      TYPE = dt \
    }; \
    /* An exception must not cross into Rust: 0 and false report one instead. */ \
    size_t compress_size_bound(SZ3_Config config) { \
        try { \
            return SZ3::SZ_compress_size_bound<ty>(config.into()); \
        } catch (...) { \
            return 0; \
        } \
    } \
    size_t compress(SZ3_Config config, const ty * data, char * compressedData, size_t compressedCapacity) { \
        try { \
            return SZ_compress<ty>(config.into(), data, compressedData, compressedCapacity); \
        } catch (...) { \
            return 0; \
        } \
    } \
    bool decompress(const char * compressedData, size_t compressedSize, ty * decompressedData) { \
        try { \
            auto conf = SZ3::Config{}; \
            SZ_decompress<ty>(conf, compressedData, compressedSize, decompressedData); \
            return true; \
        } catch (...) { \
            return false; \
        } \
    } \
  }


func(impl_f32, float, SZ_FLOAT)
func(impl_f64, double, SZ_DOUBLE)
func(impl_u8, uint8_t, SZ_UINT8)
func(impl_i8, int8_t, SZ_INT8)
func(impl_u16, uint16_t, SZ_UINT16)
func(impl_i16, int16_t, SZ_INT16)
func(impl_u32, uint32_t, SZ_UINT32)
func(impl_i32, int32_t, SZ_INT32)
func(impl_u64, uint64_t, SZ_UINT64)
func(impl_i64, int64_t, SZ_INT64)

// magic(4) + version(4) + payload length(8), little endian.
static constexpr size_t SZ3_HEADER_LEN = sizeof(uint32_t) + sizeof(uint32_t) + sizeof(uint64_t);

// Produce a configuration that does not describe a valid SZ3 stream, without dimensions, elements, and an invalid data type
static SZ3_Config unreadable_config() {
    auto conf = SZ3::Config{};
    auto out = SZ3_Config(conf);
    out.N = 0;
    out.num = 0;
    out.dataType = UINT8_MAX;
    return out;
}

SZ3_Config decompress_config(const char * compressedData, size_t compressedSize) {
    if (compressedSize < SZ3_HEADER_LEN) {
        return unreadable_config();
    }
    auto cmpDataPos = reinterpret_cast<const SZ3::uchar *>(compressedData);
    uint32_t magic;
    SZ3::read(magic, cmpDataPos);
    uint32_t ver;
    SZ3::read(ver, cmpDataPos);
    uint64_t cmpDataSize;
    SZ3::read(cmpDataSize,  cmpDataPos);
    // cmpDataSize comes out of the buffer, so it can name an offset past its end.
    if (cmpDataSize > compressedSize - SZ3_HEADER_LEN) {
        return unreadable_config();
    }
    auto cmpConfPos = cmpDataPos + cmpDataSize;
    size_t remaining = compressedSize - SZ3_HEADER_LEN - cmpDataSize;
    auto conf = SZ3::Config{};
    try {
        conf.load(cmpConfPos, remaining);
    } catch (const std::exception &) {
        return unreadable_config();
    }
    return SZ3_Config(conf);
}

void dealloc_size_t(size_t * data) {
    delete[] data;
}
