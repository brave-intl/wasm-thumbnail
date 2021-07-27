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

      WASMStore = Wasmer::Store.new
      WASMImportObject = Wasmer::ImportObject.new
      WASMImportObject.register(
        "env",
        register_panic: Wasmer::Function.new(
          WASMStore,
          :register_panic,
          Wasmer::FunctionType.new([Wasmer::Type::I32,
                                    Wasmer::Type::I32,
                                    Wasmer::Type::I32,
                                    Wasmer::Type::I32,
                                    Wasmer::Type::I32,
                                    Wasmer::Type::I32], [])
        )
      )

      # Return an instance so you don't have to constantly compile
      class GetWasmInstance
        def self.call
          # Let's compile the module to be able to execute it!
          Wasmer::Instance.new(
            Wasmer::Module.new(WASMStore, IO.read("#{__dir__}/rb/data/wasm_thumbnail.wasm", mode: "rb")),
            WASMImportObject
          )
        end
      end
    end
  end
end
