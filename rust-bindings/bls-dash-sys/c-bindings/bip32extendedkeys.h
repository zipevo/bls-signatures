#ifndef BIP32EXTENDEDKEYS_H_
#define BIP32EXTENDEDKEYS_H_
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#ifdef __cplusplus
extern "C" {
#endif

typedef void* CExtendedPublicKey;
typedef void* CChainCode;

// ExtendedPublicKey
CExtendedPublicKey CExtendedPublicKeyFromBytes(
    const void* data,
    const bool legacy,
    bool* didErr);
CExtendedPublicKey CExtendedPublicKeyPublicChild(
    const CExtendedPublicKey pk,
    const uint32_t index,
    const bool legacy);
CChainCode CExtendedPublicKeyGetChainCode(const CExtendedPublicKey pk);
void* CExtendedPublicKeySerialize(const CExtendedPublicKey pk, const bool legacy);
void CExtendedPublicKeyFree(const CExtendedPublicKey pk);

// ChainCode
void* CChainCodeSerialize(const CChainCode cc);
void CChainCodeFree(const CChainCode cc);

#ifdef __cplusplus
}
#endif
#endif  // BIP32EXTENDEDKEYS_H_
