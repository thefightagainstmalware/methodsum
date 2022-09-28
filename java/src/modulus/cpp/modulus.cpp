#include <jni.h>
#include <iostream>
#ifdef __cplusplus
extern "C" {
#endif
JNIEXPORT jlong JNICALL Java_gq_malwarefight_methodhash_MethodSum_unsignedLongMod
        (JNIEnv * env, jclass cls, jlong dividend, jlong divisor) {
    return (jlong) ((unsigned long long int) (dividend)) % ((unsigned long long int) divisor);
}
#ifdef __cplusplus
}
#endif