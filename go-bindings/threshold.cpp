// Copyright (c) 2021 The Dash Core developers

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//    http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <vector>
#include <stdint.h>
#include "bls.hpp"
#include "privatekey.h"
#include "elements.h"
#include "blschia.h"
#include "threshold.h"
#include "utils.hpp"
#include "error.h"

CPrivateKey CThresholdPrivateKeyShare(void** sks, const size_t sksLen, const void* id, size_t idLen, bool* didErr) {
    bls::PrivateKey* sk = nullptr;
    try {
        sk = new bls::PrivateKey(
            bls::Threshold::PrivateKeyShare(
                toBLSVector<bls::PrivateKey>(sks, sksLen),
                bls::Bytes((uint8_t*)id, idLen)
            )
        );
    } catch(const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return sk;
}

CPrivateKey CThresholdPrivateKeyRecover(void** sks,
                                        const size_t sksLen,
                                        void** msgs,
                                        const void* msgsLens,
                                        const size_t msgsLen,
                                        bool* didErr) {
    const size_t* msgLensPtr = (size_t*)msgsLens;
    const std::vector<bls::PrivateKey> vecPrivKeys = toBLSVector<bls::PrivateKey>(sks, sksLen);
    const std::vector<size_t> vecMsgsLens = std::vector<size_t>(msgLensPtr, msgLensPtr + msgsLen);
    const std::vector<bls::Bytes> vecMsgs = toVectorBytes(msgs, msgsLen, vecMsgsLens);
    bls::PrivateKey* sk = nullptr;
    try {
        sk = new bls::PrivateKey(
            bls::Threshold::PrivateKeyRecover(vecPrivKeys, vecMsgs)
        );
    } catch(const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return sk;
}

CG1Element CThresholdPublicKeyShare(void** pks, const size_t pksLen, const void* id, size_t idLen, bool* didErr) {
    bls::G1Element* el = nullptr;
    try {
        el = new bls::G1Element(
            bls::Threshold::PublicKeyShare(
                toBLSVector<bls::G1Element>(pks, pksLen),
                bls::Bytes((uint8_t*)id, idLen)
            )
        );
    } catch(const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return el;
}

CG1Element CThresholdPublicKeyRecover(void** pks,
                                      size_t pksLen,
                                      void** msgs,
                                      const void* msgsLens,
                                      const size_t msgsLen,
                                      bool* didErr) {
    const size_t* msgLensPtr = (size_t*)msgsLens;
    const std::vector<bls::G1Element> vecPubKeys = toBLSVector<bls::G1Element>(pks, pksLen);
    const std::vector<size_t> vecMsgsLens = std::vector<size_t>(msgLensPtr, msgLensPtr + msgsLen);
    const std::vector<bls::Bytes> vecMsgs = toVectorBytes(msgs, msgsLen, vecMsgsLens);
    bls::G1Element* el = nullptr;
    try {
        el = new bls::G1Element(
            bls::Threshold::PublicKeyRecover(vecPubKeys, vecMsgs)
        );
    } catch(const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return el;
}

CG2Element CThresholdSignatureShare(void** sigs, const size_t sigsLen, const void* id, const size_t idLen, bool* didErr) {
    bls::G2Element* el = nullptr;
    try {
        el = new bls::G2Element(
            bls::Threshold::SignatureShare(
                toBLSVector<bls::G2Element>(sigs, sigsLen),
                bls::Bytes((uint8_t*)id, idLen)
            )
        );
    } catch(const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return el;
}

CG2Element CThresholdSignatureRecover(void** sigs,
                                      const size_t sigsLen,
                                      void** msgs,
                                      const void* msgsLens,
                                      const size_t msgsLen,
                                      bool* didErr) {
    const size_t* msgLensPtr = (size_t*)msgsLens;
    const std::vector<size_t> vecMsgsLens = std::vector<size_t>(msgLensPtr, msgLensPtr + msgsLen);
    bls::G2Element* el = nullptr;
    try {
        el = new bls::G2Element(
            bls::Threshold::SignatureRecover(
                toBLSVector<bls::G2Element>(sigs, sigsLen),
                toVectorBytes(msgs, msgsLen, vecMsgsLens)
            )
        );
    } catch(const std::exception& ex) {
        gErrMsg = ex.what();
        *didErr = true;
        return nullptr;
    }
    return el;
}

CG2Element CThresholdSign(const CPrivateKey sk, const void* msg, size_t msgLen) {
    bls::PrivateKey* skPtr = (bls::PrivateKey*)sk;
    bls::G2Element sig = bls::Threshold::Sign(*skPtr, bls::Bytes((uint8_t*)msg, msgLen));
    return new bls::G2Element(sig);
}

bool CThresholdVerify(const CG1Element pk, const void* msg, size_t msgLen, const CG2Element sig) {
    bls::G1Element* pkPtr = (bls::G1Element*)pk;
    bls::G2Element* sigPtr = (bls::G2Element*)sig;
    return bls::Threshold::Verify(*pkPtr, bls::Bytes((uint8_t*)msg, msgLen), *sigPtr);
}
