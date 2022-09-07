#include "bip32extendedkeys.h"

#include <vector>

#include "bls.hpp"
#include "blschia.h"
#include "error.h"

CExtendedPublicKey CExtendedPublicKeyFromBytes(
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

CExtendedPublicKey CExtendedPublicKeyPublicChild(
    const CExtendedPublicKey pk,
    const uint32_t index,
    const bool legacy)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    return new bls::ExtendedPublicKey(pkPtr->PublicChild(index, legacy));
}

CChainCode CExtendedPublicKeyGetChainCode(const CExtendedPublicKey pk)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    return new bls::ChainCode(pkPtr->GetChainCode());
}

void* CExtendedPublicKeySerialize(
    const CExtendedPublicKey pk,
    const bool legacy)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    const std::vector<uint8_t> serialized = pkPtr->Serialize(legacy);
    uint8_t* buffer = (uint8_t*)malloc(bls::ExtendedPublicKey::SIZE);
    memcpy(buffer, serialized.data(), bls::ExtendedPublicKey::SIZE);
    return (void*)buffer;
}

bool CExtendedPublicKeyIsEqual(
    const CExtendedPublicKey pk1,
    const CExtendedPublicKey pk2)
{
    const bls::ExtendedPublicKey* pk1Ptr = (bls::ExtendedPublicKey*)pk1;
    const bls::ExtendedPublicKey* pk2Ptr = (bls::ExtendedPublicKey*)pk2;
    return *pk1Ptr == *pk2Ptr;
}

void CExtendedPublicKeyFree(const CExtendedPublicKey pk)
{
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    delete pkPtr;
}

CExtendedPrivateKey CExtendedPrivateKeyFromBytes(const void* data, bool* didErr)
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

CExtendedPrivateKey CExtendedPrivateKeyFromSeed(const void* data, bool* didErr)
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

CExtendedPrivateKey CExtendedPrivateKeyPrivateChild(
    const CExtendedPrivateKey sk,
    const uint32_t index,
    const bool legacy)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ExtendedPrivateKey(skPtr->PrivateChild(index, legacy));
}

CExtendedPublicKey CExtendedPrivateKeyPublicChild(
    const CExtendedPrivateKey sk,
    const uint32_t index)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ExtendedPublicKey(skPtr->PublicChild(index));
}

CChainCode CExtendedPrivateKeyGetChainCode(const CExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::ChainCode(skPtr->GetChainCode());
}

void* CExtendedPrivateKeySerialize(const CExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    uint8_t* buffer =
        bls::Util::SecAlloc<uint8_t>(bls::ExtendedPrivateKey::SIZE);
    skPtr->Serialize(buffer);

    return (void*)buffer;
}

bool CExtendedPrivateKeyIsEqual(
    const CExtendedPrivateKey sk1,
    const CExtendedPrivateKey sk2)
{
    const bls::ExtendedPrivateKey* sk1Ptr = (bls::ExtendedPrivateKey*)sk1;
    const bls::ExtendedPrivateKey* sk2Ptr = (bls::ExtendedPrivateKey*)sk2;
    return *sk1Ptr == *sk2Ptr;
}

void* CExtendedPrivateKeyGetPrivateKey(const CExtendedPrivateKey sk)
{
    bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    return new bls::PrivateKey(skPtr->GetPrivateKey());
}

void* CExtendedPrivateKeyGetPublicKey(
    const CExtendedPrivateKey sk,
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

CExtendedPublicKey CExtendedPrivateKeyGetExtendedPublicKey(
    const CExtendedPrivateKey sk,
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

void CExtendedPrivateKeyFree(const CExtendedPrivateKey sk)
{
    const bls::ExtendedPrivateKey* skPtr = (bls::ExtendedPrivateKey*)sk;
    delete skPtr;
}

void* CChainCodeSerialize(const CChainCode cc)
{
    const bls::ChainCode* ccPtr = (bls::ChainCode*)cc;
    const std::vector<uint8_t> serialized = ccPtr->Serialize();
    uint8_t* buffer = (uint8_t*)malloc(bls::ChainCode::SIZE);
    memcpy(buffer, serialized.data(), bls::ChainCode::SIZE);
    return (void*)buffer;
}

bool CChainCodeIsEqual(const CChainCode cc1, const CChainCode cc2)
{
    const bls::ChainCode* cc1Ptr = (bls::ChainCode*)cc1;
    const bls::ChainCode* cc2Ptr = (bls::ChainCode*)cc2;
    return *cc1Ptr == *cc2Ptr;
}

void CChainCodeFree(const CChainCode cc)
{
    const bls::ChainCode* ccPtr = (bls::ChainCode*)cc;
    delete ccPtr;
}
