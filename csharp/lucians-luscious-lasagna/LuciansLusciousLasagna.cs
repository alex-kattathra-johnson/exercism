class Lasagna
{
    private readonly int timeToCook = 40;

    public int ExpectedMinutesInOven()
    {
        return timeToCook;
    }

    public int RemainingMinutesInOven(int minutes)
    {
        return ExpectedMinutesInOven() - minutes;
    }
    public int PreparationTimeInMinutes(int layers)
    {
        return layers * 2;
    }

    public int ElapsedTimeInMinutes(int layers, int timeInOven)
    {
        return PreparationTimeInMinutes(layers) + timeInOven;
    }
}
