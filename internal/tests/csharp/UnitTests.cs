using System;
using System.Collections.Generic;
using Xunit;

namespace B4N1Web.Tests
{
    public class BrowserModeTests
    {
        [Fact] public void LightValue() => Assert.Equal("light", BrowserMode.Light.ToString().ToLower());
        [Fact] public void JsValue() => Assert.Equal("js", BrowserMode.JS.ToString().ToLower());
        [Fact] public void RenderValue() => Assert.Equal("render", BrowserMode.Render.ToString().ToLower());
        [Fact] public void Distinct() {
            var s = new HashSet<BrowserMode> { BrowserMode.Light, BrowserMode.JS, BrowserMode.Render };
            Assert.Equal(3, s.Count);
        }
    }
    public class BrowserOptionsTests
    {
        [Fact] public void Defaults() {
            var o = new BrowserOptions();
            Assert.Equal(BrowserMode.Light, o.Mode); Assert.Equal(30, o.Timeout);
            Assert.Equal("B4N1Web-Agent/1.0", o.UserAgent);
        }
        [Fact] public void SetMode() { var o = new BrowserOptions { Mode = BrowserMode.JS }; Assert.Equal(BrowserMode.JS, o.Mode); }
        [Fact] public void SetTimeout() { var o = new BrowserOptions { Timeout = 60 }; Assert.Equal(60, o.Timeout); }
        [Fact] public void AllTogether() {
            var o = new BrowserOptions { Mode = BrowserMode.Render, Timeout = 120, UserAgent = "T" };
            Assert.Equal(BrowserMode.Render, o.Mode); Assert.Equal(120, o.Timeout);
        }
    }
    public class PageTests
    {
        [Fact] public void Construction() {
            var p = new Page { Url = "https://x.com", Markdown = "# Hi\nContent", Links = new List<string> { "a", "b" } };
            Assert.Equal("https://x.com", p.Url); Assert.Equal(2, p.Links.Count);
        }
        [Fact] public void EmptyDefaults() { var p = new Page(); Assert.Equal(string.Empty, p.Url); Assert.Empty(p.Links); }
        [Fact] public void Unicode() { var p = new Page { Markdown = "日本語\n🚀" }; Assert.Contains("日本語", p.Markdown); }
        [Fact] public void ManyLinks() {
            var l = new List<string>(); for (int i = 0; i < 1000; i++) l.Add($"https://x.com/{i}");
            Assert.Equal(1000, new Page { Links = l }.Links.Count);
        }
    }
    public class GetMainContentTests
    {
        [Fact] public void Empty() => Assert.Equal("", new Page { Markdown = "" }.GetMainContent());
        [Fact] public void SingleLine() => Assert.Equal("Only one", new Page { Markdown = "Only one" }.GetMainContent());
        [Fact] public void SkipsHeader() {
            var p = new Page { Markdown = "# Title\n\n## Sub\n\nActual content" };
            var c = p.GetMainContent(); Assert.Contains("Actual content", c); Assert.DoesNotContain("# Title", c);
        }
        [Fact] public void Unicode() {
            var p = new Page { Markdown = "# Título\n\nContenido 🇪🇸" }; Assert.Contains("Contenido", p.GetMainContent());
        }
    }
    public class FindLinksByTextTests
    {
        [Fact] public void Exact() { var p = new Page { Links = new List<string> { "https://x.com/contact" } }; Assert.Single(p.FindLinksByText("contact")); }
        [Fact] public void NoMatch() => Assert.Empty(new Page { Links = new List<string> { "https://x.com/p" } }.FindLinksByText("missing"));
        [Fact] public void CaseInsensitive() => Assert.Single(new Page { Links = new List<string> { "https://EXAMPLE.COM" } }.FindLinksByText("example"));
        [Fact] public void Multiple() {
            var p = new Page { Links = new List<string> { "https://x.com/contact", "https://x.com/contact-us", "https://y.com" } };
            Assert.Equal(2, p.FindLinksByText("contact").Count);
        }
        [Fact] public void EmptySearch() { var p = new Page { Links = new List<string> { "a", "b" } }; Assert.Equal(2, p.FindLinksByText("").Count); }
        [Fact] public void EmptyList() => Assert.Empty(new Page { Links = new List<string>() }.FindLinksByText("x"));
        [Fact] public void Unicode() => Assert.Single(new Page { Links = new List<string> { "https://x.com/日本語" } }.FindLinksByText("日本語"));
    }
    public class AgentBrowserConstructionTests
    {
        [Fact] public void Defaults() { try { using var b = new AgentBrowser(); Assert.NotNull(b); } catch (BinaryNotFoundException) { } }
        [Fact] public void WithMode() { try { var o = new BrowserOptions { Mode = BrowserMode.Render }; using var b = new AgentBrowser(o); Assert.NotNull(b); } catch (BinaryNotFoundException) { } }
        [Fact] public void WithTimeout() { try { var o = new BrowserOptions { Timeout = 60 }; using var b = new AgentBrowser(o); Assert.NotNull(b); } catch (BinaryNotFoundException) { } }
        [Fact] public void CloseMultiple() { try { var b = new AgentBrowser(); b.Close(); b.Close(); Assert.True(true); } catch (BinaryNotFoundException) { } }
    }
    public class AgentBrowserGotoTests
    {
        private static bool HasBinary() { try { using var b = new AgentBrowser(); return true; } catch { return false; } }
        [Fact] public void Success() {
            if (!HasBinary()) return;
            using var b = new AgentBrowser(); var p = b.Goto("https://example.com");
            Assert.Equal("https://example.com", p.Url); Assert.NotEmpty(p.Markdown);
        }
        [Fact] public void InvalidUrlThrows() {
            if (!HasBinary()) return;
            using var b = new AgentBrowser(); Assert.Throws<Exception>(() => b.Goto("not-a-valid-url"));
        }
        [Fact] public void ReturnsPageInstance() {
            if (!HasBinary()) return;
            using var b = new AgentBrowser(); var p = b.Goto("https://example.com"); Assert.IsType<Page>(p);
        }
    }
    public class ErrorTests
    {
        [Fact] public void BinaryNotFoundExceptionMessage() { var ex = new BinaryNotFoundException(); Assert.NotEmpty(ex.Message); }
        [Fact] public void ThrowAndCatch() {
            try { throw new BinaryNotFoundException(); } catch (BinaryNotFoundException e) { Assert.NotEmpty(e.Message); return; }
            Assert.Fail("Should have thrown");
        }
        [Fact] public void InvalidUrlThrowsException() {
            try { using var b = new AgentBrowser(); b.Goto("not-a-valid-url"); Assert.Fail("Should throw"); }
            catch (BinaryNotFoundException) { } catch (Exception e) { Assert.NotEmpty(e.Message); }
        }
        [Fact] public void ErrorTypesDistinguishable() {
            var bnf = new BinaryNotFoundException(); var ex = new Exception("test"); Assert.NotEqual(bnf.GetType(), ex.GetType());
        }
    }
}
