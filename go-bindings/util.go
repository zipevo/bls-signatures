package blschia

import (
	"crypto/sha256"
	"encoding/hex"
	"errors"
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
