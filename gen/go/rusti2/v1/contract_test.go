package rusti2v1

import (
	"bytes"
	"testing"

	"google.golang.org/protobuf/proto"
)

// This wire vector is also checked by the Rust SDK. Renumbering a field must
// fail rather than silently breaking callers compiled against older contracts.
func TestStatObjectWireContract(t *testing.T) {
	want := &StatObjectRequest{Bucket: "media", Key: "avatar"}
	wire := []byte("\x0a\x05media\x12\x06avatar")
	encoded, err := proto.Marshal(want)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(encoded, wire) {
		t.Fatalf("wire contract changed: %x", encoded)
	}
	var decoded StatObjectRequest
	if err := proto.Unmarshal(wire, &decoded); err != nil {
		t.Fatal(err)
	}
	if !proto.Equal(want, &decoded) {
		t.Fatalf("decoded contract: %v", &decoded)
	}
}
