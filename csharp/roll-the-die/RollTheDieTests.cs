using Xunit;
using Exercism.Tests;

public class RollTheDieTests
{
    [Fact]
    [Task(1)]
    public void RollDie()
    {
        var player = new Player();
        for (var i = 0; i < 10000; i++)
        {
            Assert.InRange(player.RollDie(), 1, 18);
        }
    }

    [Fact]
    [Task(2)]
    public void GenerateSpellStrength()
    {
        var player = new Player();
        for (var i = 0; i < 10000; i++)
        {
            Assert.InRange(player.GenerateSpellStrength(), 0.0, 100.0);
        }
    }
}
