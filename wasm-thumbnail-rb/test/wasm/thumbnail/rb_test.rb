# frozen_string_literal: true

require "test_helper"

class Wasm::Thumbnail::RbTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Wasm::Thumbnail::Rb::VERSION
  end

  def test_it_does_something_useful
    file_bytes = File.binread("#{__dir__}/brave.png").unpack("C*")
    image = Wasm::Thumbnail::Rb.resize_and_pad(file_bytes: file_bytes,
                                               width: 100,
                                               height: 200,
                                               size: 250_000)
    puts "Image resized and padded to size #{image.length}"
  end
end
