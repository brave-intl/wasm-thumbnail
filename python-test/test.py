#!/usr/bin/env python

import time
from PIL import Image

from wasmer import engine, Store, Module, Instance
from wasmer_compiler_cranelift import Compiler

path = 'wasm_thumbnail.wasm'
store = Store(engine.JIT(Compiler))
module = Module(store, open(path, 'rb').read())

def resize_and_pad_image(image_bytes, width, height, size):
    instance = Instance(module)

    image_length = len(image_bytes)
    input_pointer = instance.exports.allocate(image_length)
    memory = instance.exports.memory.uint8_view(input_pointer)
    memory[0:image_length] = image_bytes

    output_pointer = instance.exports.resize_and_pad(input_pointer, image_length, width, height, size)

    memory = instance.exports.memory.uint8_view(output_pointer)
    out_bytes = bytes(memory[:size])

    return out_bytes

with open('brave.png', 'rb') as image:
    image_bytes = image.read()

    tic = time.perf_counter()

    out_bytes = resize_and_pad_image(image_bytes, 100, 100, 250000)
    with open('out.jpg', 'wb+') as out_image:
        out_image.write(out_bytes)

    toc = time.perf_counter()
    print(f"Resized brave.png with WASM in {toc - tic:0.4f} seconds")

tic = time.perf_counter()

try:
    m = Image.open("brave.png").convert('RGB')
    m.thumbnail((100, 100),Image.ANTIALIAS)
except:
    print("unknown image.")

toc = time.perf_counter()
print(f"Resized brave.png with PIL in {toc - tic:0.4f} seconds")
