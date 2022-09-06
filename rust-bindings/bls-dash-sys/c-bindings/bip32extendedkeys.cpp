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

CChainCode CExtendedPublicKeyGetChainCode(const CExtendedPublicKey pk) {
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    return new bls::ChainCode(pkPtr->GetChainCode());
}

void* CExtendedPublicKeySerialize(const CExtendedPublicKey pk, const bool legacy) {
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    const std::vector<uint8_t> serialized = pkPtr->Serialize(legacy);
    uint8_t* buffer = (uint8_t*)malloc(bls::ExtendedPublicKey::SIZE);
    memcpy(buffer, serialized.data(), bls::ExtendedPublicKey::SIZE);
    return (void*)buffer;
}

void CExtendedPublicKeyFree(const CExtendedPublicKey pk) {
    const bls::ExtendedPublicKey* pkPtr = (bls::ExtendedPublicKey*)pk;
    delete pkPtr;
}

void* CChainCodeSerialize(const CChainCode cc) {
    const bls::ChainCode* ccPtr = (bls::ChainCode*)cc;
    const std::vector<uint8_t> serialized = ccPtr->Serialize();
    uint8_t* buffer = (uint8_t*)malloc(bls::ChainCode::SIZE);
    memcpy(buffer, serialized.data(), bls::ChainCode::SIZE);
    return (void*)buffer;
}

void CChainCodeFree(const CChainCode cc) {
    const bls::ChainCode* ccPtr = (bls::ChainCode*)cc;
    delete ccPtr;
}
