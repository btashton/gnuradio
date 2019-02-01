#ifndef RUST_LIB
#define RUST_LIB

#ifdef __cplusplus
extern "C" {
#endif

void rust_32f_x2_multiply_32f(float* cVector, const float* aVector, const float* bVector, unsigned int num_points);
void rust_f_32f_x2_multiply_32f(float* cVector, const float* aVector, const float* bVector, unsigned int num_points);
#ifdef __cplusplus
}
#endif

#endif /* RUST_LIB */
