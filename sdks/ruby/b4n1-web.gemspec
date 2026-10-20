# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name          = "b4n1-web"
  spec.version       = File.read(File.expand_path("lib/b4n1web/version.rb", __dir__))[/VERSION = "(.+)"/, 1]
  spec.authors       = ["Bani Montoya"]
  spec.email         = ["b4n1@b4n1.com"]

  spec.summary       = "B4n1Web - Agentic Browser Engine for AI agents"
  spec.description   = "Ultra-lightweight headless browser with bundled binary for AI agents. Supports Light/JS/Render modes."
  spec.homepage      = "https://b4n1.com"
  spec.license       = "BUSL-1.1"

  spec.required_ruby_version = ">= 3.0"

  spec.files = Dir.glob("lib/**/*.rb") + Dir.glob("bin/*") +
               %w[README.md LICENSE EULA.txt]
  spec.bindir        = "bin"
  spec.executables   = ["b4n1web"]
  spec.require_paths = ["lib"]

  spec.metadata = {
    "homepage_uri"          => "https://b4n1.com",
    "source_code_uri"       => "https://github.com/B4N1-com/b4n1-web",
    "changelog_uri"         => "https://github.com/B4N1-com/b4n1-web/blob/master/CHANGELOG.md",
    "bug_tracker_uri"       => "https://github.com/B4N1-com/b4n1-web/issues"
  }
end
