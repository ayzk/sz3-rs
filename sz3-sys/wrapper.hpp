#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <new>
#include <stdexcept>

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


// What a wrapper function returns. On an error other than SZ3_OK it also sets *message to a copy of the exception's
// message, which the caller frees with free_error_message.
enum SZ3_ErrorKind : int {
    SZ3_OK = 0,
    SZ3_INVALID_ARGUMENT = 1,
    SZ3_OUT_OF_RANGE = 2,
    SZ3_RUNTIME = 3,
    SZ3_OUT_OF_MEMORY = 4,
    SZ3_OTHER = 5,
};

// malloc, not new: it runs after a std::bad_alloc too, and gives nullptr instead of throwing.
static SZ3_ErrorKind fail(SZ3_ErrorKind kind, const char * what, char ** message) {
    size_t len = std::strlen(what);
    *message = static_cast<char *>(std::malloc(len + 1));
    if (*message) {
        std::memcpy(*message, what, len + 1);
    }
    return kind;
}

// An exception must not cross into Rust: each is turned into its kind and message.
template <typename F>
static SZ3_ErrorKind catch_sz3(char ** message, F && f) {
    try {
        f();
        return SZ3_OK;
    } catch (const std::invalid_argument & e) {
        return fail(SZ3_INVALID_ARGUMENT, e.what(), message);
    } catch (const std::out_of_range & e) {
        return fail(SZ3_OUT_OF_RANGE, e.what(), message);
    } catch (const std::bad_alloc & e) {
        return fail(SZ3_OUT_OF_MEMORY, e.what(), message);
    } catch (const std::runtime_error & e) {
        return fail(SZ3_RUNTIME, e.what(), message);
    } catch (const std::exception & e) {
        return fail(SZ3_OTHER, e.what(), message);
    } catch (...) {
        return fail(SZ3_OTHER, "unknown exception", message);
    }
}

void free_error_message(char * message) {
    std::free(message);
}

#define func(ns, type, dt) \
  namespace ns { \
    using ty = type; \
	enum DATA_TYPE : uint8_t { \
      TYPE = dt \
    }; \
    SZ3_ErrorKind compress_size_bound(SZ3_Config config, size_t * bound, char ** message) { \
        return catch_sz3(message, [&] { *bound = SZ3::SZ_compress_size_bound<ty>(config.into()); }); \
    } \
    SZ3_ErrorKind compress(SZ3_Config config, const ty * data, char * compressedData, size_t compressedCapacity, \
                           size_t * compressedSize, char ** message) { \
        return catch_sz3(message, [&] { \
            *compressedSize = SZ_compress<ty>(config.into(), data, compressedData, compressedCapacity); \
        }); \
    } \
    SZ3_ErrorKind decompress(const char * compressedData, size_t compressedSize, ty * decompressedData, \
                             char ** message) { \
        return catch_sz3(message, [&] { \
            auto conf = SZ3::Config{}; \
            SZ_decompress<ty>(conf, compressedData, compressedSize, decompressedData); \
        }); \
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

SZ3_ErrorKind decompress_config(const char * compressedData, size_t compressedSize, SZ3_Config * config,
                                char ** message) {
    if (compressedSize < SZ3_HEADER_LEN) {
        return fail(SZ3_OUT_OF_RANGE, "SZ3 data is shorter than its header", message);
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
        return fail(SZ3_OUT_OF_RANGE, "SZ3 data is shorter than the payload its header declares", message);
    }
    auto cmpConfPos = cmpDataPos + cmpDataSize;
    size_t remaining = compressedSize - SZ3_HEADER_LEN - cmpDataSize;
    return catch_sz3(message, [&] {
        auto conf = SZ3::Config{};
        conf.load(cmpConfPos, remaining);
        *config = SZ3_Config(conf);
    });
}

void dealloc_size_t(size_t * data) {
    delete[] data;
}
