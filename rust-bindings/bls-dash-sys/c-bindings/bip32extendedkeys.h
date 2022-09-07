#ifndef BIP32EXTENDEDKEYS_H_
#define BIP32EXTENDEDKEYS_H_
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#ifdef __cplusplus
extern "C" {
#endif

typedef void* CExtendedPublicKey;
typedef void* CExtendedPrivateKey;
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
void* CExtendedPublicKeySerialize(
    const CExtendedPublicKey pk,
    const bool legacy);
bool CExtendedPublicKeyIsEqual(
    const CExtendedPublicKey pk1,
    const CExtendedPublicKey pk2);
void CExtendedPublicKeyFree(const CExtendedPublicKey pk);

// ExtendedPrivateKey
CExtendedPrivateKey CExtendedPrivateKeyFromBytes(
    const void* data,
    bool* didErr);
CExtendedPrivateKey CExtendedPrivateKeyFromSeed(const void* data, bool* didErr);
CExtendedPrivateKey CExtendedPrivateKeyPrivateChild(
    const CExtendedPrivateKey sk,
    const uint32_t index,
    const bool legacy);
CExtendedPublicKey CExtendedPrivateKeyPublicChild(
    const CExtendedPrivateKey sk,
    const uint32_t index);
CChainCode CExtendedPrivateKeyGetChainCode(const CExtendedPrivateKey sk);
void* CExtendedPrivateKeySerialize(const CExtendedPrivateKey sk);
bool CExtendedPrivateKeyIsEqual(
    const CExtendedPrivateKey sk1,
    const CExtendedPrivateKey sk2);
void* CExtendedPrivateKeyGetPrivateKey(const CExtendedPrivateKey sk);
CExtendedPublicKey CExtendedPrivateKeyGetExtendedPublicKey(
    const CExtendedPrivateKey sk,
    const bool legacy,
    bool* didErr);
void* CExtendedPrivateKeyGetPublicKey(
    const CExtendedPrivateKey sk,
    bool* didErr);
void CExtendedPrivateKeyFree(const CExtendedPrivateKey sk);

// ChainCode
void* CChainCodeSerialize(const CChainCode cc);
bool CChainCodeIsEqual(const CChainCode cc1, const CChainCode cc2);
void CChainCodeFree(const CChainCode cc);

#ifdef __cplusplus
}
#endif
#endif  // BIP32EXTENDEDKEYS_H_
