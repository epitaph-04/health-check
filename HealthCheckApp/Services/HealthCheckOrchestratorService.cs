using HealthCheckApp.Models;
using HealthCheckApp.Hubs; // Placeholder for HealthCheckHub
using Microsoft.AspNetCore.SignalR;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace HealthCheckApp.Services
{
    public class HealthCheckOrchestratorService : IHostedService, IDisposable
    {
        private readonly ILogger<HealthCheckOrchestratorService> _logger;
        private readonly IServiceProvider _serviceProvider;
        private readonly ConfigurationService _configService;
        private readonly IHubContext<HealthCheckHub> _hubContext; // HealthCheckHub will be created later
        private Timer? _timer;
        private readonly ConcurrentDictionary<string, HealthCheckResult> _healthCheckResults = new ConcurrentDictionary<string, HealthCheckResult>();
        private List<ApplicationConfig> _applicationConfigs = new List<ApplicationConfig>();

        public HealthCheckOrchestratorService(
            ILogger<HealthCheckOrchestratorService> logger,
            IServiceProvider serviceProvider,
            ConfigurationService configService,
            IHubContext<HealthCheckHub> hubContext) // HealthCheckHub will be created later
        {
            _logger = logger;
            _serviceProvider = serviceProvider;
            _configService = configService;
            _hubContext = hubContext;
        }

        public Task StartAsync(CancellationToken cancellationToken)
        {
            _logger.LogInformation("Health Check Orchestrator Service starting.");
            _applicationConfigs = _configService.Config?.Applications ?? new List<ApplicationConfig>();

            foreach (var appConfig in _applicationConfigs)
            {
                _healthCheckResults.TryAdd(appConfig.Name, new HealthCheckResult(appConfig.Name) { Status = HealthStatus.Unknown });
            }

            // Initial check run shortly after start, then periodic
            _timer = new Timer(async _ => await RunAllChecksAsync(), null, TimeSpan.FromSeconds(5), TimeSpan.FromSeconds(30));

            return Task.CompletedTask;
        }

        private async Task RunAllChecksAsync()
        {
            _logger.LogInformation("Running all health checks.");
            if (!_applicationConfigs.Any())
            {
                _logger.LogInformation("No applications configured for health checks.");
                return;
            }

            // Get all registered health check services
            var healthCheckServices = _serviceProvider.GetServices<IHealthCheckService>();

            foreach (var appConfig in _applicationConfigs)
            {
                try
                {
                    var service = healthCheckServices.FirstOrDefault(s => s.Type == appConfig.Type);
                    if (service != null)
                    {
                        _logger.LogInformation($"Checking health for {appConfig.Name} ({appConfig.Type}).");
                        HealthCheckResult result = await service.CheckHealthAsync(appConfig);
                        _healthCheckResults[appConfig.Name] = result;
                        await _hubContext.Clients.All.SendAsync("ReceiveHealthUpdate", result);
                         _logger.LogInformation($"Health check for {appConfig.Name} completed: {result.Status}");
                    }
                    else
                    {
                        _logger.LogWarning($"No IHealthCheckService found for type {appConfig.Type} (Application: {appConfig.Name}).");
                         var errorResult = new HealthCheckResult(appConfig.Name)
                        {
                            Status = HealthStatus.Unhealthy,
                            ErrorMessage = $"Configuration error: No health check service registered for type '{appConfig.Type}'.",
                            LastCheckedUtc = DateTime.UtcNow
                        };
                        _healthCheckResults[appConfig.Name] = errorResult;
                        await _hubContext.Clients.All.SendAsync("ReceiveHealthUpdate", errorResult);
                    }
                }
                catch (Exception ex)
                {
                    _logger.LogError(ex, $"Error during health check for {appConfig.Name}.");
                    var errorResult = new HealthCheckResult(appConfig.Name)
                    {
                        Status = HealthStatus.Unhealthy,
                        ErrorMessage = $"Orchestrator error: {ex.Message}",
                        LastCheckedUtc = DateTime.UtcNow
                    };
                    _healthCheckResults[appConfig.Name] = errorResult;
                    await _hubContext.Clients.All.SendAsync("ReceiveHealthUpdate", errorResult);
                }
            }
        }

        public IEnumerable<HealthCheckResult> GetAllCurrentStatuses()
        {
            return _healthCheckResults.Values.ToList();
        }

        public Task StopAsync(CancellationToken cancellationToken)
        {
            _logger.LogInformation("Health Check Orchestrator Service stopping.");
            _timer?.Change(Timeout.Infinite, 0);
            return Task.CompletedTask;
        }

        public void Dispose()
        {
            _timer?.Dispose();
            GC.SuppressFinalize(this);
        }
    }
}
