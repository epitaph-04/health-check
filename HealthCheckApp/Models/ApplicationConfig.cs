using System.Collections.Generic;

namespace HealthCheckApp.Models;

public class ApplicationConfig
{
    public string Name { get; set; }
    public ApplicationType Type { get; set; }
    public string Target { get; set; }
    public int ExpectedResponseCode { get; set; } = 200;
    public string Method { get; set; } = "GET";
    public string? RequestBody { get; set; }
    public Dictionary<string, string>? Headers { get; set; }
    public string? Query { get; set; }
    public int TimeoutSeconds { get; set; } = 30;
}
