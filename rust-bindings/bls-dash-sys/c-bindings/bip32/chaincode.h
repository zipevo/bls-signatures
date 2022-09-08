#ifndef BIP32CHAINCODE_H_
#define BIP32CHAINCODE_H_

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* CBIP32ChainCode;

void* CBIP32ChainCodeSerialize(const CBIP32ChainCode cc);
bool CBIP32ChainCodeIsEqual(const CBIP32ChainCode cc1, const CBIP32ChainCode cc2);
void CBIP32ChainCodeFree(const CBIP32ChainCode cc);

#ifdef __cplusplus
}
#endif
#endif  // BIP32CHAINCODE_H_