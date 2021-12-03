package blschia

// #include <stdlib.h>
// #include "threshold.h"
// #include "blschia.h"
import "C"
import (
	"runtime"
	"unsafe"
)

// ThresholdPrivateKeyShare retrieves a shared PrivateKey from a set of PrivateKey and an arbitrary data
// this function is a binding of bls::Threshold::PrivateKeyShare
func ThresholdPrivateKeyShare(sks []*PrivateKey, data []byte) (*PrivateKey, error) {
	cDataPtr := C.CBytes(data)
	defer C.free(cDataPtr)
	cPrivKeyArrPtr := C.AllocPtrArray(C.size_t(len(sks)))
	for i, privKey := range sks {
		C.SetPtrArray(cPrivKeyArrPtr, unsafe.Pointer(privKey.val), C.int(i))
	}
	defer C.FreePtrArray(cPrivKeyArrPtr)
	var cDidErr C.bool
	sk := PrivateKey{
		val: C.CThresholdPrivateKeyShare(cPrivKeyArrPtr, C.size_t(len(sks)), cDataPtr, C.size_t(len(data)), &cDidErr),
	}
	if cDidErr {
		return nil, errFromC()
	}
	runtime.SetFinalizer(&sk, func(pk *PrivateKey) { sk.free() })
	return &sk, nil
}

// ThresholdPublicKeyShare retrieves a shared G1Element (public key) from a set of G1Element and an arbitrary data
// this function is a binding of bls::Threshold::PublicKeyShare
func ThresholdPublicKeyShare(pks []*G1Element, data []byte) (*G1Element, error) {
	cDataPtr := C.CBytes(data)
	defer C.free(cDataPtr)
	cArrPtr := C.AllocPtrArray(C.size_t(len(pks)))
	for i, pk := range pks {
		C.SetPtrArray(cArrPtr, unsafe.Pointer(pk.val), C.int(i))
	}
	defer C.FreePtrArray(cArrPtr)
	var cDidErr C.bool
	pk := G1Element{
		val: C.CThresholdPublicKeyShare(cArrPtr, C.size_t(len(pks)), cDataPtr, C.size_t(len(data)), &cDidErr),
	}
	if cDidErr {
		return nil, errFromC()
	}
	runtime.SetFinalizer(&pk, func(pk *G1Element) { pk.free() })
	return &pk, nil
}

// ThresholdSignatureShare retrieves a shared G2Element (signature) from a set of G2Element and an arbitrary data
// this function is a binding of bls::Threshold::SignatureShare
func ThresholdSignatureShare(sigs []*G2Element, data []byte) (*G2Element, error) {
	cDataPtr := C.CBytes(data)
	defer C.free(cDataPtr)
	cArrPtr := C.AllocPtrArray(C.size_t(len(sigs)))
	for i, sig := range sigs {
		C.SetPtrArray(cArrPtr, unsafe.Pointer(sig.val), C.int(i))
	}
	defer C.FreePtrArray(cArrPtr)
	var cDidErr C.bool
	sig := G2Element{
		val: C.CThresholdSignatureShare(cArrPtr, C.size_t(len(sigs)), cDataPtr, C.size_t(len(data)), &cDidErr),
	}
	if cDidErr {
		return nil, errFromC()
	}
	runtime.SetFinalizer(&sig, func(pk *G2Element) { sig.free() })
	return &sig, nil
}

// ThresholdPrivateKeyRecover recovers PrivateKey from the set of shared PrivateKey with a list of messages
// this function is a binding of bls::Threshold::PrivateKeyRecover
func ThresholdPrivateKeyRecover(sks []*PrivateKey, msgs [][]byte) (*PrivateKey, error) {
	cArrPtr := C.AllocPtrArray(C.size_t(len(sks)))
	for i, sk := range sks {
		C.SetPtrArray(cArrPtr, unsafe.Pointer(sk.val), C.int(i))
	}
	defer C.FreePtrArray(cArrPtr)
	cMsgArrPtr, msgLens := cAllocMsgs(msgs)
	defer C.FreePtrArray(cMsgArrPtr)
	var cDidErr C.bool
	sk := PrivateKey{
		val: C.CThresholdPrivateKeyRecover(
			cArrPtr,
			C.size_t(len(sks)),
			cMsgArrPtr,
			unsafe.Pointer(&msgLens[0]),
			C.size_t(len(msgs)),
			&cDidErr,
		),
	}
	if cDidErr {
		return nil, errFromC()
	}
	runtime.SetFinalizer(&sk, func(sk *PrivateKey) { sk.free() })
	return &sk, nil
}

// ThresholdPublicKeyRecover recovers G1Element (public key) from the set of shared G1Element with a list of messages
// this function is a binding of bls::Threshold::PublicKeyRecover
func ThresholdPublicKeyRecover(pks []*G1Element, msgs [][]byte) (*G1Element, error) {
	cArrPtr := C.AllocPtrArray(C.size_t(len(pks)))
	for i, pk := range pks {
		C.SetPtrArray(cArrPtr, unsafe.Pointer(pk.val), C.int(i))
	}
	defer C.FreePtrArray(cArrPtr)
	cMsgArrPtr, msgLens := cAllocMsgs(msgs)
	defer C.FreePtrArray(cMsgArrPtr)
	var cDidErr C.bool
	pk := G1Element{
		val: C.CThresholdPublicKeyRecover(
			cArrPtr,
			C.size_t(len(pks)),
			cMsgArrPtr,
			unsafe.Pointer(&msgLens[0]),
			C.size_t(len(msgs)),
			&cDidErr,
		),
	}
	if cDidErr {
		return nil, errFromC()
	}
	runtime.SetFinalizer(&pk, func(pk *G1Element) { pk.free() })
	return &pk, nil
}

// ThresholdSignatureRecover recovers G2Element (signature) from the set of shared G2Element with a list of messages
// this function is a binding of bls::Threshold::SignatureRecover
func ThresholdSignatureRecover(sigs []*G2Element, msgs [][]byte) (*G2Element, error) {
	cArrPtr := C.AllocPtrArray(C.size_t(len(sigs)))
	for i, sig := range sigs {
		C.SetPtrArray(cArrPtr, unsafe.Pointer(sig.val), C.int(i))
	}
	defer C.FreePtrArray(cArrPtr)
	cMsgArrPtr, msgLens := cAllocMsgs(msgs)
	defer C.FreePtrArray(cMsgArrPtr)
	var cDidErr C.bool
	sig := G2Element{
		val: C.CThresholdSignatureRecover(
			cArrPtr,
			C.size_t(len(sigs)),
			cMsgArrPtr,
			unsafe.Pointer(&msgLens[0]),
			C.size_t(len(msgs)),
			&cDidErr,
		),
	}
	if cDidErr {
		return nil, errFromC()
	}
	runtime.SetFinalizer(&sig, func(sig *G2Element) { sig.free() })
	return &sig, nil
}

// ThresholdSign signs of the data with PrivateKey
// this function is a binding of bls::Threshold::Sign
func ThresholdSign(sk *PrivateKey, data []byte) *G2Element {
	cDataPtr := C.CBytes(data)
	defer C.free(cDataPtr)
	sig := G2Element{
		val: C.CThresholdSign(sk.val, cDataPtr, C.size_t(len(data))),
	}
	runtime.SetFinalizer(&sig, func(sig *G2Element) { sig.free() })
	return &sig
}

// ThresholdVerify verifies of the data with G1Element (public key)
// this function is a binding of bls::Threshold::Verify
func ThresholdVerify(pk *G1Element, data []byte, sig *G2Element) bool {
	cDataPtr := C.CBytes(data)
	defer C.free(cDataPtr)
	val := C.CThresholdVerify(pk.val, cDataPtr, C.size_t(len(data)), sig.val)
	return bool(val)
}
