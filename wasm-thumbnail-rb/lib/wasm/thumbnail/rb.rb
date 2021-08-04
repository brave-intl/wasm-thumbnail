# frozen_string_literal: true

require_relative "rb/version"
require "wasmer"
module Wasm
  module Thumbnail
    # As opposed to the PY implementation
    module Rb
      class Error < StandardError; end

      # Now the module is compiled, we can instantiate it. Doing so outside the method where used results in errors.
      def self.register_panic(_msg_ptr = nil, _msg_len = nil, _file_ptr = nil, _file_len = nil, _line = nil, _column = nil)
        puts("WASM panicked")
      end

      def self.resize_and_pad_with_header(file_bytes:, width:, height:, size:, quality: 80)
        # Let's compile the module to be able to execute it!
        wasm_instance = Wasm::Thumbnail::Rb::GetWasmInstance.call

        # This tells us how much space we'll need to put our image in the WASM env
        image_length = file_bytes.length
        input_pointer = wasm_instance.exports.allocate.call(image_length)
        # Get a pointer on the allocated memory so we can write to it
        memory = wasm_instance.exports.memory.uint8_view input_pointer

        # Put the image to resize in the allocated space
        (0..image_length - 1).each do |nth|
          memory[nth] = file_bytes[nth]
        end

        # Do the actual resize and pad
        # Note that this writes to a portion of memory the new JPEG file, but right pads the rest of the space
        # we gave it with 0.
        begin
          output_pointer = wasm_instance.exports.resize_and_pad.call(input_pointer,
                                                                     image_length,
                                                                     width,
                                                                     height,
                                                                     size,
                                                                     quality)
        rescue RuntimeError
          raise "Error processing the image."
        end
        # Get a pointer to the result
        memory = wasm_instance.exports.memory.uint8_view output_pointer

        # Only take the buffer that we told the rust function we needed. The resize function
        # makes a smaller image than the buffer we said, and then pads out the rest.
        bytes = memory.to_a.take(size)

        # Deallocate
        wasm_instance.exports.deallocate.call(input_pointer, image_length)
        wasm_instance.exports.deallocate.call(output_pointer, bytes.length)

        bytes
      end

      def self.resize_and_pad(file_bytes:, width:, height:, size:, quality: 80)
        bytes = resize_and_pad_with_header(file_bytes: file_bytes,
                                           width: width,
                                           height: height,
                                           size: size + 4,
                                           quality: quality)

        # The first 4 bytes are a header until the image. The actual image probably ends well before
        # the whole buffer, but we keep the junk data on the end to make all the images the same size
        # for privacy concerns.
        bytes[4..].pack("C*")
      end

      # Return an instance so you don't have to constantly compile
      class GetWasmInstance
        def self.call
          store = Wasmer::Store.new
          import_object = Wasmer::ImportObject.new
          import_object.register(
            "env",
            register_panic: Wasmer::Function.new(
              store,
              ->(*args) { Wasm::Thumbnail::Rb.register_panic(*args) },
              Wasmer::FunctionType.new([Wasmer::Type::I32,
                                        Wasmer::Type::I32,
                                        Wasmer::Type::I32,
                                        Wasmer::Type::I32,
                                        Wasmer::Type::I32,
                                        Wasmer::Type::I32], [])
            )
          )

          # Let's compile the module to be able to execute it!
          Wasmer::Instance.new(
            Wasmer::Module.new(store, IO.read("#{__dir__}/rb/data/wasm_thumbnail.wasm", mode: "rb")),
            import_object
          )
        end
      end
    end
  end
end
