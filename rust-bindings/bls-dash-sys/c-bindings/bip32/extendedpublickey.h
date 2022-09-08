#ifndef BIP32EXTENDEDPUBLICKEY_H_
#define BIP32EXTENDEDPUBLICKEY_H_
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#include "chaincode.h"
#ifdef __cplusplus
extern "C" {
#endif

typedef void* CBIP32ExtendedPublicKey;

// ExtendedPublicKey
CBIP32ExtendedPublicKey CBIP32ExtendedPublicKeyFromBytes(
    const void* data,
    const bool legacy,
    bool* didErr);
CBIP32ExtendedPublicKey CBIP32ExtendedPublicKeyPublicChild(
    const CBIP32ExtendedPublicKey pk,
    const uint32_t index,
    const bool legacy);
CBIP32ChainCode CBIP32ExtendedPublicKeyGetChainCode(const CBIP32ExtendedPublicKey pk);
void* CBIP32ExtendedPublicKeySerialize(
    const CBIP32ExtendedPublicKey pk,
    const bool legacy);
bool CBIP32ExtendedPublicKeyIsEqual(
    const CBIP32ExtendedPublicKey pk1,
    const CBIP32ExtendedPublicKey pk2);
void CBIP32ExtendedPublicKeyFree(const CBIP32ExtendedPublicKey pk);

#ifdef __cplusplus
}
#endif
#endif  // BIP32EXTENDEDPUBLICKEY_H_
