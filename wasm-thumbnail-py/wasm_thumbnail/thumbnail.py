#!/usr/bin/env python

import importlib
from importlib import resources
import struct

from wasmer import engine, Store, Module, Instance, ImportObject, Function, FunctionType, Type
from wasmer_compiler_cranelift import Compiler

def get_unpadded_length(data):
    """Get the unpadded length from the header."""
    data_length_without_header = len(data) - 4
    if data_length_without_header < 0:
        raise ValueError('Data must be at least 4 bytes long', len(data))

    return struct.unpack('!L', data[0:4])[0]

def decode_padded_image(data):
    """Extract a payload from its padding by reading its length header."""
    payload_length = get_unpadded_length(data)

    if data_length_without_header < payload_length:
        raise ValueError('Payload is shorter than the expected length',
                         data_length_without_header, payload_length)

    return data[4:4 + payload_length]

def resize_and_pad_image(image_bytes, width, height, size, quality = 80):
    """Resize an image and pad to fit size, output is prefixed by image length without padding.
    Throws an error if the resized image does not fit in size or is not a supported format"""
    instance = Instance(module)

    image_length = len(image_bytes)
    input_pointer = instance.exports.allocate(image_length)
    memory = instance.exports.memory.uint8_view(input_pointer)
    memory[0:image_length] = image_bytes

    output_pointer = instance.exports.resize_and_pad(input_pointer, image_length, width, height, size, quality)
    instance.exports.deallocate(input_pointer, image_length)

    memory = instance.exports.memory.uint8_view(output_pointer)
    out_bytes = bytes(memory[:size])
    instance.exports.deallocate(output_pointer, size)

    unpadded_length = get_unpadded_length(out_bytes)
    if unpadded_length == 0:
        raise RuntimeError('Image resizing failed')

    return out_bytes

def register_panic(msg_ptr: int, msg_len: int, file_ptr: int, file_len: int, line: int, column: int):
    """Panic handler to be called from WASM for debugging purposes"""
    msg = bytes(instance.exports.memory.uint8_view(msg_ptr)[:msg_len]).decode("utf-8")
    file = bytes(instance.exports.memory.uint8_view(file_ptr)[:file_len]).decode("utf-8")
    print("wasm panicked at '{}', {}:{}:{}".format(msg, file, line, column))

wasm = resources.open_binary('wasm_thumbnail.data', "wasm_thumbnail.wasm")
store = Store(engine.JIT(Compiler))
module = Module(store, wasm.read())
# import_object = ImportObject()
# import_object.register(
#     "env",
#     {
#         "register_panic": Function(store, register_panic)
#     }
# )
