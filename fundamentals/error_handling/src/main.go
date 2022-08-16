package main

import "fmt"

func returnsError(value int) error {
	return fmt.Errorf("this is an error with value %v", value)
}

type Foo struct {
	num int32
}

func (f *Foo) thisIsOnFoo(value int) error {
	return fmt.Errorf("this is an error with value %v", value)
}

func CreateFoo(fail bool, value int32) (*Foo, error) {
	if fail {
		return nil, fmt.Errorf("this is an error with value")
	}
	return &Foo{num: value}, nil
}

func main() {

	foo, err := CreateFoo(true, 66)
	if err != nil {
		fmt.Println("there was an error")
		return
	}

	fmt.Println(foo.num)

}
