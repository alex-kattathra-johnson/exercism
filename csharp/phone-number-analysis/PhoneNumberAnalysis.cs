using System;

public static class PhoneNumber
{
    public static (bool IsNewYork, bool IsFake, string LocalNumber) Analyze(string phoneNumber)
    {
        string[] phoneNumberParts = phoneNumber.Split('-');
        bool IsNewYork = phoneNumberParts[0] == "212";
        bool IsFake = phoneNumberParts[1] == "555";
        string LocalNumber = phoneNumberParts[2];
        return (IsNewYork, IsFake, LocalNumber);
    }

    public static bool IsFake((bool IsNewYork, bool IsFake, string LocalNumber) phoneNumberInfo) => phoneNumberInfo.IsFake;
}
