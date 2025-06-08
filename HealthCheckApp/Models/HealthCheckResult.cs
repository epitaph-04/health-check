using System;

namespace HealthCheckApp.Models
{
    public class HealthCheckResult
    {
        public string ApplicationName { get; set; }
        public HealthStatus Status { get; set; }
        public TimeSpan ResponseTime { get; set; }
        public DateTime LastCheckedUtc { get; set; }
        public string? ErrorMessage { get; set; } // Null if healthy

        public HealthCheckResult(string applicationName)
        {
            ApplicationName = applicationName;
            Status = HealthStatus.Unknown;
            LastCheckedUtc = DateTime.UtcNow; // Initialize to now, will be updated
        }
    }
}
