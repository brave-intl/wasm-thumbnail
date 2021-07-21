#!/usr/bin/env python

import os
import time

from wasm_thumbnail import *


IMAGE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "brave.png")

with open(IMAGE, 'rb') as image:
    image_bytes = image.read()

    tic = time.perf_counter()

    out_bytes = resize_and_pad_image(image_bytes, 100, 100, 240000)
    with open('out.jpg', 'wb+') as out_image:
        out_image.write(out_bytes)

    toc = time.perf_counter()
    print(f"Resized brave.png with WASM in {toc - tic:0.4f} seconds")
