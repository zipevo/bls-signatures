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

package blschia

// #include "blschia.h"
// #include <string.h>
import "C"
import (
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"unsafe"
)

// HashSize ...
const HashSize = 32

// Hash represents 32 byte of hash data
type Hash [32]byte

var errWrongHexStringValue = errors.New("a hex string must contain 32 bytes")

// HashFromString convert a hex string into a Hash
func HashFromString(hexString string) (Hash, error) {
	var hash Hash
	data, err := hex.DecodeString(hexString)
	if err != nil {
		return hash, err
	}
	if len(data) < HashSize {
		return hash, errWrongHexStringValue
	}
	for i, d := range data[len(data)-HashSize:] {
		hash[HashSize-(i+1)] = d
	}
	return hash, nil
}

// BuildSignHash creates the required signHash for LLMQ threshold signing process
func BuildSignHash(llmqType uint8, quorumHash Hash, signID Hash, msgHash Hash) Hash {
	hasher := sha256.New()
	hasher.Write([]byte{llmqType})
	hasher.Write(quorumHash[:])
	hasher.Write(signID[:])
	hasher.Write(msgHash[:])
	return sha256.Sum256(hasher.Sum(nil))
}

func cAllocBytes(data []byte) unsafe.Pointer {
	l := C.size_t(len(data))
	ptr := unsafe.Pointer(C.SecAllocBytes(l))
	C.memcpy(ptr, unsafe.Pointer(&data[0]), l)
	return ptr
}
