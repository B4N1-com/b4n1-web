# frozen_string_literal: true

require_relative "b4n1web/version"
require_relative "b4n1web/errors"
require_relative "b4n1web/binary"
require_relative "b4n1web/page"
require_relative "b4n1web/agent_browser"

# B4n1Web Ruby SDK — ultra-lightweight agentic browser for AI agents.
#
#   require "b4n1web"
#   browser = B4n1Web::AgentBrowser.new
#   page = browser.goto("https://example.com")
#   puts page.markdown
#   browser.close
module B4n1Web
end
