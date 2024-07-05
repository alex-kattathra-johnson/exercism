using System;

public static class LogAnalysis 
{
    public static string SubstringAfter(this string str, string delim)
    {
        string[] parts = str.Split(delim, 2);
        return parts[1];
    }

    public static string SubstringBetween(this string str, string start, string end)
    {
        int startIdx = str.IndexOf(start) + start.Length;
        int endIdx = str.IndexOf(end);
        int length = endIdx - startIdx;
        return str.Substring(startIdx, length);
    }

    public static string Message(this string str) => str.SubstringAfter(": ");

    public static string LogLevel(this string str) => str.SubstringBetween("[", "]");
}

