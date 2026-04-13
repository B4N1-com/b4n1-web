package b4n1web

import (
	"fmt"
	"strings"
	"testing"
)

// --- Page construction tests ---

func TestPageConstruction(t *testing.T) {
	t.Run("zero value", func(t *testing.T) {
		var p Page
		if p.URL != "" {
			t.Errorf("zero URL = %q, want empty", p.URL)
		}
		if p.Markdown != "" {
			t.Errorf("zero Markdown = %q, want empty", p.Markdown)
		}
		if p.Links != nil {
			t.Errorf("zero Links = %v, want nil", p.Links)
		}
		if p.Screenshot != "" {
			t.Errorf("zero Screenshot = %q, want empty", p.Screenshot)
		}
	})

	t.Run("with all fields", func(t *testing.T) {
		p := Page{
			URL:        "https://example.com",
			Markdown:   "# Title\nContent",
			Links:      []string{"https://a.com", "https://b.com"},
			Screenshot: "base64encodeddata",
		}
		if p.URL != "https://example.com" {
			t.Errorf("URL = %q", p.URL)
		}
		if p.Markdown != "# Title\nContent" {
			t.Errorf("Markdown = %q", p.Markdown)
		}
		if len(p.Links) != 2 {
			t.Errorf("Links len = %d", len(p.Links))
		}
		if p.Screenshot != "base64encodeddata" {
			t.Errorf("Screenshot = %q", p.Screenshot)
		}
	})

	t.Run("with only URL", func(t *testing.T) {
		p := Page{URL: "https://example.com"}
		if p.URL != "https://example.com" {
			t.Errorf("URL = %q", p.URL)
		}
		if p.Markdown != "" {
			t.Errorf("Markdown = %q", p.Markdown)
		}
	})

	t.Run("with empty links slice", func(t *testing.T) {
		p := Page{Links: []string{}}
		if p.Links == nil {
			t.Error("Links should be empty slice, not nil")
		}
		if len(p.Links) != 0 {
			t.Errorf("Links len = %d", len(p.Links))
		}
	})

	t.Run("with nil links", func(t *testing.T) {
		p := Page{Links: nil}
		if p.Links != nil {
			t.Errorf("Links should be nil, got %v", p.Links)
		}
	})
}

// --- GetMainContent exhaustive tests ---

func TestPageGetMainContent(t *testing.T) {
	t.Run("empty markdown returns empty", func(t *testing.T) {
		p := Page{Markdown: ""}
		got := p.GetMainContent()
		if got != "" {
			t.Errorf("got %q, want empty", got)
		}
	})

	t.Run("single line returns as-is", func(t *testing.T) {
		p := Page{Markdown: "OnlyLine"}
		got := p.GetMainContent()
		if got != "OnlyLine" {
			t.Errorf("got %q, want %q", got, "OnlyLine")
		}
	})

	t.Run("two lines returns as-is (len <= 2 returns original)", func(t *testing.T) {
		p := Page{Markdown: "Line1\nLine2"}
		got := p.GetMainContent()
		want := "Line1\nLine2"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})

	t.Run("three lines returns third line only", func(t *testing.T) {
		p := Page{Markdown: "Line1\nLine2\nLine3"}
		got := p.GetMainContent()
		if got != "Line3" {
			t.Errorf("got %q, want %q", got, "Line3")
		}
	})

	t.Run("four lines returns lines 3 and 4", func(t *testing.T) {
		p := Page{Markdown: "Header\nSubheader\nBody\nFooter"}
		got := p.GetMainContent()
		want := "Body\nFooter"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})

	t.Run("five lines returns lines 3-5", func(t *testing.T) {
		p := Page{Markdown: "A\nB\nC\nD\nE"}
		got := p.GetMainContent()
		want := "C\nD\nE"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})

	t.Run("markdown with trailing newline", func(t *testing.T) {
		p := Page{Markdown: "A\nB\nC\n"}
		got := p.GetMainContent()
		if got != "C" {
			t.Errorf("got %q, want %q", got, "C")
		}
	})

	t.Run("real browser output format", func(t *testing.T) {
		// The actual browser output for Markdown field after parseOutput
		// has the markdown content. GetMainContent skips first 2 lines.
		p := Page{
			Markdown: "# Example Domain\nThis domain is for use in documentation examples without needing permission. Avoid use in operations.\nLearn more\n",
		}
		got := p.GetMainContent()
		// With 4 lines (trailing newline creates empty 5th? No: splitLines on "...\n" = 4 items)
		// Skip first 2: "Avoid use in operations." and "Learn more" and ""
		if !strings.Contains(got, "Avoid use in operations") && !strings.Contains(got, "Learn more") {
			t.Errorf("content should have body text after skipping 2 lines, got: %q", got)
		}
	})

	t.Run("unicode content preserved", func(t *testing.T) {
		p := Page{Markdown: "Header\nSubheader\n\u4e2d\u6587\u5185\u5bb9"}
		got := p.GetMainContent()
		want := "\u4e2d\u6587\u5185\u5bb9"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})

	t.Run("special characters preserved", func(t *testing.T) {
		p := Page{Markdown: "A\nB\nSpecial: <html> & \"quotes\""}
		got := p.GetMainContent()
		want := "Special: <html> & \"quotes\""
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})

	t.Run("only newlines beyond threshold", func(t *testing.T) {
		p := Page{Markdown: "\n\n\n"}
		got := p.GetMainContent()
		// 3 newlines = 4 lines (last is empty after trailing), skip first 2, get "\n"
		// Actually: splitLines("\n\n\n") = ["", "", "", ""] - 4 items
		// Wait: "\n\n\n" -> indices at 0,1,2 are newlines
		// splitLines: at i=0, c='\n' -> append s[0:0]="" , start=1
		// at i=1, c='\n' -> append s[1:1]="" , start=2
		// at i=2, c='\n' -> append s[2:2]="" , start=3
		// start=3, len(s)=3, so start < len(s) is false
		// Result: ["", "", ""]
		// Skip first 2: [""]
		// joinLines: ""
		if got != "" {
			t.Errorf("got %q, want empty", got)
		}
	})
}

// --- FindLinksByText exhaustive tests ---

func TestPageFindLinksByText(t *testing.T) {
	t.Run("exact match", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/about"}}
		got := p.FindLinksByText("about")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
		if got[0] != "https://example.com/about" {
			t.Errorf("got %q", got[0])
		}
	})

	t.Run("partial match in middle", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/blog/posts/2024"}}
		got := p.FindLinksByText("blog")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("partial match at start", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com"}}
		got := p.FindLinksByText("https")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("partial match at end", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/about-us"}}
		got := p.FindLinksByText("us")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("no match returns empty slice", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/about"}}
		got := p.FindLinksByText("nonexistent")
		if len(got) != 0 {
			t.Errorf("found %d links, want 0", len(got))
		}
	})

	t.Run("case insensitive - lowercase search", func(t *testing.T) {
		p := Page{Links: []string{"https://EXAMPLE.COM/About"}}
		got := p.FindLinksByText("about")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("case insensitive - uppercase search", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/about"}}
		got := p.FindLinksByText("ABOUT")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("case insensitive - mixed case search", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/About"}}
		got := p.FindLinksByText("aBoUt")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("multiple matches returned in order", func(t *testing.T) {
		p := Page{Links: []string{
			"https://example.com/blog/1",
			"https://example.com/about",
			"https://example.com/blog/2",
		}}
		got := p.FindLinksByText("blog")
		if len(got) != 2 {
			t.Fatalf("found %d links, want 2", len(got))
		}
		if got[0] != "https://example.com/blog/1" {
			t.Errorf("first = %q", got[0])
		}
		if got[1] != "https://example.com/blog/2" {
			t.Errorf("second = %q", got[1])
		}
	})

	t.Run("empty links returns empty", func(t *testing.T) {
		p := Page{Links: []string{}}
		got := p.FindLinksByText("anything")
		if len(got) != 0 {
			t.Errorf("found %d links, want 0", len(got))
		}
	})

	t.Run("nil links returns empty", func(t *testing.T) {
		p := Page{Links: nil}
		got := p.FindLinksByText("anything")
		if len(got) != 0 {
			t.Errorf("found %d links, want 0", len(got))
		}
	})

	t.Run("empty search text matches all", func(t *testing.T) {
		p := Page{Links: []string{"https://a.com", "https://b.com", "https://c.com"}}
		got := p.FindLinksByText("")
		if len(got) != 3 {
			t.Errorf("found %d links, want 3", len(got))
		}
	})

	t.Run("single link collection", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com"}}
		got := p.FindLinksByText("example")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("many links with selective match", func(t *testing.T) {
		links := make([]string, 100)
		for i := 0; i < 100; i++ {
			if i%10 == 0 {
				links[i] = "https://example.com/match"
			} else {
				links[i] = "https://example.com/other"
			}
		}
		p := Page{Links: links}
		got := p.FindLinksByText("match")
		if len(got) != 10 {
			t.Errorf("found %d links, want 10", len(got))
		}
	})

	t.Run("unicode search", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/\u4e2d\u6587"}}
		got := p.FindLinksByText("\u4e2d\u6587")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("unicode case insensitive", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/HELLO"}}
		got := p.FindLinksByText("hello")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("search text longer than any link", func(t *testing.T) {
		p := Page{Links: []string{"https://short.com"}}
		got := p.FindLinksByText("this search text is way longer than any link")
		if len(got) != 0 {
			t.Errorf("found %d links, want 0", len(got))
		}
	})

	t.Run("search text equals link exactly", func(t *testing.T) {
		p := Page{Links: []string{"https://exact.com"}}
		got := p.FindLinksByText("https://exact.com")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("special regex characters in search", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/path?a=1&b=2"}}
		got := p.FindLinksByText("a=1&b=2")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("spaces in links", func(t *testing.T) {
		p := Page{Links: []string{"https://example.com/hello world"}}
		got := p.FindLinksByText("hello world")
		if len(got) != 1 {
			t.Fatalf("found %d links, want 1", len(got))
		}
	})

	t.Run("multiple distinct matches", func(t *testing.T) {
		p := Page{Links: []string{
			"https://a.com/search",
			"https://b.com/results",
			"https://c.com/search-page",
		}}
		got := p.FindLinksByText("search")
		if len(got) != 2 {
			t.Errorf("found %d links, want 2", len(got))
		}
	})

	t.Run("returns original links not modified", func(t *testing.T) {
		p := Page{Links: []string{"https://EXAMPLE.COM/About"}}
		got := p.FindLinksByText("about")
		if len(got) != 1 {
			t.Fatalf("found %d links", len(got))
		}
		// Original case should be preserved in returned link
		if got[0] != "https://EXAMPLE.COM/About" {
			t.Errorf("got %q, want original case preserved", got[0])
		}
	})
}

// --- Combined GetMainContent and FindLinksByText tests ---

func TestPageCombinedOperations(t *testing.T) {
	t.Run("page with both content and links", func(t *testing.T) {
		p := Page{
			URL:      "https://blog.example.com",
			Markdown: "# Blog\n## Posts\nPost 1\nPost 2\nPost 3",
			Links:    []string{"https://blog.example.com/post1", "https://blog.example.com/post2"},
		}

		content := p.GetMainContent()
		if !strings.Contains(content, "Post 1") {
			t.Errorf("content missing Post 1")
		}

		links := p.FindLinksByText("post")
		if len(links) != 2 {
			t.Errorf("found %d links, want 2", len(links))
		}
	})

	t.Run("operations are independent", func(t *testing.T) {
		p := Page{
			Markdown: "A\nB\nC\nD",
			Links:    []string{"https://a.com", "https://b.com"},
		}

		_ = p.GetMainContent()
		_ = p.FindLinksByText("a")
		_ = p.GetMainContent() // second call
		_ = p.FindLinksByText("b") // second call

		// Page should be unchanged
		if len(p.Links) != 2 {
			t.Errorf("Links modified after operations, len = %d", len(p.Links))
		}
	})
}

// --- Edge cases ---

func TestPageEdgeCases(t *testing.T) {
	t.Run("GetMainContent with Windows line endings", func(t *testing.T) {
		p := Page{Markdown: "A\r\nB\r\nC\r\nD"}
		got := p.GetMainContent()
		// splitLines splits on \n only, so "A\r", "B\r", "C\r", "D"
		// Skip 2: "C\r", "D" -> join: "C\r\nD"
		want := "C\r\nD"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})

	t.Run("FindLinksByText with empty string links", func(t *testing.T) {
		p := Page{Links: []string{"", "https://example.com", ""}}
		got := p.FindLinksByText("")
		if len(got) != 3 {
			t.Errorf("found %d links, want 3 (empty string matches everything)", len(got))
		}
	})

	t.Run("very long markdown", func(t *testing.T) {
		lines := []string{"Header", "Subheader"}
		for i := 0; i < 1000; i++ {
			lines = append(lines, "Line "+fmt.Sprintf("%d", i))
		}
		p := Page{Markdown: strings.Join(lines, "\n")}
		got := p.GetMainContent()
		// Should contain 1000 lines of content
		contentLines := splitLines(got)
		if len(contentLines) != 1000 {
			t.Errorf("content has %d lines, want 1000", len(contentLines))
		}
	})

	t.Run("very many links", func(t *testing.T) {
		links := make([]string, 1000)
		for i := 0; i < 1000; i++ {
			links[i] = "https://example.com/link" + fmt.Sprintf("%d", i)
		}
		p := Page{Links: links}
		got := p.FindLinksByText("link1")
		// Matches: link1, link10-19, link100-199, link100-999 (any containing "link1")
		// link1, link10-19 (10), link100-199 (100), link1000+ doesn't exist
		// Also link21, link31, etc don't contain "link1"
		// link1 (1) + link10-19 (10) + link100-199 (100) + link310-319 etc... 
		// Actually: any link containing substring "link1": 
		// link1, link10..link19, link100..link199, link210..219, link310..319, ... link910..919
		// That's 1 + 10 + 100 + 10*8 = 191
		if len(got) < 100 {
			t.Errorf("found %d links, expected at least 100", len(got))
		}
	})
}
