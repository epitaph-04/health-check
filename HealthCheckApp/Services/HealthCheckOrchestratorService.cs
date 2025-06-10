using HealthCheckApp.Models;
using HealthCheckApp.Hubs;
using Microsoft.AspNetCore.SignalR;
using System.Collections.Concurrent;

namespace HealthCheckApp.Services
{
    public class HealthCheckOrchestratorService(
        ILogger<HealthCheckOrchestratorService> logger,
        IServiceProvider serviceProvider,
        ConfigurationService configService,
        IHubContext<HealthCheckHub, IHealthCheckClient> hubContext)
        : BackgroundService
    {
        private readonly TimeSpan _period = TimeSpan.FromSeconds(30);
        private readonly ConcurrentDictionary<string, HealthCheckResult> _healthCheckResults = new ();
        private List<ApplicationConfig> _applicationConfigs = new ();

        protected override async Task ExecuteAsync(CancellationToken stoppingToken)
        {
            logger.LogInformation("Health Check Orchestrator Service starting.");
            _applicationConfigs = configService.Config.Applications ?? new ();

            foreach (var appConfig in _applicationConfigs)
            {
                _healthCheckResults.TryAdd(appConfig.Name, new HealthCheckResult(appConfig.Name) { Status = HealthStatus.Unknown });
            }
            
            using var timer = new PeriodicTimer(_period);

            while (!stoppingToken.IsCancellationRequested && await timer.WaitForNextTickAsync(stoppingToken))
            {
                logger.LogInformation("Executing health check {Time}", DateTime.Now);
                await RunAllChecksAsync();
            }
        }

        private async Task RunAllChecksAsync()
        {
            logger.LogInformation("Running all health checks.");
            if (_applicationConfigs.Count == 0)
            {
                logger.LogInformation("No applications configured for health checks.");
                return;
            }
            
            var healthCheckServices = serviceProvider.GetServices<IHealthCheckService>().ToArray();

            foreach (var appConfig in _applicationConfigs)
            {
                try
                {
                    var service = healthCheckServices?.FirstOrDefault(s => s.Type == appConfig.Type);
                    if (service != null)
                    {
                        logger.LogInformation("Checking health for {appName} ({appType}).", appConfig.Name, appConfig.Type);
                        HealthCheckResult result = await service.CheckHealthAsync(appConfig);
                        _healthCheckResults[appConfig.Name] = result;
                        await hubContext.Clients.All.ReceiveHealthUpdate(result);
                         logger.LogInformation("Health check for {appName} completed: {status}", appConfig.Name, result.Status);
                    }
                    else
                    {
                        logger.LogWarning($"No IHealthCheckService found for type {appConfig.Type} (Application: {appConfig.Name}).");
                         var errorResult = new HealthCheckResult(appConfig.Name)
                        {
                            Status = HealthStatus.Unhealthy,
                            ErrorMessage = $"Configuration error: No health check service registered for type '{appConfig.Type}'.",
                            LastCheckedUtc = DateTime.UtcNow
                        };
                        _healthCheckResults[appConfig.Name] = errorResult;
                        await hubContext.Clients.All.ReceiveHealthUpdate(errorResult);
                    }
                }
                catch (Exception ex)
                {
                    logger.LogError(ex, "Error during health check for {appName}.", appConfig.Name);
                    var errorResult = new HealthCheckResult(appConfig.Name)
                    {
                        Status = HealthStatus.Unhealthy,
                        ErrorMessage = $"Orchestrator error: {ex.Message}",
                        LastCheckedUtc = DateTime.UtcNow
                    };
                    _healthCheckResults[appConfig.Name] = errorResult;
                    await hubContext.Clients.All.ReceiveHealthUpdate(errorResult);
                }
            }
        }

        public IEnumerable<HealthCheckResult> GetAllCurrentStatuses()
        {
            return _healthCheckResults.Values.ToList();
        }
    }
}
