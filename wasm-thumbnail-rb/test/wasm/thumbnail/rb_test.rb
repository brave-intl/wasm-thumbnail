# frozen_string_literal: true

require "test_helper"

class Wasm::Thumbnail::RbTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Wasm::Thumbnail::Rb::VERSION
  end

  def test_it_resizes_with_custom_quality
    file_bytes = File.binread("#{__dir__}/brave.png").unpack("C*")
    image = Wasm::Thumbnail::Rb.resize_and_pad(file_bytes: file_bytes,
                                               width: 100,
                                               height: 200,
                                               size: 250_000,
                                               quality: 40)
    puts "Image resized and padded to size #{image.length}"
  end

  def test_it_resizes_with_default_quality
    file_bytes = File.binread("#{__dir__}/brave.png").unpack("C*")
    image = Wasm::Thumbnail::Rb.resize_and_pad(file_bytes: file_bytes,
                                               width: 100,
                                               height: 200,
                                               size: 250_000)
    puts "Image resized and padded to size #{image.length}"
  end

  def test_it_calls_panic_if_image_too_big
    file_bytes = File.binread("#{__dir__}/brave.png").unpack("C*")
    exception = assert_raises RuntimeError do
      Wasm::Thumbnail::Rb.resize_and_pad(file_bytes: file_bytes,
                                         width: 100,
                                         height: 200,
                                         size: 5)
    end
    assert_equal(exception.message.split(/:/, 2).first, "Error processing the image")
  end

  def test_should_not_crash_after_many_resizes
    # Photo from https://unsplash.com/photos/SRFhLQ_zmak
    file_bytes = File.binread("#{__dir__}/shipwreck.jpg").unpack("C*")
    quality_to_try = 100
    # Run a few times. Adjust as needed, checking memory usage as well
    while quality_to_try > 96
      begin
        return Wasm::Thumbnail::Rb.resize_and_pad(
          file_bytes: file_bytes,
          width: 2700,
          height: 528,
          size: 5,
          quality: quality_to_try
        )
      rescue RuntimeError => e
        quality_to_try -= 1
        puts "Trying quality #{quality_to_try}"
        next
      end
    end
  end
end
