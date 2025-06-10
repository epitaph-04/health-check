using HealthCheckApp.Models;
using Microsoft.AspNetCore.SignalR;

namespace HealthCheckApp.Hubs
{
    public class HealthCheckHub : Hub<IHealthCheckClient> { }
    
    public interface IHealthCheckClient
    {
        Task ReceiveHealthUpdate(HealthCheckResult result);
    }
}
