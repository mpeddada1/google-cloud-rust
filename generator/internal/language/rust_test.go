// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package language

import (
	"fmt"
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/google/go-cmp/cmp/cmpopts"
	"github.com/googleapis/google-cloud-rust/generator/internal/genclient"
)

func createRustCodec() *RustCodec {
	wkt := &RustPackage{
		Name:    "gax_wkt",
		Package: "types",
		Path:    "../../types",
	}

	return &RustCodec{
		ModulePath:    "model",
		ExtraPackages: []*RustPackage{wkt},
		PackageMapping: map[string]*RustPackage{
			"google.protobuf": wkt,
		},
	}
}

func TestRust_ParseOptions(t *testing.T) {
	copts := &genclient.CodecOptions{
		Options: map[string]string{
			"package-name-override": "test-only",
			"copyright-year":        "2035",
			"module-path":           "alternative::generated",
			"package:wkt":           "package=types,path=src/wkt,source=google.protobuf,source=test-only",
			"package:gax":           "package=gax,path=src/gax,feature=sdk_client",
		},
	}
	codec, err := NewRustCodec(copts)
	if err != nil {
		t.Fatal(err)
	}
	gp := &RustPackage{
		Name:    "wkt",
		Package: "types",
		Path:    "src/wkt",
	}
	want := &RustCodec{
		PackageNameOverride:      "test-only",
		GenerationYear:           "2035",
		ModulePath:               "alternative::generated",
		DeserializeWithdDefaults: true,
		ExtraPackages: []*RustPackage{
			gp,
			{
				Name:    "gax",
				Package: "gax",
				Path:    "src/gax",
				Features: []string{
					"sdk_client",
				},
			},
		},
		PackageMapping: map[string]*RustPackage{
			"google.protobuf": gp,
			"test-only":       gp,
		},
	}
	if diff := cmp.Diff(want, codec, cmpopts.IgnoreFields(RustCodec{}, "ExtraPackages", "PackageMapping")); diff != "" {
		t.Errorf("codec mismatch (-want, +got):\n%s", diff)
	}
	if want.PackageNameOverride != codec.PackageNameOverride {
		t.Errorf("mismatched in packageNameOverride, want=%s, got=%s", want.PackageNameOverride, codec.PackageNameOverride)
	}
	checkRustPackages(t, codec, want)
}

func TestRust_RequiredPackages(t *testing.T) {
	copts := &genclient.CodecOptions{
		OutDir: "src/generated/newlib",
		Options: map[string]string{
			"package:gtype": "package=types,path=src/generated/type,source=google.type,source=test-only",
			"package:gax":   "package=gax,path=src/gax,version=1.2.3",
			"package:auth":  "ignore=true",
		},
	}
	codec, err := NewRustCodec(copts)
	if err != nil {
		t.Fatal(err)
	}
	got := codec.RequiredPackages()
	want := []string{
		"gtype      = { path = \"../../../src/generated/type\", package = \"types\" }",
		"gax        = { version = \"1.2.3\", path = \"../../../src/gax\", package = \"gax\" }",
	}
	less := func(a, b string) bool { return a < b }
	if diff := cmp.Diff(want, got, cmpopts.SortSlices(less)); diff != "" {
		t.Errorf("mismatched required packages (-want, +got):\n%s", diff)
	}
}

func TestRust_RequiredPackagesLocal(t *testing.T) {
	// This is not a thing we expect to do in the Rust repository, but the
	// behavior is consistent.
	copts := &genclient.CodecOptions{
		OutDir: "",
		Options: map[string]string{
			"package:gtype": "package=types,path=src/generated/type,source=google.type,source=test-only",
		},
	}
	codec, err := NewRustCodec(copts)
	if err != nil {
		t.Fatal(err)
	}
	got := codec.RequiredPackages()
	want := []string{
		"gtype      = { path = \"src/generated/type\", package = \"types\" }",
	}
	less := func(a, b string) bool { return a < b }
	if diff := cmp.Diff(want, got, cmpopts.SortSlices(less)); diff != "" {
		t.Errorf("mismatched required packages (-want, +got):\n%s", diff)
	}
}

func TestRust_PackageName(t *testing.T) {
	rustPackageNameImpl(t, "test-only-overridden", &genclient.CodecOptions{
		Options: map[string]string{
			"package-name-override": "test-only-overridden",
		},
	}, &genclient.API{
		Name:        "test-only-name",
		PackageName: "google.cloud.service.v3",
	})
	rustPackageNameImpl(t, "gcp-sdk-service-v3", &genclient.CodecOptions{}, &genclient.API{
		Name:        "test-only-name",
		PackageName: "google.cloud.service.v3",
	})
	rustPackageNameImpl(t, "gcp-sdk-type", &genclient.CodecOptions{}, &genclient.API{
		Name:        "type",
		PackageName: "",
	})
}

func rustPackageNameImpl(t *testing.T, want string, copts *genclient.CodecOptions, api *genclient.API) {
	t.Helper()
	codec, err := NewRustCodec(copts)
	if err != nil {
		t.Fatal(err)
	}
	got := codec.PackageName(api)
	if want != got {
		t.Errorf("mismatch in package name, want=%s, got=%s", want, got)
	}

}

func checkRustPackages(t *testing.T, got *RustCodec, want *RustCodec) {
	t.Helper()
	less := func(a, b *RustPackage) bool { return a.Name < b.Name }
	if diff := cmp.Diff(want.ExtraPackages, got.ExtraPackages, cmpopts.SortSlices(less)); diff != "" {
		t.Errorf("package mismatch (-want, +got):\n%s", diff)
	}
}

func TestRust_Validate(t *testing.T) {
	api := newTestAPI(
		[]*genclient.Message{{Name: "m1", Package: "p1"}},
		[]*genclient.Enum{{Name: "e1", Package: "p1"}},
		[]*genclient.Service{{Name: "s1", Package: "p1"}})
	c := &RustCodec{}
	if err := c.Validate(api); err != nil {
		t.Errorf("unexpected error in API validation %q", err)
	}
	if c.SourceSpecificationPackageName != "p1" {
		t.Errorf("mismatched source package name, want=p1, got=%s", c.SourceSpecificationPackageName)
	}
}

func TestRust_ValidateMessageMismatch(t *testing.T) {
	api := newTestAPI(
		[]*genclient.Message{{Name: "m1", Package: "p1"}, {Name: "m2", Package: "p2"}},
		[]*genclient.Enum{{Name: "e1", Package: "p1"}},
		[]*genclient.Service{{Name: "s1", Package: "p1"}})
	c := &RustCodec{}
	if err := c.Validate(api); err == nil {
		t.Errorf("expected an error in API validation got=%s", c.SourceSpecificationPackageName)
	}

	api = newTestAPI(
		[]*genclient.Message{{Name: "m1", Package: "p1"}},
		[]*genclient.Enum{{Name: "e1", Package: "p1"}, {Name: "e2", Package: "p2"}},
		[]*genclient.Service{{Name: "s1", Package: "p1"}})
	c = &RustCodec{}
	if err := c.Validate(api); err == nil {
		t.Errorf("expected an error in API validation got=%s", c.SourceSpecificationPackageName)
	}

	api = newTestAPI(
		[]*genclient.Message{{Name: "m1", Package: "p1"}},
		[]*genclient.Enum{{Name: "e1", Package: "p1"}},
		[]*genclient.Service{{Name: "s1", Package: "p1"}, {Name: "s2", Package: "p2"}})
	c = &RustCodec{}
	if err := c.Validate(api); err == nil {
		t.Errorf("expected an error in API validation got=%s", c.SourceSpecificationPackageName)
	}
}

func TestWellKnownTypesExist(t *testing.T) {
	api := newTestAPI([]*genclient.Message{}, []*genclient.Enum{}, []*genclient.Service{})
	c := &RustCodec{}
	c.LoadWellKnownTypes(api.State)
	for _, name := range []string{"Any", "Duration", "Empty", "FieldMask", "Timestamp"} {
		if _, ok := api.State.MessageByID[fmt.Sprintf(".google.protobuf.%s", name)]; !ok {
			t.Errorf("cannot find well-known message %s in API", name)
		}
	}
}

func TestRust_WellKnownTypesAsMethod(t *testing.T) {
	api := newTestAPI([]*genclient.Message{}, []*genclient.Enum{}, []*genclient.Service{})
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)

	want := "gax_wkt::Empty"
	got := c.MethodInOutTypeName(".google.protobuf.Empty", api.State)
	if want != got {
		t.Errorf("mismatched well-known type name as method argument or response, want=%s, got=%s", want, got)
	}
}

func TestRust_MethodInOut(t *testing.T) {
	message := &genclient.Message{
		Name: "Target",
		ID:   "..Target",
	}
	nested := &genclient.Message{
		Name:   "Nested",
		ID:     "..Target.Nested",
		Parent: message,
	}
	api := newTestAPI([]*genclient.Message{message, nested}, []*genclient.Enum{}, []*genclient.Service{})
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)

	want := "crate::model::Target"
	got := c.MethodInOutTypeName("..Target", api.State)
	if want != got {
		t.Errorf("mismatched well-known type name as method argument or response, want=%s, got=%s", want, got)
	}

	want = "crate::model::target::Nested"
	got = c.MethodInOutTypeName("..Target.Nested", api.State)
	if want != got {
		t.Errorf("mismatched well-known type name as method argument or response, want=%s, got=%s", want, got)
	}
}

func TestRust_FieldAttributes(t *testing.T) {
	message := &genclient.Message{
		Name: "Fake",
		ID:   "..Fake",
		Fields: []*genclient.Field{
			{
				Name:     "f_int64",
				JSONName: "fInt64",
				Typez:    genclient.INT64_TYPE,
				Optional: false,
				Repeated: false,
			},
			{
				Name:     "f_int64_optional",
				JSONName: "fInt64Optional",
				Typez:    genclient.INT64_TYPE,
				Optional: true,
				Repeated: false,
			},
			{
				Name:     "f_int64_repeated",
				JSONName: "fInt64Repeated",
				Typez:    genclient.INT64_TYPE,
				Optional: false,
				Repeated: true,
			},

			{
				Name:     "f_bytes",
				JSONName: "fBytes",
				Typez:    genclient.BYTES_TYPE,
				Optional: false,
				Repeated: false,
			},
			{
				Name:     "f_bytes_optional",
				JSONName: "fBytesOptional",
				Typez:    genclient.BYTES_TYPE,
				Optional: true,
				Repeated: false,
			},
			{
				Name:     "f_bytes_repeated",
				JSONName: "fBytesRepeated",
				Typez:    genclient.BYTES_TYPE,
				Optional: false,
				Repeated: true,
			},

			{
				Name:     "f_string",
				JSONName: "fString",
				Typez:    genclient.STRING_TYPE,
				Optional: false,
				Repeated: false,
			},
			{
				Name:     "f_string_optional",
				JSONName: "fStringOptional",
				Typez:    genclient.STRING_TYPE,
				Optional: true,
				Repeated: false,
			},
			{
				Name:     "f_string_repeated",
				JSONName: "fStringRepeated",
				Typez:    genclient.STRING_TYPE,
				Optional: false,
				Repeated: true,
			},
		},
	}
	api := newTestAPI([]*genclient.Message{message}, []*genclient.Enum{}, []*genclient.Service{})

	expectedAttributes := map[string]string{
		"f_int64":          `#[serde_as(as = "serde_with::DisplayFromStr")]`,
		"f_int64_optional": `#[serde_as(as = "Option<serde_with::DisplayFromStr>")]`,
		"f_int64_repeated": `#[serde_as(as = "Vec<serde_with::DisplayFromStr>")]`,

		"f_bytes":          `#[serde_as(as = "serde_with::base64::Base64")]`,
		"f_bytes_optional": `#[serde_as(as = "Option<serde_with::base64::Base64>")]`,
		"f_bytes_repeated": `#[serde_as(as = "Vec<serde_with::base64::Base64>")]`,

		"f_string":          ``,
		"f_string_optional": ``,
		"f_string_repeated": ``,
	}
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)
	for _, field := range message.Fields {
		want, ok := expectedAttributes[field.Name]
		if !ok {
			t.Fatalf("missing expected value for %s", field.Name)
		}
		got := strings.Join(c.FieldAttributes(field, api.State), "\n")
		if got != want {
			t.Errorf("mismatched field type for %s, got=%s, want=%s", field.Name, got, want)
		}
	}
}

func TestRust_MapFieldAttributes(t *testing.T) {
	target := &genclient.Message{
		Name: "Target",
		ID:   "..Target",
	}
	map1 := &genclient.Message{
		Name:  "$map<string, string>",
		ID:    "$map<string, string>",
		IsMap: true,
		Fields: []*genclient.Field{
			{
				Name:  "key",
				Typez: genclient.STRING_TYPE,
			},
			{
				Name:  "value",
				Typez: genclient.STRING_TYPE,
			},
		},
	}
	map2 := &genclient.Message{
		Name:  "$map<string, int64>",
		ID:    "$map<string, int64>",
		IsMap: true,
		Fields: []*genclient.Field{
			{
				Name:     "key",
				JSONName: "key",
				Typez:    genclient.STRING_TYPE,
			},
			{
				Name:     "value",
				JSONName: "value",
				Typez:    genclient.INT64_TYPE,
			},
		},
	}
	map3 := &genclient.Message{
		Name:  "$map<int64, string>",
		ID:    "$map<int64, string>",
		IsMap: true,
		Fields: []*genclient.Field{
			{
				Name:  "key",
				Typez: genclient.INT64_TYPE,
			},
			{
				Name:  "value",
				Typez: genclient.STRING_TYPE,
			},
		},
	}
	map4 := &genclient.Message{
		Name:  "$map<string, bytes>",
		ID:    "$map<string, bytes>",
		IsMap: true,
		Fields: []*genclient.Field{
			{
				Name:  "key",
				Typez: genclient.STRING_TYPE,
			},
			{
				Name:  "value",
				Typez: genclient.BYTES_TYPE,
			},
		},
	}
	message := &genclient.Message{
		Name: "Fake",
		ID:   "..Fake",
		Fields: []*genclient.Field{
			{
				Name:     "target",
				JSONName: "target",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  target.ID,
				Optional: true,
				Repeated: false,
			},
			{
				Name:     "map",
				JSONName: "map",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  map1.ID,
			},
			{
				Name:     "map_i64",
				JSONName: "mapI64",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  map2.ID,
			},
			{
				Name:     "map_i64_key",
				JSONName: "mapI64Key",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  map3.ID,
			},
			{
				Name:     "map_bytes",
				JSONName: "mapBytes",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  map4.ID,
			},
		},
	}
	api := newTestAPI([]*genclient.Message{target, map1, map2, map3, map4, message}, []*genclient.Enum{}, []*genclient.Service{})

	expectedAttributes := map[string]string{
		"target":      ``,
		"map":         `#[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]`,
		"map_i64":     `#[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]` + "\n" + `#[serde_as(as = "std::collections::HashMap<_, serde_with::DisplayFromStr>")]`,
		"map_i64_key": `#[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]` + "\n" + `#[serde_as(as = "std::collections::HashMap<serde_with::DisplayFromStr, _>")]`,
		"map_bytes":   `#[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]` + "\n" + `#[serde_as(as = "std::collections::HashMap<_, serde_with::base64::Base64>")]`,
	}
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)
	for _, field := range message.Fields {
		want, ok := expectedAttributes[field.Name]
		if !ok {
			t.Fatalf("missing expected value for %s", field.Name)
		}
		got := strings.Join(c.FieldAttributes(field, api.State), "\n")
		if got != want {
			t.Errorf("mismatched field type for %s, got=%s, want=%s", field.Name, got, want)
		}
	}
}

func TestRust_WktFieldAttributes(t *testing.T) {
	message := &genclient.Message{
		Name: "Fake",
		ID:   "..Fake",
		Fields: []*genclient.Field{
			{
				Name:     "f_int64",
				JSONName: "fInt64",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  ".google.protobuf.Int64Value",
				Optional: true,
			},
			{
				Name:     "f_uint64",
				JSONName: "fUint64",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  ".google.protobuf.UInt64Value",
				Optional: true,
			},
			{
				Name:     "f_bytes",
				JSONName: "fBytes",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  ".google.protobuf.BytesValue",
				Optional: true,
			},
			{
				Name:     "f_string",
				JSONName: "fString",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  ".google.protobuf.StringValue",
				Optional: true,
			},
		},
	}
	api := newTestAPI([]*genclient.Message{message}, []*genclient.Enum{}, []*genclient.Service{})

	expectedAttributes := map[string]string{
		"f_int64":  `#[serde_as(as = "Option<serde_with::DisplayFromStr>")]`,
		"f_uint64": `#[serde_as(as = "Option<serde_with::DisplayFromStr>")]`,
		"f_bytes":  `#[serde_as(as = "Option<serde_with::base64::Base64>")]`,
		"f_string": ``,
	}
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)
	for _, field := range message.Fields {
		want, ok := expectedAttributes[field.Name]
		if !ok {
			t.Fatalf("missing expected value for %s", field.Name)
		}
		got := strings.Join(c.FieldAttributes(field, api.State), "\n")
		if got != want {
			t.Errorf("mismatched field type for %s, got=%s, want=%s", field.Name, got, want)
		}
	}
}

func TestRust_FieldLossyName(t *testing.T) {
	message := &genclient.Message{
		Name:          "SecretPayload",
		ID:            "..SecretPayload",
		Documentation: "A secret payload resource in the Secret Manager API.",
		Fields: []*genclient.Field{
			{
				Name:          "data",
				JSONName:      "data",
				Documentation: "The secret data. Must be no larger than 64KiB.",
				Typez:         genclient.BYTES_TYPE,
				TypezID:       "bytes",
			},
			{
				Name:          "dataCrc32c",
				JSONName:      "dataCrc32c",
				Documentation: "Optional. If specified, SecretManagerService will verify the integrity of the received data.",
				Typez:         genclient.INT64_TYPE,
				TypezID:       "int64",
				Optional:      true,
			},
		},
	}
	api := newTestAPI([]*genclient.Message{message}, []*genclient.Enum{}, []*genclient.Service{})

	expectedAttributes := map[string]string{
		"data":       `#[serde_as(as = "serde_with::base64::Base64")]`,
		"dataCrc32c": `#[serde(rename = "dataCrc32c")]` + "\n" + `#[serde_as(as = "Option<serde_with::DisplayFromStr>")]`,
	}
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)
	for _, field := range message.Fields {
		want, ok := expectedAttributes[field.Name]
		if !ok {
			t.Fatalf("missing expected value for %s", field.Name)
		}
		got := strings.Join(c.FieldAttributes(field, api.State), "\n")
		if got != want {
			t.Errorf("mismatched field type for %s, got=%s, want=%s", field.Name, got, want)
		}
	}
}

func TestRust_SyntheticField(t *testing.T) {
	message := &genclient.Message{
		Name: "Unused",
		ID:   "..Unused",
		Fields: []*genclient.Field{
			{
				Name:     "updateMask",
				JSONName: "updateMask",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  ".google.protobuf.FieldMask",
				Optional: true,
			},
			{
				Name:      "project",
				JSONName:  "project",
				Typez:     genclient.STRING_TYPE,
				TypezID:   "string",
				Synthetic: true,
			},
			{
				Name:      "data_crc32c",
				JSONName:  "dataCrc32c",
				Typez:     genclient.STRING_TYPE,
				TypezID:   "string",
				Synthetic: true,
			},
		},
	}
	api := newTestAPI([]*genclient.Message{message}, []*genclient.Enum{}, []*genclient.Service{})

	expectedAttributes := map[string]string{
		"updateMask":  ``,
		"project":     `#[serde(skip)]`,
		"data_crc32c": `#[serde(skip)]`,
	}
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)
	for _, field := range message.Fields {
		want, ok := expectedAttributes[field.Name]
		if !ok {
			t.Fatalf("missing expected value for %s", field.Name)
		}
		got := strings.Join(c.FieldAttributes(field, api.State), "\n")
		if got != want {
			t.Errorf("mismatched field type for %s, got=%s, want=%s", field.Name, got, want)
		}
	}
}

func TestRust_FieldType(t *testing.T) {
	target := &genclient.Message{
		Name: "Target",
		ID:   "..Target",
	}
	message := &genclient.Message{
		Name: "Fake",
		ID:   "..Fake",
		Fields: []*genclient.Field{
			{
				Name:     "f_int32",
				Typez:    genclient.INT32_TYPE,
				Optional: false,
				Repeated: false,
			},
			{
				Name:     "f_int32_optional",
				Typez:    genclient.INT32_TYPE,
				Optional: true,
				Repeated: false,
			},
			{
				Name:     "f_int32_repeated",
				Typez:    genclient.INT32_TYPE,
				Optional: false,
				Repeated: true,
			},
			{
				Name:     "f_msg",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  "..Target",
				Optional: true,
				Repeated: false,
			},
			{
				Name:     "f_msg_repeated",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  "..Target",
				Optional: false,
				Repeated: true,
			},
			{
				Name:     "f_timestamp",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  ".google.protobuf.Timestamp",
				Optional: true,
				Repeated: false,
			},
			{
				Name:     "f_timestamp_repeated",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  ".google.protobuf.Timestamp",
				Optional: false,
				Repeated: true,
			},
		},
	}
	api := newTestAPI([]*genclient.Message{target, message}, []*genclient.Enum{}, []*genclient.Service{})

	expectedTypes := map[string]string{
		"f_int32":              "i32",
		"f_int32_optional":     "Option<i32>",
		"f_int32_repeated":     "Vec<i32>",
		"f_msg":                "Option<crate::model::Target>",
		"f_msg_repeated":       "Vec<crate::model::Target>",
		"f_timestamp":          "Option<gax_wkt::Timestamp>",
		"f_timestamp_repeated": "Vec<gax_wkt::Timestamp>",
	}
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)
	for _, field := range message.Fields {
		want, ok := expectedTypes[field.Name]
		if !ok {
			t.Fatalf("missing expected value for %s", field.Name)
		}
		got := c.FieldType(field, api.State)
		if got != want {
			t.Errorf("mismatched field type for %s, got=%s, want=%s", field.Name, got, want)
		}
	}
}

func TestRust_QueryParams(t *testing.T) {
	options := &genclient.Message{
		Name:   "Options",
		ID:     "..Options",
		Fields: []*genclient.Field{},
	}
	optionsField := &genclient.Field{
		Name:     "options_field",
		JSONName: "optionsField",
		Typez:    genclient.MESSAGE_TYPE,
		TypezID:  options.ID,
	}
	anotherField := &genclient.Field{
		Name:     "another_field",
		JSONName: "anotherField",
		Typez:    genclient.STRING_TYPE,
		TypezID:  options.ID,
	}
	request := &genclient.Message{
		Name: "TestRequest",
		ID:   "..TestRequest",
		Fields: []*genclient.Field{
			optionsField, anotherField,
			{
				Name: "unused",
			},
		},
	}
	method := &genclient.Method{
		Name:         "Test",
		ID:           "..TestService.Test",
		InputTypeID:  request.ID,
		OutputTypeID: ".google.protobuf.Empty",
		PathInfo: &genclient.PathInfo{
			Verb: "GET",
			QueryParameters: map[string]bool{
				"options_field": true,
				"another_field": true,
			},
		},
	}
	api := newTestAPI(
		[]*genclient.Message{options, request},
		[]*genclient.Enum{},
		[]*genclient.Service{
			{
				Name:    "TestService",
				ID:      "..TestService",
				Methods: []*genclient.Method{method},
			},
		})
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)

	got := c.QueryParams(method, api.State)
	want := []*genclient.Field{optionsField, anotherField}
	less := func(a, b *genclient.Field) bool { return a.Name < b.Name }
	if diff := cmp.Diff(want, got, cmpopts.SortSlices(less)); diff != "" {
		t.Errorf("mismatched query parameters (-want, +got):\n%s", diff)
	}
}

func TestRust_AsQueryParameter(t *testing.T) {
	options := &genclient.Message{
		Name:   "Options",
		ID:     "..Options",
		Fields: []*genclient.Field{},
	}
	optionsField := &genclient.Field{
		Name:     "options_field",
		JSONName: "optionsField",
		Typez:    genclient.MESSAGE_TYPE,
		TypezID:  options.ID,
	}
	anotherField := &genclient.Field{
		Name:     "another_field",
		JSONName: "anotherField",
		Typez:    genclient.STRING_TYPE,
		TypezID:  options.ID,
	}
	request := &genclient.Message{
		Name:   "TestRequest",
		ID:     "..TestRequest",
		Fields: []*genclient.Field{optionsField, anotherField},
	}
	api := newTestAPI(
		[]*genclient.Message{options, request},
		[]*genclient.Enum{},
		[]*genclient.Service{})
	c := createRustCodec()
	c.LoadWellKnownTypes(api.State)

	want := "&serde_json::to_value(&req.options_field).map_err(Error::serde)?"
	got := c.AsQueryParameter(optionsField, api.State)
	if want != got {
		t.Errorf("mismatched as query parameter for options_field, want=%s, got=%s", want, got)
	}

	want = "&req.another_field"
	got = c.AsQueryParameter(anotherField, api.State)
	if want != got {
		t.Errorf("mismatched as query parameter for another_field, want=%s, got=%s", want, got)
	}

}

type rustCaseConvertTest struct {
	Input    string
	Expected string
}

func TestRust_ToSnake(t *testing.T) {
	c := &RustCodec{}
	var snakeConvertTests = []rustCaseConvertTest{
		{"FooBar", "foo_bar"},
		{"foo_bar", "foo_bar"},
		{"data_crc32c", "data_crc32c"},
		{"True", "r#true"},
		{"Static", "r#static"},
		{"Trait", "r#trait"},
		{"Self", "r#self"},
		{"self", "r#self"},
		{"yield", "r#yield"},
	}
	for _, test := range snakeConvertTests {
		if output := c.ToSnake(test.Input); output != test.Expected {
			t.Errorf("Output %q not equal to expected %q, input=%s", output, test.Expected, test.Input)
		}
	}
}

func TestRust_ToPascal(t *testing.T) {
	c := &RustCodec{}
	var pascalConvertTests = []rustCaseConvertTest{
		{"foo_bar", "FooBar"},
		{"FooBar", "FooBar"},
		{"True", "True"},
		{"Self", "r#Self"},
		{"self", "r#Self"},
		{"yield", "Yield"},
	}
	for _, test := range pascalConvertTests {
		if output := c.ToPascal(test.Input); output != test.Expected {
			t.Errorf("Output %q not equal to expected %q", output, test.Expected)
		}
	}
}

func TestRust_FormatDocComments(t *testing.T) {
	input := `Some comments describing the thing.

The next line has some extra trailing whitespace:` + "   " + `

We want to respect whitespace at the beginning, because it important in Markdown:
- A thing
  - A nested thing
- The next thing

Now for some fun with block quotes

` + "```" + `
Maybe they wanted to show some JSON:
{
  "foo": "bar"
}
` + "```"

	want := []string{
		"/// Some comments describing the thing.",
		"///",
		"/// The next line has some extra trailing whitespace:",
		"///",
		"/// We want to respect whitespace at the beginning, because it important in Markdown:",
		"/// - A thing",
		"///   - A nested thing",
		"/// - The next thing",
		"///",
		"/// Now for some fun with block quotes",
		"///",
		"/// ```norust",
		"/// Maybe they wanted to show some JSON:",
		"/// {",
		`///   "foo": "bar"`,
		"/// }",
		"/// ```",
	}

	c := &RustCodec{}
	got := c.FormatDocComments(input)
	if diff := cmp.Diff(want, got); diff != "" {
		t.Errorf("mismatch in FormatDocComments (-want, +got)\n:%s", diff)
	}
}

func TestRust_FormatDocCommentsBullets(t *testing.T) {
	input := `In this example, in proto field could take one of the following values:

* full_name for a violation in the full_name value
* email_addresses[1].email for a violation in the email field of the
  first email_addresses message
* email_addresses[3].type[2] for a violation in the second type
  value in the third email_addresses message.)`
	want := []string{
		"/// In this example, in proto field could take one of the following values:",
		"///",
		"/// * full_name for a violation in the full_name value",
		"/// * email_addresses[1].email for a violation in the email field of the",
		"///   first email_addresses message",
		"/// * email_addresses[3].type[2] for a violation in the second type",
		"///   value in the third email_addresses message.)",
	}

	c := createRustCodec()
	got := c.FormatDocComments(input)
	if diff := cmp.Diff(want, got); diff != "" {
		t.Errorf("mismatch in FormatDocComments (-want, +got)\n:%s", diff)
	}
}

func TestRust_MessageNames(t *testing.T) {
	message := &genclient.Message{
		Name: "Replication",
		ID:   "..Replication",
		Fields: []*genclient.Field{
			{
				Name:     "automatic",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  "..Automatic",
				Optional: true,
				Repeated: false,
			},
		},
	}
	nested := &genclient.Message{
		Name:   "Automatic",
		ID:     "..Replication.Automatic",
		Parent: message,
	}

	api := newTestAPI([]*genclient.Message{message, nested}, []*genclient.Enum{}, []*genclient.Service{})

	c := createRustCodec()
	if got := c.MessageName(message, api.State); got != "Replication" {
		t.Errorf("mismatched message name, got=%s, want=Replication", got)
	}
	if got := c.FQMessageName(message, api.State); got != "crate::model::Replication" {
		t.Errorf("mismatched message name, got=%s, want=crate::model::Replication", got)
	}

	if got := c.MessageName(nested, api.State); got != "Automatic" {
		t.Errorf("mismatched message name, got=%s, want=Automatic", got)
	}
	if got := c.FQMessageName(nested, api.State); got != "crate::model::replication::Automatic" {
		t.Errorf("mismatched message name, got=%s, want=crate::model::replication::Automatic", got)
	}
}

func TestRust_EnumNames(t *testing.T) {
	message := &genclient.Message{
		Name: "SecretVersion",
		ID:   "..SecretVersion",
		Fields: []*genclient.Field{
			{
				Name:     "automatic",
				Typez:    genclient.MESSAGE_TYPE,
				TypezID:  "..Automatic",
				Optional: true,
				Repeated: false,
			},
		},
	}
	nested := &genclient.Enum{
		Name:   "State",
		ID:     "..SecretVersion.State",
		Parent: message,
	}

	api := newTestAPI([]*genclient.Message{message}, []*genclient.Enum{nested}, []*genclient.Service{})

	c := createRustCodec()
	if got := c.EnumName(nested, api.State); got != "State" {
		t.Errorf("mismatched message name, got=%s, want=Automatic", got)
	}
	if got := c.FQEnumName(nested, api.State); got != "crate::model::secret_version::State" {
		t.Errorf("mismatched message name, got=%s, want=crate::model::secret_version::State", got)
	}
}