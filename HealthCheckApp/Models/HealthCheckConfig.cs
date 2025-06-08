using System.Collections.Generic;

namespace HealthCheckApp.Models;

public class HealthCheckConfig
{
    public List<ApplicationConfig> Applications { get; set; } = new List<ApplicationConfig>();
}
