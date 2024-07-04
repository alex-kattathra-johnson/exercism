using System;

class Log {
    public string level { get; set; }
    public string message { get; set; }

    public Log(string level, string message) {
        this.level = level;
        this.message = message;
    }
}

static class LogLine
{
    private static Log CreateLog(string logLine) {
        char[] separators = new char[] { ' ', ':', '[', ']' };
        string[] parts = logLine.Split(separators, 2, StringSplitOptions.RemoveEmptyEntries);
        parts[0] = parts[0].ToLower();
        parts[1] = parts[1].Trim();
        return new Log(parts[0], parts[1]);
    }

    public static string Message(string logLine)
    {
        Log log = CreateLog(logLine);
        return log.message;
    }

    public static string LogLevel(string logLine)
    {
        Log log = CreateLog(logLine);
        return log.level;
    }

    public static string Reformat(string logLine)
    {
        Log log = CreateLog(logLine);
        return $"{log.message} ({log.level})";
    }
}
