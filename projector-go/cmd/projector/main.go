package main

import (
	"fmt"
	"log"

	"github.com/chamatt/ts-node-rust/projector/pkg/projector"
)

func main() {
	opts, err := projector.GetOpts()

	if err != nil {
		log.Fatalf("unable to get opts: %v", err)
	}
	fmt.Printf("opts: %+v\n", opts)

}
