# frozen_string_literal: true

require "json"

module B4n1Web
  # Structured page data returned by the b4n1web binary.
  class Page
    attr_reader :url, :markdown, :links, :screenshot

    def initialize(url:, markdown:, links:, screenshot: nil)
      @url = url
      @markdown = markdown
      @links = links
      @screenshot = screenshot
    end

    # Parses the binary's text protocol:
    #   URL: <url>
    #   Markdown:
    #   <free-form markdown lines...>
    #   Links: ["https://...", ...]
    #   Screenshot: <path>            (optional)
    def self.parse(url, output)
      markdown = +""
      links = []
      screenshot = nil
      in_markdown = false

      output.to_s.each_line do |line|
        line = line.chomp
        if line.start_with?("URL:")
          next
        elsif line.start_with?("Markdown:")
          in_markdown = true
          rest = line.delete_prefix("Markdown:").strip
          markdown << "#{rest}\n" unless rest.empty?
        elsif line.start_with?("Links:")
          in_markdown = false
          links = JSON.parse(line.delete_prefix("Links:").strip) || []
        elsif line.start_with?("Screenshot:")
          in_markdown = false
          shot = line.delete_prefix("Screenshot:").strip
          screenshot = shot.empty? ? nil : shot
        else
          markdown << "#{line}\n" if in_markdown
        end
      end

      new(url: url, markdown: markdown.strip, links: links, screenshot: screenshot)
    end

    # Main content heuristics: drop the first two lines (title + empty line).
    def get_main_content
      lines = markdown.split("\n")
      content = lines.length > 2 ? lines[2..] : lines
      content.join("\n").strip
    end

    def find_links_by_text(text)
      return [] if text.nil? || text.empty?
      needle = text.downcase
      links.select { |link| link.downcase.include?(needle) }
    end
  end
end
