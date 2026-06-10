using B4N1Web;
using Xunit;

namespace B4n1Web.Tests;

public class ErrorTests
{
    [Fact]
    public void BinaryNotFoundException_ShouldBeException()
    {
        var ex = new BinaryNotFoundException();

        Assert.IsAssignableFrom<Exception>(ex);
    }

    [Fact]
    public void BinaryNotFoundException_Message_ShouldReferenceB4n1Web()
    {
        var ex = new BinaryNotFoundException();

        Assert.Contains("B4n1Web binary not found", ex.Message);
    }

    [Fact]
    public void BinaryNotFoundException_Message_ShouldIncludeInstallInstructions()
    {
        var ex = new BinaryNotFoundException();

        Assert.Contains("install", ex.Message, StringComparison.OrdinalIgnoreCase);
    }

    [Fact]
    public void BinaryNotFoundException_Message_ShouldIncludeBinaryName()
    {
        var ex = new BinaryNotFoundException();

        Assert.Contains("b4n1web", ex.Message, StringComparison.OrdinalIgnoreCase);
    }

    [Fact]
    public void BinaryNotFoundException_CanBeCaughtAsBinaryNotFoundException()
    {
        Exception? caught = null;

        try
        {
            ThrowBinaryNotFound();
        }
        catch (BinaryNotFoundException ex)
        {
            caught = ex;
        }

        Assert.IsType<BinaryNotFoundException>(caught);
    }

    [Fact]
    public void BinaryNotFoundException_CanBeCaughtAsException()
    {
        Exception? caught = null;

        try
        {
            ThrowBinaryNotFound();
        }
        catch (Exception ex)
        {
            caught = ex;
        }

        Assert.IsType<BinaryNotFoundException>(caught);
    }

    private static void ThrowBinaryNotFound()
    {
        throw new BinaryNotFoundException();
    }
}
