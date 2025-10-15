package main

/*
#cgo LDFLAGS: -L../target/release -ltransport_api_wrapper
#include "../cpp/transport_api_wrapper.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	query := C.CString("berlin")
	defer C.free(unsafe.Pointer(query))

	result := C.locations(query)
	defer C.free_string(result)

	fmt.Println(C.GoString(result))
}