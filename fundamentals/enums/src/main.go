package main

type GoEnum = int

const (
	Foo GoEnum = iota
	Bar
	Baz
)

func main() {
	println(Foo)
	println(Bar)
	println(Baz)
}
