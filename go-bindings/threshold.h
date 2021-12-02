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

#ifndef GO_BINDINGS_THRESHOLD_H_
#define GO_BINDINGS_THRESHOLD_H_
#include <stdbool.h>
#include <stdlib.h>
#include <stdint.h>
#include "privatekey.h"
#include "elements.h"
#ifdef __cplusplus
extern "C" {
#endif

CPrivateKey CThresholdPrivateKeyShare(void** sks, const size_t sksLen, const void* id, size_t idLen);
CPrivateKey CThresholdPrivateKeyRecover(void** sks,
                                        const size_t sksLen,
                                        void** msgs,
                                        const void* msgsLens,
                                        const size_t msgsLen);

CG1Element CThresholdPublicKeyShare(void** pks, const size_t pksLen, const void* id, size_t idLen);
CG1Element CThresholdPublicKeyRecover(void** pks,
                                      size_t pksLen,
                                      void** msgs,
                                      const void* msgsLens,
                                      const size_t msgsLen);

CG2Element CThresholdSignatureShare(void** sigs, const size_t sigsLen, const void* id, const size_t idLen);
CG2Element CThresholdSignatureRecover(void** sigs,
                                      const size_t sigsLen,
                                      void** msgs,
                                      const void* msgsLens,
                                      const size_t msgsLen);

CG2Element CThresholdSign(const CPrivateKey sk, const void* msg, size_t msgLen);
bool CThresholdVerify(const CG1Element pk, const void* msg, size_t msgLen, const CG2Element sig);

#ifdef __cplusplus
}
#endif
#endif  // GO_BINDINGS_THRESHOLD_H_
