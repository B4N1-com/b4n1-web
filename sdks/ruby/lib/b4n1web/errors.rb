# frozen_string_literal: true

module B4n1Web
  class Error < StandardError; end

  # Raised when no b4n1web binary can be resolved (bundled/system/PATH).
  class BinaryNotFoundError < Error; end

  # Raised when the binary exits non-zero or times out.
  class NavigationError < Error; end
end
