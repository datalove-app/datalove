# FAQ

## Can you add support for `x`?

No, not unless it is implemented by the [Multicodec](https://github.com/multiformats/multicodec) standard but missing from this library.

## How do I update the list of encodings?

First, check [https://github.com/multiformats/multicodec] to see the status of Multicodec and if there are any related issues. Next, check the [table](https://github.com/multiformats/multicodec/blob/master/table.csv) to see if it is there.

This library ships with a module that can help you generate the map that is used at compile time to make maintaining the list of encodings bearable.

## I like Elixir atoms. Why did you not use atoms?

Multicodec is growing and already has several hundred codecs. It does not seem like a good idea to burden consumers of this library with this burden. More specifically, BEAM has a default limit for the number of atoms in the VM at once. It can be increased, but this hardly seems like something we should force anyone to do just decode some stuff on the wire.

The other major related reason is we are parsing data from an external source to generate the list of mappings. I am never comfortable serializing such data straight into atoms without at least some audit. At compile time there are some minimal checks already, but there need to be more.

If this is a real issue or you see it otherwise, please open an issue to discuss. I love atoms too.