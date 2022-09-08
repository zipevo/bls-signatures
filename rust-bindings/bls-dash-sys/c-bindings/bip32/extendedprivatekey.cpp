#include "extendedprivatekey.h"

#include <vector>

#include "../blschia.h"
#include "../error.h"
#include "bls.hpp"
#include "extendedpublickey.h"

CBIP32ExtendedPublicKey CBIP32ExtendedPublicKeyFromBytes(
    const void* data,
    const bool legacy,
    bool* didErr)
{
    bls::ExtendedPublicKey* el = nullptr;
    try {
        el = new bls::ExtendedPublicKey(bls::ExtendedPublicKey::FromBytes(
            bls::Bytes((uint8_t*)(data), bls::ExtendedPublicKey::SIZE),
            legacy));
    } catch (const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    *didErr = false;
    return el;
}

CBIP32ExtendedPublicKey CBIP32ExtendedPublicKeyPublicChild(
    const CBIP32ExtendedPublicKey pk,
    const uint32_t index,
    const bool legacy)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    return new bls::ExtendedPublicKey(pkPtr->PublicChild(index, legacy));
}

CBIP32ChainCode CBIP32ExtendedPublicKeyGetChainCode(const CBIP32ExtendedPublicKey pk)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    return new bls::ChainCode(pkPtr->GetChainCode());
}

void* CBIP32ExtendedPublicKeySerialize(
    const CBIP32ExtendedPublicKey pk,
    const bool legacy)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    const std::vector<uint8_t> serialized = pkPtr->Serialize(legacy);
    uint8_t* buffer = (uint8_t*)malloc(bls::ExtendedPublicKey::SIZE);
    memcpy(buffer, serialized.data(), bls::ExtendedPublicKey::SIZE);
    return (void*)buffer;
}

bool CBIP32ExtendedPublicKeyIsEqual(
    const CBIP32ExtendedPublicKey pk1,
    const CBIP32ExtendedPublicKey pk2)
{
    const bls::ExtendedPublicKey* pk1Ptr = (bls::ExtendedPublicKey*)pk1;
    const bls::ExtendedPublicKey* pk2Ptr = (bls::ExtendedPublicKey*)pk2;
    return *pk1Ptr == *pk2Ptr;
}

void CBIP32ExtendedPublicKeyFree(const CBIP32ExtendedPublicKey pk)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    delete pkPtr;
}

CBIP32ExtendedPrivateKey CBIP32ExtendedPrivateKeyFromBytes(const void* data, bool* didErr)
{
    bls::ExtendedPrivateKey* el = nullptr;
    try {
        el = new bls::ExtendedPrivateKey(bls::ExtendedPrivateKey::FromBytes(
            bls::Bytes((uint8_t*)(data), bls::ExtendedPrivateKey::SIZE)));
    } catch (const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    *didErr = false;
    return el;
}

CBIP32ExtendedPrivateKey CBIP32ExtendedPrivateKeyFromSeed(const void* data, bool* didErr)
{
    bls::ExtendedPrivateKey* el = nullptr;
    try {
        el = new bls::ExtendedPrivateKey(bls::ExtendedPrivateKey::FromSeed(
            bls::Bytes((uint8_t*)(data), bls::ExtendedPrivateKey::SIZE)));
    } catch (const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    *didErr = false;
    return el;
}

CBIP32ExtendedPrivateKey CBIP32ExtendedPrivateKeyPrivateChild(
    const CBIP32ExtendedPrivateKey sk,
    const uint32_t index,
    const bool legacy)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ExtendedPrivateKey(skPtr->PrivateChild(index, legacy));
}

CBIP32ExtendedPublicKey CBIP32ExtendedPrivateKeyPublicChild(
    const CBIP32ExtendedPrivateKey sk,
    const uint32_t index)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ExtendedPublicKey(skPtr->PublicChild(index));
}

CBIP32ChainCode CBIP32ExtendedPrivateKeyGetChainCode(const CBIP32ExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ChainCode(skPtr->GetChainCode());
}

void* CBIP32ExtendedPrivateKeySerialize(const CBIP32ExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    uint8_t* buffer =
        bls::Util::SecAlloc<uint8_t>(bls::ExtendedPrivateKey::SIZE);
    skPtr->Serialize(buffer);

    return (void*)buffer;
}

bool CBIP32ExtendedPrivateKeyIsEqual(
    const CBIP32ExtendedPrivateKey sk1,
    const CBIP32ExtendedPrivateKey sk2)
{
    const bls::ExtendedPrivateKey* sk1Ptr = (bls::ExtendedPrivateKey*)sk1;
    const bls::ExtendedPrivateKey* sk2Ptr = (bls::ExtendedPrivateKey*)sk2;
    return *sk1Ptr == *sk2Ptr;
}

void* CBIP32ExtendedPrivateKeyGetPrivateKey(const CBIP32ExtendedPrivateKey sk)
{
    bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::PrivateKey(skPtr->GetPrivateKey());
}

void* CBIP32ExtendedPrivateKeyGetPublicKey(
    const CBIP32ExtendedPrivateKey sk,
    bool* didErr)
{
    bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    bls::G1Element* el = nullptr;
    try {
        el = new bls::G1Element(skPtr->GetPublicKey());
        *didErr = false;
    } catch (const std::exception& ex) {
        // set err
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return el;
}

CBIP32ExtendedPublicKey CBIP32ExtendedPrivateKeyGetExtendedPublicKey(
    const CBIP32ExtendedPrivateKey sk,
    const bool legacy,
    bool* didErr)
{
    bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    bls::ExtendedPublicKey* pk = nullptr;
    try {
        pk = new bls::ExtendedPublicKey(skPtr->GetExtendedPublicKey(legacy));
        *didErr = false;
    } catch (const std::exception& ex) {
        // set err
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return pk;
}

void CBIP32ExtendedPrivateKeyFree(const CBIP32ExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    delete skPtr;
}
