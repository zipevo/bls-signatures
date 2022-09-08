#include "chaincode.h"

#include <vector>

#include "bls.hpp"

void* CBIP32ChainCodeSerialize(const CBIP32ChainCode cc)
{
    const bls::ChainCode* ccPtr = (bls::ChainCode*)cc;
    const std::vector<uint8_t> serialized = ccPtr->Serialize();
    uint8_t* buffer = (uint8_t*)malloc(bls::ChainCode::SIZE);
    memcpy(buffer, serialized.data(), bls::ChainCode::SIZE);
    return (void*)buffer;
}

bool CBIP32ChainCodeIsEqual(const CBIP32ChainCode cc1, const CBIP32ChainCode cc2)
{
    const bls::ChainCode* cc1Ptr = (bls::ChainCode*)cc1;
    const bls::ChainCode* cc2Ptr = (bls::ChainCode*)cc2;
    return *cc1Ptr == *cc2Ptr;
}

void CBIP32ChainCodeFree(const CBIP32ChainCode cc)
{
    const bls::ChainCode* ccPtr = (bls::ChainCode*)cc;
    delete ccPtr;
}