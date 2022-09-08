#ifndef BIP32EXTENDEDPRIVATEKEY_H_
#define BIP32EXTENDEDPRIVATEKEY_H_

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#include "chaincode.h"
#include "extendedpublickey.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* CBIP32ExtendedPrivateKey;

// ExtendedPrivateKey
CBIP32ExtendedPrivateKey CBIP32ExtendedPrivateKeyFromBytes(
    const void* data,
    bool* didErr);
CBIP32ExtendedPrivateKey CBIP32ExtendedPrivateKeyFromSeed(const void* data, bool* didErr);
CBIP32ExtendedPrivateKey CBIP32ExtendedPrivateKeyPrivateChild(
    const CBIP32ExtendedPrivateKey sk,
    const uint32_t index,
    const bool legacy);
CBIP32ExtendedPublicKey CBIP32ExtendedPrivateKeyPublicChild(
    const CBIP32ExtendedPrivateKey sk,
    const uint32_t index);
CBIP32ChainCode CBIP32ExtendedPrivateKeyGetChainCode(const CBIP32ExtendedPrivateKey sk);
void* CBIP32ExtendedPrivateKeySerialize(const CBIP32ExtendedPrivateKey sk);
bool CBIP32ExtendedPrivateKeyIsEqual(
    const CBIP32ExtendedPrivateKey sk1,
    const CBIP32ExtendedPrivateKey sk2);
void* CBIP32ExtendedPrivateKeyGetPrivateKey(const CBIP32ExtendedPrivateKey sk);
CBIP32ExtendedPublicKey CBIP32ExtendedPrivateKeyGetExtendedPublicKey(
    const CBIP32ExtendedPrivateKey sk,
    const bool legacy,
    bool* didErr);
void* CBIP32ExtendedPrivateKeyGetPublicKey(
    const CBIP32ExtendedPrivateKey sk,
    bool* didErr);
void CBIP32ExtendedPrivateKeyFree(const CBIP32ExtendedPrivateKey sk);

#ifdef __cplusplus
}
#endif
#endif  // BIP32EXTENDEDPRIVATEKEY_H_
