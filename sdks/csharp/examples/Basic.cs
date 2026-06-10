using B4N1Web;

// B4n1Web SDK Basic Example
// Requires the b4n1web binary installed:
//   curl -sL https://web.b4n1.com/install | bash
//
// Run: dotnet run --project examples/Basic.csproj

try
{
    // 1. Create AgentBrowser with Light mode
    var browser = new AgentBrowser(new BrowserOptions
    {
        Mode = BrowserMode.Light,
        Timeout = 15
    });

    Console.WriteLine("=== B4n1Web SDK Demo ===\n");

    // 2. Basic navigation
    Console.WriteLine("--- Goto (basic) ---");
    var page = browser.Goto("https://example.com");
    Console.WriteLine($"URL: {page.Url}");
    Console.WriteLine($"Markdown length: {page.Markdown.Length} chars");
    Console.WriteLine($"Links count: {page.Links.Count}");
    Console.WriteLine();

    // 3. Navigation with waitFor selector
    Console.WriteLine("--- Goto with waitFor ---");
    try
    {
        var page2 = browser.Goto("https://example.com", waitFor: "h1");
        Console.WriteLine($"URL: {page2.Url}");
        Console.WriteLine($"Content (first 200 chars): {page2.GetMainContent()[..Math.Min(200, page2.GetMainContent().Length)]}...");
        Console.WriteLine($"Links count: {page2.Links.Count}");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"waitFor not supported by this binary version: {ex.Message}");
    }
    Console.WriteLine();

    // 4. Page helper methods
    Console.WriteLine("--- Page helpers ---");
    Console.WriteLine($"GetMainContent() length: {page.GetMainContent().Length} chars");
    var foundLinks = page.FindLinksByText("more");
    Console.WriteLine($"FindLinksByText(\"more\"): {foundLinks.Count} links found");
    Console.WriteLine();

    // 5. GetLinks from last page
    Console.WriteLine("--- GetLinks() ---");
    var links = browser.GetLinks();
    Console.WriteLine($"Links from last page: {links.Length}");
    foreach (var link in links)
    {
        Console.WriteLine($"  - {link}");
    }
    Console.WriteLine();

    // 6. GetLinksFromPage (static)
    Console.WriteLine("--- GetLinksFromPage() ---");
    var staticLinks = AgentBrowser.GetLinksFromPage("https://example.com");
    Console.WriteLine($"Links from static method: {staticLinks.Length}");
    foreach (var link in staticLinks)
    {
        Console.WriteLine($"  - {link}");
    }
    Console.WriteLine();

    // 7. JsOutput (may not be populated in Light mode)
    Console.WriteLine("--- JsOutput ---");
    Console.WriteLine($"JsOutput: {page.JsOutput ?? "(null in Light mode)"}");
    Console.WriteLine();

    // 8. Screenshot (requires Render mode or binary support)
    Console.WriteLine("--- Screenshot ---");
    try
    {
        var ss = browser.Screenshot(800, 600);
        Console.WriteLine($"Screenshot captured: {ss.Length} chars of base64 data");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Screenshot not supported by this binary: {ex.Message}");
    }
    Console.WriteLine();

    // 9. WaitForSelector
    Console.WriteLine("--- WaitForSelector ---");
    try
    {
        var found = browser.WaitForSelector("h1", 5000);
        Console.WriteLine($"Selector 'h1' found: {found}");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"WaitForSelector not supported by this binary: {ex.Message}");
    }
    Console.WriteLine();

    // 10. Click
    Console.WriteLine("--- Click ---");
    try
    {
        browser.Click("a");
        Console.WriteLine("Click executed");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Click not supported by this binary: {ex.Message}");
    }
    Console.WriteLine();

    // 11. TypeText
    Console.WriteLine("--- TypeText ---");
    try
    {
        browser.TypeText("input", "hello world", clearFirst: true);
        Console.WriteLine("TypeText executed");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"TypeText not supported by this binary: {ex.Message}");
    }
    Console.WriteLine();

    // 12. Cleanup
    Console.WriteLine("--- Cleanup ---");
    browser.Close();
    Console.WriteLine("Browser closed.");
    Console.WriteLine();

    // 13. Async usage
    Console.WriteLine("--- Async ---");
    await using var asyncBrowser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light, Timeout = 10 });
    var asyncPage = await asyncBrowser.GotoAsync("https://example.com");
    Console.WriteLine($"Async page loaded: {asyncPage.Url} ({asyncPage.Markdown.Length} chars)");
    Console.WriteLine();

    Console.WriteLine("=== All features demonstrated successfully! ===");
    return 0;
}
catch (BinaryNotFoundException ex)
{
    Console.Error.WriteLine($"ERROR: {ex.Message}");
    return 1;
}
catch (Exception ex)
{
    Console.Error.WriteLine($"ERROR: {ex.GetType().Name}: {ex.Message}");
    return 2;
}
