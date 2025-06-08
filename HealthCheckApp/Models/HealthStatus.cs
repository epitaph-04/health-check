namespace HealthCheckApp.Models
{
    public enum HealthStatus
    {
        Unknown,    // Status before the first check
        Healthy,
        Unhealthy,
        Degraded    // Partially working or slow
    }
}
