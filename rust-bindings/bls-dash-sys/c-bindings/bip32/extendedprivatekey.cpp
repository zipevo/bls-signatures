#include "extendedprivatekey.h"

#include <vector>

#include "../blschia.h"
#include "../error.h"
#include "bls.hpp"
#include "extendedpublickey.h"

BIP32ExtendedPublicKey BIP32ExtendedPublicKeyFromBytes(
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

BIP32ExtendedPublicKey BIP32ExtendedPublicKeyPublicChild(
    const BIP32ExtendedPublicKey pk,
    const uint32_t index,
    const bool legacy)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    return new bls::ExtendedPublicKey(pkPtr->PublicChild(index, legacy));
}

BIP32ChainCode BIP32ExtendedPublicKeyGetChainCode(const BIP32ExtendedPublicKey pk)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    return new bls::ChainCode(pkPtr->GetChainCode());
}

void* BIP32ExtendedPublicKeySerialize(
    const BIP32ExtendedPublicKey pk,
    const bool legacy)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    const std::vector<uint8_t> serialized = pkPtr->Serialize(legacy);
    uint8_t* buffer = (uint8_t*)malloc(bls::ExtendedPublicKey::SIZE);
    memcpy(buffer, serialized.data(), bls::ExtendedPublicKey::SIZE);
    return (void*)buffer;
}

bool BIP32ExtendedPublicKeyIsEqual(
    const BIP32ExtendedPublicKey pk1,
    const BIP32ExtendedPublicKey pk2)
{
    const bls::ExtendedPublicKey* pk1Ptr = (bls::ExtendedPublicKey*)pk1;
    const bls::ExtendedPublicKey* pk2Ptr = (bls::ExtendedPublicKey*)pk2;
    return *pk1Ptr == *pk2Ptr;
}

void BIP32ExtendedPublicKeyFree(const BIP32ExtendedPublicKey pk)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    delete pkPtr;
}

BIP32ExtendedPrivateKey BIP32ExtendedPrivateKeyFromBytes(const void* data, bool* didErr)
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

BIP32ExtendedPrivateKey BIP32ExtendedPrivateKeyFromSeed(const void* data, bool* didErr)
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

BIP32ExtendedPrivateKey BIP32ExtendedPrivateKeyPrivateChild(
    const BIP32ExtendedPrivateKey sk,
    const uint32_t index,
    const bool legacy)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ExtendedPrivateKey(skPtr->PrivateChild(index, legacy));
}

BIP32ExtendedPublicKey BIP32ExtendedPrivateKeyPublicChild(
    const BIP32ExtendedPrivateKey sk,
    const uint32_t index)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ExtendedPublicKey(skPtr->PublicChild(index));
}

BIP32ChainCode BIP32ExtendedPrivateKeyGetChainCode(const BIP32ExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ChainCode(skPtr->GetChainCode());
}

void* BIP32ExtendedPrivateKeySerialize(const BIP32ExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    uint8_t* buffer =
        bls::Util::SecAlloc<uint8_t>(bls::ExtendedPrivateKey::SIZE);
    skPtr->Serialize(buffer);

    return (void*)buffer;
}

bool BIP32ExtendedPrivateKeyIsEqual(
    const BIP32ExtendedPrivateKey sk1,
    const BIP32ExtendedPrivateKey sk2)
{
    const bls::ExtendedPrivateKey* sk1Ptr = (bls::ExtendedPrivateKey*)sk1;
    const bls::ExtendedPrivateKey* sk2Ptr = (bls::ExtendedPrivateKey*)sk2;
    return *sk1Ptr == *sk2Ptr;
}

void* BIP32ExtendedPrivateKeyGetPrivateKey(const BIP32ExtendedPrivateKey sk)
{
    bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::PrivateKey(skPtr->GetPrivateKey());
}

void* BIP32ExtendedPrivateKeyGetPublicKey(
    const BIP32ExtendedPrivateKey sk,
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

BIP32ExtendedPublicKey BIP32ExtendedPrivateKeyGetExtendedPublicKey(
    const BIP32ExtendedPrivateKey sk,
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

void BIP32ExtendedPrivateKeyFree(const BIP32ExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    delete skPtr;
}
