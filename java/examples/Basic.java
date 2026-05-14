import com.b4n1.web.*;

/**
 * Basic usage example for B4n1Web Java SDK.
 *
 * Compile:
 *   javac -cp src/main/java Basic.java
 *
 * Run (requires b4n1web binary installed):
 *   java -cp .:src/main/java Basic
 */
public class Basic {

    public static void main(String[] args) {
        // Modes: LIGHT (HTTP fetch), JS (light + JS), RENDER (headless browser)
        demoBasicNavigation();
        demoWaitForSelector();
        demoClickAndType();
        demoScreenshot();
        demoGetLinksFromPage();
        demoErrorHandling();
    }

    static void demoBasicNavigation() {
        System.out.println("=== Basic Navigation ===");

        try (AgentBrowser browser = new AgentBrowser(new BrowserOptions() {{
                setMode(BrowserMode.LIGHT);
                setTimeout(15);
        }})) {
            // Basic goto (no waitFor)
            Page page = browser.goto_("https://example.com");
            System.out.println("URL: " + page.getUrl());
            System.out.println("Markdown length: " + page.getMarkdown().length());

            // Goto with waitFor selector
            Page pageWithWait = browser.goto_("https://example.com", "h1");
            System.out.println("JS Output: " + pageWithWait.getJsOutput());

            // Navigate alias
            Page page2 = browser.navigate("https://example.com");
            System.out.println("Navigated to: " + page2.getUrl());

        } catch (BinaryNotFoundException e) {
            System.err.println("Binary not installed, skipping demo: " + e.getMessage());
        }
    }

    static void demoWaitForSelector() {
        System.out.println("\n=== Wait For Selector ===");

        try (AgentBrowser browser = new AgentBrowser()) {
            browser.goto_("https://example.com");
            boolean found = browser.waitForSelector("h1", 5000);
            System.out.println("Selector 'h1' found: " + found);

            boolean notFound = browser.waitForSelector(".nonexistent", 1000);
            System.out.println("Selector '.nonexistent' found: " + notFound);
        } catch (BinaryNotFoundException e) {
            System.err.println("Binary not installed, skipping demo: " + e.getMessage());
        }
    }

    static void demoClickAndType() {
        System.out.println("\n=== Click and Type ===");

        try (AgentBrowser browser = new AgentBrowser(new BrowserOptions() {{
                setMode(BrowserMode.RENDER);
        }})) {
            // Click an element
            browser.click("#submit-button");

            // Type into a field (append, no clear)
            browser.typeText("#search-input", "hello world", false);

            // Type into a field (clear first)
            browser.typeText("#search-input", "new text", true);

            System.out.println("Click and type executed successfully.");
        } catch (BinaryNotFoundException e) {
            System.err.println("Binary not installed, skipping demo: " + e.getMessage());
        }
    }

    static void demoScreenshot() {
        System.out.println("\n=== Screenshot ===");

        try (AgentBrowser browser = new AgentBrowser()) {
            browser.goto_("https://example.com");
            String screenshot = browser.screenshot(1024, 768);
            System.out.println("Screenshot base64 length: " + screenshot.length());

            String smallScreenshot = browser.screenshot(640, 480);
            System.out.println("Small screenshot base64 length: " + smallScreenshot.length());
        } catch (BinaryNotFoundException e) {
            System.err.println("Binary not installed, skipping demo: " + e.getMessage());
        }
    }

    static void demoGetLinksFromPage() {
        System.out.println("\n=== Get Links From Page ===");

        try (AgentBrowser browser = new AgentBrowser()) {
            Page page = browser.goto_("https://example.com");

            // As List
            page.getLinks().forEach(link -> System.out.println("Link: " + link));

            // As String array
            String[] linksArray = page.getLinksArray();
            System.out.println("Total links (array): " + linksArray.length);

            // Static convenience
            String[] staticLinks = AgentBrowser.getLinksFromPage("https://example.com");
            System.out.println("Static links count: " + staticLinks.length);

        } catch (BinaryNotFoundException e) {
            System.err.println("Binary not installed, skipping demo: " + e.getMessage());
        }
    }

    static void demoErrorHandling() {
        System.out.println("\n=== Error Handling ===");

        // Browser without binary should throw BinaryNotFoundException
        // (can't test this directly since it depends on install)

        try (AgentBrowser browser = new AgentBrowser()) {
            // Invalid URL should throw NavigationException
            Page page = browser.goto_("not-a-valid-url");
            System.out.println("Unexpected success: " + page.getUrl());
        } catch (NavigationException e) {
            System.out.println("Caught NavigationException for URL: " + e.getUrl());
            System.out.println("Message: " + e.getMessage());
        } catch (BinaryNotFoundException e) {
            System.err.println("Binary not installed, skipping error handling demo: " + e.getMessage());
        }
    }
}
