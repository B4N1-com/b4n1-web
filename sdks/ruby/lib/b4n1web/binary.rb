# frozen_string_literal: true

require "rbconfig"
require_relative "errors"

module B4n1Web
  # Resolves the b4n1web binary. Order mirrors the Python/JS/C#/Java SDKs:
  #   1. B4N1WEB_BIN_PATH (explicit override, must exist + be executable)
  #   2. bundled binary shipped inside the gem (lib/b4n1web/bin/*)
  #   well-known user/system locations
  #   4. PATH scan
  module Binary
    PLATFORM_BINARIES = {
      ["linux", "x86_64"]  => "b4n1web-linux-amd64",
      ["linux", "aarch64"] => "b4n1web-linux-arm64",
      ["linux", "i686"]    => "b4n1web-linux-ia32",
      ["darwin", "x86_64"] => "b4n1web-macos-x64",
      ["darwin", "arm64"]  => "b4n1web-macos-arm64",
      ["mingw", "x64"]     => "b4n1web-windows-amd64.exe",
      ["mswin", "x64"]     => "b4n1web-windows-amd64.exe"
    }.freeze

    module_function

    def platform_binary_name
      os = RbConfig::CONFIG["host_os"]
      arch = RbConfig::CONFIG["host_cpu"]
      arch = "x86_64" if arch == "amd64"
      arch = "i686" if arch == "i386"
      plat = case os
             when /linux/ then "linux"
             when /darwin|mac os/ then "darwin"
             when /mingw|cygwin/ then "mingw"
             when /mswin|windows/ then "mswin"
             end
      return nil unless plat
      PLATFORM_BINARIES[[plat, arch]]
    end

    # Returns the absolute path of a runnable b4n1web binary or nil.
    def resolve
      env = ENV["B4N1WEB_BIN_PATH"]
      return env if env && File.file?(env) && File.executable?(env)

      name = platform_binary_name
      if name
        bundled = File.expand_path(File.join(__dir__, "bin", name))
        return bundled if File.file?(bundled) && File.executable?(bundled)
      end

      candidates = []
      candidates << ENV["B4N1WEB_BINARY"] if ENV["B4N1WEB_BINARY"]
      home = Dir.home
      candidates << File.join(home, ".local", "bin", "b4n1web")
      candidates << File.join(home, ".b4n1web", "bin", "b4n1web")
      candidates << "/usr/local/bin/b4n1web"
      candidates << "/usr/bin/b4n1web"
      ENV.fetch("PATH", "").split(File::PATH_SEPARATOR).each do |dir|
        candidates << File.join(dir, "b4n1web")
      end

      candidates.compact.find { |p| File.file?(p) && File.executable?(p) }
    end

    # Same as resolve but raises loudly (no silent nil).
    def resolve!
      resolve || raise(BinaryNotFoundError,
                       "b4n1web binary not found. Install it (curl -sL " \
                       "https://raw.githubusercontent.com/B4N1-com/b4n1-web/" \
                       "master/scripts/install.sh | bash) or set B4N1WEB_BIN_PATH.")
    end
  end
end
