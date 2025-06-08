using HealthCheckApp.Models;
using System.Threading.Tasks;

namespace HealthCheckApp.Services
{
    public interface IHealthCheckService
    {
        ApplicationType Type { get; } // To identify which service handles which config type
        Task<HealthCheckResult> CheckHealthAsync(ApplicationConfig appConfig);
    }
}
