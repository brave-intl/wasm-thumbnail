# frozen_string_literal: true

require_relative "lib/wasm/thumbnail/rb/version"

Gem::Specification.new do |spec|
  spec.name          = "wasm-thumbnail-rb"
  spec.version       = Wasm::Thumbnail::Rb::VERSION
  spec.authors       = ["Tyler Smart"]
  spec.email         = ["tsmart@brave.com"]
  spec.licenses      = ["MPL-2.0"]
  spec.summary       = "WASM based thumbnail library"
  spec.homepage      = "https://github.com/brave-intl/wasm-thumbnail"
  spec.required_ruby_version = Gem::Requirement.new(">= 2.7.0")

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/brave-intl/wasm-thumbnail"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{\A(?:test|spec|features)/}) }
  end
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_dependency "wasmer", "~> 1.0"

  spec.add_development_dependency "pry"

end
