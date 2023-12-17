#!/usr/bin/env python

import os
import time
import sys
from io import BytesIO

import requests
sys.path.append('../')

from wasm_thumbnail import *

USER_AGENT = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"


# IMAGE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "Check.gif")

def get_with_max_size(url, max_bytes=1000000):
    is_large = False
    response = requests.get(url, stream=True, timeout=10, headers={'User-Agent': USER_AGENT})
    response.raise_for_status()
    if response.headers.get('Content-Length') and int(response.headers.get('Content-Length')) > max_bytes:
        is_large = True
    count = 0
    content = BytesIO()
    for chunk in response.iter_content(4096):
        count += len(chunk)
        content.write(chunk)
        if count > max_bytes:
            is_large = True

    return content.getvalue(), is_large

# with open(IMAGE, 'rb') as image:
# image_bytes = image.read()

tic = time.perf_counter()

# url = "https://www.washingtonpost.com/favicon.svg"
url = "https://www.usnews.com/favicon.ico"

content, is_large = get_with_max_size(url)  # 5mb max

out_bytes = resize_and_pad_image(content, 1168, 657, 250000)
with open('out.jpg', 'wb+') as out_image:
    out_image.write(out_bytes)

toc = time.perf_counter()
print(f"Resized {url} with WASM in {toc - tic:0.4f} seconds")
