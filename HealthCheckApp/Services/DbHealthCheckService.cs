using HealthCheckApp.Models;
using System;
using System.Data.Common; // For DbException
using System.Diagnostics;
using System.Threading.Tasks;

namespace HealthCheckApp.Services
{
    public class DbHealthCheckService : IHealthCheckService
    {
        public ApplicationType Type => ApplicationType.DB;

        // In a real app, you might inject DbProviderFactory or a specific connection factory
        public DbHealthCheckService() { }

        public async Task<HealthCheckResult> CheckHealthAsync(ApplicationConfig appConfig)
        {
            var result = new HealthCheckResult(appConfig.Name);
            var stopwatch = new Stopwatch();
            stopwatch.Start();

            try
            {
                // --- SIMULATION LOGIC ---
                if (string.IsNullOrWhiteSpace(appConfig.Target))
                {
                    result.Status = HealthStatus.Unhealthy;
                    result.ErrorMessage = "Database connection string (Target) is missing.";
                }
                else if (appConfig.Target.Contains("simulated-failure-connection"))
                {
                    await Task.Delay(TimeSpan.FromMilliseconds(new Random().Next(50, 200))); // Simulate connection attempt time
                    throw new Exception("Simulated connection failure.");
                }
                else if (string.IsNullOrWhiteSpace(appConfig.Query))
                {
                    result.Status = HealthStatus.Unhealthy;
                    result.ErrorMessage = "Database query is missing.";
                }
                else
                {
                    // Simulate query execution time
                    await Task.Delay(TimeSpan.FromMilliseconds(new Random().Next(10, 150)));

                    if (appConfig.Target.Contains("simulated-failure-query")) {
                        result.Status = HealthStatus.Unhealthy;
                        result.ErrorMessage = "Simulated query execution failure.";
                    } else {
                         // Simulate successful check
                        result.Status = HealthStatus.Healthy;
                    }
                }
                // --- END SIMULATION LOGIC ---

                // In a real scenario:
                // using (var connection = new DbConnection(appConfig.Target)) // Specific provider needed
                // {
                //    await connection.OpenAsync(); // Consider CancellationToken
                //    using (var command = connection.CreateCommand())
                //    {
                //        command.CommandText = string.IsNullOrWhiteSpace(appConfig.Query) ? "SELECT 1" : appConfig.Query;
                //        command.CommandTimeout = appConfig.TimeoutSeconds; // Or a portion for the command itself
                //        await command.ExecuteNonQueryAsync(); // Or ExecuteScalarAsync, etc.
                //        result.Status = HealthStatus.Healthy;
                //    }
                // }
            }
            catch (DbException ex) // Catch specific DB exceptions
            {
                result.Status = HealthStatus.Unhealthy;
                result.ErrorMessage = $"Database operation failed: {ex.Message}";
            }
            catch (Exception ex) // Catch other exceptions like simulated ones or timeouts
            {
                result.Status = HealthStatus.Unhealthy;
                result.ErrorMessage = $"An error occurred during DB check: {ex.Message}";
            }
            finally
            {
                stopwatch.Stop();
                result.ResponseTime = stopwatch.Elapsed;
                result.LastCheckedUtc = DateTime.UtcNow;
            }

            return result;
        }
    }
}
