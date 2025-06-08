using Microsoft.AspNetCore.SignalR;

namespace HealthCheckApp.Hubs
{
    public class HealthCheckHub : Hub
    {
        // No server-callable methods needed from clients for the current design.
        // The HealthCheckOrchestratorService will use IHubContext<HealthCheckHub>
        // to push messages to clients.
        //
        // Example of a server-callable method (not used in current plan but good for reference):
        // public async Task SendMessage(string user, string message)
        // {
        //     await Clients.All.SendAsync("ReceiveMessage", user, message);
        // }
    }
}
