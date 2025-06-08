using System.Collections.Generic;

namespace HealthCheckApp.Models;

public class ApplicationConfig
{
    public string Name { get; set; }
    public ApplicationType Type { get; set; }
    public string Target { get; set; } // URL for HTTP, Connection String for DB
    public int ExpectedResponseCode { get; set; } = 200; // Default for HTTP
    public string Method { get; set; } = "GET"; // Default for HTTP
    public string? RequestBody { get; set; } // Optional body for HTTP POST/PUT
    public Dictionary<string, string>? Headers { get; set; } // Optional headers for HTTP
    public string? Query { get; set; } // For DB checks, e.g., "SELECT 1"
    public int TimeoutSeconds { get; set; } = 30; // Default timeout
}
