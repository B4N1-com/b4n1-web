using B4N1Web;
using Xunit;

namespace B4n1Web.Tests;

public class BrowserModeTests
{
    [Fact]
    public void BrowserMode_ShouldHaveThreeValues()
    {
        var values = Enum.GetValues<BrowserMode>();
        Assert.Equal(3, values.Length);
    }

    [Fact]
    public void Light_ToString_ReturnsLight()
    {
        Assert.Equal("Light", BrowserMode.Light.ToString());
    }

    [Fact]
    public void JS_ToString_ReturnsJS()
    {
        Assert.Equal("JS", BrowserMode.JS.ToString());
    }

    [Fact]
    public void Render_ToString_ReturnsRender()
    {
        Assert.Equal("Render", BrowserMode.Render.ToString());
    }

    [Fact]
    public void BrowserOptions_DefaultMode_ShouldBeLight()
    {
        var options = new BrowserOptions();
        Assert.Equal(BrowserMode.Light, options.Mode);
    }

    [Fact]
    public void BrowserOptions_DefaultTimeout_ShouldBe30()
    {
        var options = new BrowserOptions();
        Assert.Equal(30, options.Timeout);
    }

    [Fact]
    public void BrowserOptions_DefaultUserAgent_ShouldContainB4N1Web()
    {
        var options = new BrowserOptions();
        Assert.Contains("B4N1Web", options.UserAgent);
    }

    [Fact]
    public void BrowserOptions_Mode_CanBeSetToJS()
    {
        var options = new BrowserOptions { Mode = BrowserMode.JS };
        Assert.Equal(BrowserMode.JS, options.Mode);
    }

    [Fact]
    public void BrowserOptions_Mode_CanBeSetToRender()
    {
        var options = new BrowserOptions { Mode = BrowserMode.Render };
        Assert.Equal(BrowserMode.Render, options.Mode);
    }

    [Fact]
    public void BrowserOptions_Timeout_CanBeCustomized()
    {
        var options = new BrowserOptions { Timeout = 60 };
        Assert.Equal(60, options.Timeout);
    }

    [Fact]
    public void BrowserOptions_UserAgent_CanBeCustomized()
    {
        var options = new BrowserOptions { UserAgent = "CustomAgent/1.0" };
        Assert.Equal("CustomAgent/1.0", options.UserAgent);
    }
}
