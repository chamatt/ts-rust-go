enum TSEnum {
  Foo,
  Bar,
  Baz,
}

type Foo = {
  bar?: string;
};

function doSomething(foo: Foo): boolean {
  if (foo.bar) {
    return true;
  }
  return false;
}

doSomething({});
